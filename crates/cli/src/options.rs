use clap::Parser;

use transpiler::TranspileOptions;

use crate::errors::CliError;

#[derive(Parser)]
pub struct CliOptions {
    #[clap(parse(from_os_str))]
    pub file_path: std::path::PathBuf,
    #[clap(short, long)]
    pub debug: bool,
}

impl TryInto<TranspileOptions> for CliOptions {
    type Error = CliError;
    fn try_into(self) -> Result<TranspileOptions, Self::Error> {
        let entry_file_path = self
            .file_path
            .to_str()
            .ok_or(CliError::InvalidEntryFilePath(
                self.file_path.to_string_lossy().clone().to_string(),
            ))?
            .to_string();

        Ok(TranspileOptions {
            entry_file_path,
            debug: self.debug,
        })
    }
}
