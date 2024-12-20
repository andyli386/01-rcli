use clap::Parser;
use core::fmt;
use std::str::FromStr;

use super::verify_input_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path", value_parser=verify_input_file)]
    pub input: String,

    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,

    #[arg(short, long, help = "Output file format", value_parser= parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, help = "Delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, help = "CSV has Header or not", default_value_t = true)]
    pub header: bool,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

//
// impl TryFrom<&str> for OutputFormat {
//     type Error = anyhow::Error;
//
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value.to_lowercase().as_str() {
//             "json" => Ok(OutputFormat::Json),
//             "yaml" => Ok(OutputFormat::Yaml),
//             v => anyhow::bail!("Unsupported format: {}", v),
//         }
//     }
// }

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
