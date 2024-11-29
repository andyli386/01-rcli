use anyhow::Result;
use core::fmt;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use super::{verify_input_file, verify_key_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate sign")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(long, help = "Input file path, `-` for `stdin`", value_parser=verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, help = "Key path", value_parser=verify_key_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value= "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(long, help = "Input file path, `-` for `stdin`", value_parser=verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, help = "Key path", value_parser=verify_key_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value= "blake3")]
    pub format: TextSignFormat,
    #[arg(long, help = "Signature")]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, value_parser = parse_format, default_value= "blake3")]
    pub format: TextSignFormat,
    #[arg(long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Unsupported format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[cfg(test)]
mod tests {}
