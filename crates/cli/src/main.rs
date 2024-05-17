use clap::Parser;
use options::CliOptions;
use output::{pretty_print, Status};
use transpiler::{transpile, TranspileOptions};

mod errors;
mod options;
mod output;

fn main() {
    let options = match CliOptions::try_parse() {
        Ok(options) => options,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let transpile_options: TranspileOptions = match options.try_into() {
        Ok(options) => options,
        Err(err) => {
            err.pretty_print();
            std::process::exit(1);
        }
    };

    let transpile_result = transpile(transpile_options);
    match transpile_result {
        Ok(result) => {
            pretty_print(&result.emit_result, Status::Success);
        }
        Err(err) => {
            let cli_error: errors::CliError = err.into();
            cli_error.pretty_print();
        }
    }
}
