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
