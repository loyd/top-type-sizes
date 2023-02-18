use std::io::BufRead;

/// Strips the `print-type-size ` prefix and ignores unprefixed lines.
pub fn read(mut rd: impl BufRead) -> eyre::Result<String> {
    let mut line = String::with_capacity(4096);
    let mut result = String::new();

    while rd.read_line(&mut line)? > 0 {
        let Some(refined_line) = line.strip_prefix("print-type-size ") else {
            continue
        };

        result.push_str(refined_line);
        line.clear();
    }

    // Handle a trailing newline.
    if result.chars().rev().next().map_or(false, |c| c == '\n') {
        result.pop();
    }

    Ok(result)
}
