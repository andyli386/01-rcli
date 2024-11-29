pub mod base64;
pub mod csv;
pub mod genpass;

use std::path::Path;

use base64::Base64SubCommand;
use clap::Parser;
use csv::CsvOpts;
use genpass::GenpassOpts;

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenpassOpts),
    // #[command(name = "base64", about = "Encode or decode base64")]
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("not-exist"),
            Err("File does not exists".into())
        );
    }
}
