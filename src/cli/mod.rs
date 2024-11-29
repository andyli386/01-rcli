pub mod base64;
pub mod csv;
pub mod genpass;
pub mod text;

use std::path::{Path, PathBuf};

use base64::Base64SubCommand;
use clap::Parser;
use csv::CsvOpts;
use genpass::GenpassOpts;
use text::TextSubCommand;

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
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign or verify a message")]
    Text(TextSubCommand),
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exists or is not a directory")
    }
}
fn verify_file_general(filename: &str, allow_stdin: bool) -> Result<String, String> {
    if filename == "-" {
        if allow_stdin {
            Ok(filename.into())
        } else {
            Err("Standard input (`-`) is not allowed for this option".into())
        }
    } else if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

/// Specific validator for input files (allows stdin).
fn verify_input_file(filename: &str) -> Result<String, String> {
    verify_file_general(filename, true)
}

/// Specific validator for key files (disallows stdin).
fn verify_key_file(filename: &str) -> Result<String, String> {
    verify_file_general(filename, false)
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
            Err("File does not exist".into())
        );
    }
}
