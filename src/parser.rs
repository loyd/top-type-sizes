use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, digit1, newline},
    combinator::{map, map_res, opt},
    error::{Error, ErrorKind},
    multi::{many0, many0_count, many1},
    sequence::{delimited, preceded, terminated, tuple},
    Err, Finish, IResult,
};

use crate::schema::*;

type Level = u32;

// 1 level = 4 spaces
// Example: "\n    "
fn indent(level: Level) -> impl FnMut(&str) -> IResult<&str, ()> {
    move |input| {
        let (input, actual_level) = preceded(newline, many0_count(tag("    ")))(input)?;

        // TODO: more idiomatic way?
        if actual_level == level as usize {
            Ok((input, ()))
        } else {
            Err(Err::Error(Error::new(input, ErrorKind::Tag))) // TODO: kind?
        }
    }
}

// Example: "`current_thread::Shared`"
fn name(input: &str) -> IResult<&str, &str> {
    delimited(char('`'), is_not("`"), char('`'))(input)
}

// Example: "152 bytes"
fn bytes(input: &str) -> IResult<&str, usize> {
    terminated(map_res(digit1, |s: &str| s.parse()), tag(" bytes"))(input)
}

// Example: "alignment: 152 bytes"
fn alignment(input: &str) -> IResult<&str, usize> {
    preceded(tag("alignment: "), bytes)(input)
}

// Example: "type: {async fn body of fn}"
fn local_type(input: &str) -> IResult<&str, &str> {
    preceded(tag("type: "), is_not("\n"))(input)
}

// Example: "offset: 0 bytes"
fn offset(input: &str) -> IResult<&str, usize> {
    preceded(tag("offset: "), bytes)(input)
}

// Examples:
// * field `.file`: 16 bytes
// * field `.0`: 24 bytes, alignment: 8 bytes
// * field `.buf`: 16 bytes, offset: 0 bytes, alignment: 8 bytes
fn field(input: &str) -> IResult<&str, Field> {
    let (input, kind) = alt((
        map(tag("field "), |_| FieldKind::AdtField),
        map(tag("upvar "), |_| FieldKind::Upvar),
        map(tag("local "), |_| FieldKind::GeneratorLocal),
    ))(input)?;

    let (input, (name, _, size)) = tuple((name, tag(": "), bytes))(input)?;
    let (input, offset) = opt(preceded(tag(", "), offset))(input)?;
    let (input, align) = opt(preceded(tag(", "), alignment))(input)?;
    let (input, local_type) = opt(preceded(tag(", "), local_type))(input)?;

    let field = Field {
        kind,
        // Remove useless leading `.`.
        name: name.trim_start_matches('.').into(),
        size,
        align,
        offset,
        local_type: local_type.map(|s| s.to_string()),
    };

    Ok((input, field))
}

// Example: "padding: 16 bytes"
fn padding(input: &str) -> IResult<&str, usize> {
    preceded(tag("padding: "), bytes)(input)
}

fn field_or_padding<'a>(level: Level) -> impl FnMut(&'a str) -> IResult<&str, FieldOrPadding> {
    preceded(
        indent(level),
        alt((
            map(field, FieldOrPadding::Field),
            map(padding, FieldOrPadding::Padding),
        )),
    )
}

// Example:
//     field `.file`: 16 bytes
//     field `.line`: 4 bytes
fn struct_type(input: &str) -> IResult<&str, StructType> {
    map(many1(field_or_padding(1)), |items| StructType { items })(input)
}

// Example:
//     variant `Ok`: 40 bytes
//         padding: 16 bytes
//         field `.0`: 24 bytes, alignment: 8 bytes
fn enum_variant(input: &str) -> IResult<&str, EnumVariant> {
    let (input, _) = indent(1)(input)?;
    let (input, (_, name, _, size)) = tuple((tag("variant "), name, tag(": "), bytes))(input)?;
    let (input, items) = many0(field_or_padding(2))(input)?;

    let enum_variant = EnumVariant {
        name: name.into(),
        size,
        items,
    };

    Ok((input, enum_variant))
}

// Example:
//     discriminant:  8 bytes
//     variant `Ok`: 40 bytes
//         padding: 16 bytes
//         field `.0`: 24 bytes, alignment: 8 bytes
//     variant `Err`: 40 bytes
//         field `.0`: 40 bytes
fn enum_type(input: &str) -> IResult<&str, EnumType> {
    let (input, discriminant_size) = opt(map(
        tuple((indent(1), tag("discriminant: "), bytes)),
        |(_, _, size)| size,
    ))(input)?;

    let (input, variants) = many0(enum_variant)(input)?;

    let enum_type = EnumType {
        discriminant_size,
        variants,
    };

    Ok((input, enum_type))
}

// Example:
// type: `idle::Idle`: 48 bytes, alignment: 8 bytes
//     variant `Ok`: 40 bytes
//         padding: 16 bytes
//         field `.0`: 24 bytes, alignment: 8 bytes
fn type_(input: &str) -> IResult<&str, Type> {
    let (input, (_, name, _, size, _, align)) =
        tuple((tag("type: "), name, tag(": "), bytes, tag(", "), alignment))(input)?;

    let (input, kind) = alt((
        map(struct_type, TypeKind::Struct),
        map(enum_type, TypeKind::Enum),
    ))(input)?;

    let (input, end_padding) =
        opt(preceded(indent(1), preceded(tag("end padding: "), bytes)))(input)?;

    let type_ = Type {
        name: name.into(),
        size,
        align,
        kind,
        end_padding,
    };

    Ok((input, type_))
}

fn types(input: &str) -> IResult<&str, Vec<Type>> {
    many0(preceded(opt(newline), type_))(input)
}

/// Parses refined (without the prefix) input.
pub fn parse(input: &str) -> eyre::Result<Vec<Type>> {
    // TODO: check recovery and failures.
    let (rest, types) = types(input)
        .finish()
        .map_err(|err| eyre::eyre!(err.to_string()))?;

    if !rest.is_empty() {
        let pos = input.len() - rest.len();
        let (error_line_no, context) = format_context(input, pos);
        return Err(eyre::eyre!(
            concat!(
                "cannot parse at line {}:\n{}\n\n",
                "Make sure the build directory is clean and that the -j1 flag is passed to the compiler.\n",
                "Run `cargo clean && RUSTFLAGS=-Zprint-type-sizes cargo +nightly build -j1 > type-sizes.txt`.\n",
                "If the issue persists, please file an issue on GitHub."
            ),
            error_line_no,
            context
        ));
    }

    Ok(types)
}

fn format_context(input: &str, pos: usize) -> (usize, String) {
    const CONTEXT_LINES: usize = 10;

    let error_line_no = input[..pos + 1].matches('\n').count();
    let start_no = error_line_no.saturating_sub(CONTEXT_LINES);
    let end_no = error_line_no + CONTEXT_LINES;
    let line_no_width = (end_no + 1).ilog10() as usize + 1;

    let context = input
        .lines()
        .skip(start_no)
        .take(end_no - start_no + 1)
        .zip(start_no..)
        .map(|(line, line_no)| {
            format!(
                "{prefix}{line_no:>line_no_width$} │ {line}",
                prefix = if line_no == error_line_no {
                    ">>> "
                } else {
                    "    "
                },
                line_no = line_no + 1,
                line = line,
                line_no_width = line_no_width
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    (error_line_no + 1, context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent() {
        assert_eq!(indent(1)("\n    "), Ok(("", ())));
        assert_ne!(indent(1)("\n        "), Ok(("", ())));
        assert_eq!(indent(2)("\n        "), Ok(("", ())));
        assert_ne!(indent(2)("\n    "), Ok(("", ())));
    }
}
