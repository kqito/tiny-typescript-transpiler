use clap::Parser;
use errors::CliError;
use options::CliOptions;
use output::{pretty_print, print_transpile_error, Status};
use transpiler::transpile;

mod errors;
mod options;
mod output;

fn main() {
    let options = match CliOptions::try_parse() {
        Ok(options) => options,
        Err(err) => {
            pretty_print(&CliError::MissingEntryFile.to_string(), Status::Error);
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let transpile_result = transpile(options.into());
    match transpile_result {
        Ok(result) => {
            pretty_print(&result.emit_result, Status::Success);
        }
        Err(error) => {
            print_transpile_error(&error);
            std::process::exit(1);
        }
    }
}
