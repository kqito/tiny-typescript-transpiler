use transpiler::TranspileError;

pub enum Status {
    Error,
    Warning,
    Success,
}

// signle print function that passable status 'error' or 'warning' or nothing
pub fn pretty_print(message: &str, status: Status) {
    match status {
        Status::Error => println!("\x1b[31m[ERROR]\x1b[0m {}\n", message),
        Status::Warning => println!("\x1b[33m[WARNING]\x1b[0m {}\n", message),
        Status::Success => println!("{}\n", message),
    }
}

pub fn print_transpile_error(error: &TranspileError) {
    match error {
        TranspileError::ParseError {
            source_file,
            context,
        } => {
            let lines: Vec<&str> = source_file.src.split('\n').collect();
            for error in &context.errors {
                let error_line_index = source_file
                    .src
                    .chars()
                    .take(error.pos as usize)
                    .filter(|&c| c == '\n')
                    .count();

                let error_lines = lines
                    .iter()
                    .enumerate()
                    .skip(error_line_index.saturating_sub(2))
                    .take_while(|(i, _)| *i < error_line_index + 1)
                    .collect::<Vec<_>>();

                for (i, line) in error_lines {
                    println!("\x1b[90m{:5} | \x1b[0m {}", i + 1, line);
                }

                let mut pointer = String::new();
                let pointer_pos = source_file
                    .src
                    .chars()
                    .take(error.pos as usize)
                    .collect::<Vec<char>>()
                    .into_iter()
                    .rev()
                    .take_while(|&c| c != '\n')
                    .count();

                let pointer_range = error.end - error.pos;

                for _ in 0..pointer_pos {
                    pointer.push(' ');
                }
                for _ in 0..pointer_range {
                    pointer.push('^');
                }

                println!("{}", pointer);
                pretty_print(
                    &format!("{}: {}", source_file.path, error.message),
                    Status::Error,
                );
            }
        }
        TranspileError::EmitError {
            source_file,
            context,
        } => {
            for error in &context.errors {
                pretty_print(
                    &format!("{}: {}", source_file.path, error.message),
                    Status::Error,
                );
            }
        }
        _ => pretty_print(&error.to_string(), Status::Error),
    }
}
