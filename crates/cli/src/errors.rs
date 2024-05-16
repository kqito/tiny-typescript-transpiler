use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Missing entry file")]
    MissingEntryFile,
}
