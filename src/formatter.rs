use std::fmt::Write;

use crate::{options::Options, schema::*};

// TODO: colors if TTY

struct Formatter {
    o: String,
}

impl Formatter {
    fn format_field_or_padding(&mut self, field: &FieldOrPadding, indent: &str) {
        match field {
            FieldOrPadding::Field(field) => {
                let _ = write!(self.o, "{indent}{:>7} {}", field.size, field.name);

                if field.kind == FieldKind::Upvar {
                    let _ = write!(self.o, " (upvar)");
                }

                if let Some(align) = field.align {
                    let _ = write!(self.o, " align={align}");
                }
                if let Some(offset) = field.offset {
                    let _ = write!(self.o, " offset={offset}");
                }
                if let Some(t) = &field.local_type {
                    let _ = write!(self.o, " type={t}");
                }

                let _ = writeln!(self.o);
            }
            FieldOrPadding::Padding(padding) => {
                let _ = writeln!(self.o, "{indent}{padding:>7} <padding>");
            }
        }
    }

    fn format_struct(&mut self, struct_: &StructType) {
        for item in &struct_.items {
            self.format_field_or_padding(item, "");
        }
    }

    fn format_enum(&mut self, enum_: &EnumType) {
        if let Some(size) = enum_.discriminant_size {
            let _ = writeln!(self.o, "{size:>7} <discriminant>");
        }

        for variant in &enum_.variants {
            let _ = writeln!(self.o, "{:>7} variant {}", variant.size, variant.name);

            if is_wrapping_variant(variant) {
                continue;
            }

            for item in &variant.items {
                self.format_field_or_padding(item, "    ");
            }
        }
    }

    fn format_types(&mut self, types: &[Type]) {
        for type_ in types {
            let _ = writeln!(
                self.o,
                "{} {} align={}",
                type_.size, type_.name, type_.align
            );

            match &type_.kind {
                TypeKind::Struct(s) => self.format_struct(s),
                TypeKind::Enum(e) => self.format_enum(e),
            }

            if let Some(padding) = type_.end_padding {
                let _ = writeln!(self.o, "{padding:>7} <end padding>");
            }

            let _ = writeln!(self.o);
        }
    }

    fn finalize(mut self) -> String {
        // Remove extra trailing `\n`.
        self.o.pop();
        self.o.pop(); // TODO: why this one?
        self.o
    }
}

fn is_wrapping_variant(variant: &EnumVariant) -> bool {
    variant.items.len() == 1
        && matches!(&variant.items[0], FieldOrPadding::Field(f) if f.name == "0")
}

pub fn format(mut types: Vec<Type>, options: &Options) -> String {
    if types.is_empty() {
        return "no types found".into();
    }

    if options.reverse {
        types.reverse();
    }

    let mut formatter = Formatter {
        o: String::with_capacity(100 * 1024),
    };

    formatter.format_types(&types);
    formatter.finalize()
}
