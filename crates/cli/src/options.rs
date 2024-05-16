use clap::Parser;

use transpiler::TranspileOptions;

#[derive(Parser)]
pub struct CliOptions {
    #[clap(parse(from_os_str))]
    pub file_path: std::path::PathBuf,
    #[clap(short, long)]
    pub debug: bool,
}

impl Into<TranspileOptions> for CliOptions {
    fn into(self) -> TranspileOptions {
        TranspileOptions {
            entry_file_path: self.file_path.to_str().unwrap().to_string(),
            debug: self.debug,
        }
    }
}
