use std::{fs::File, io::Read};

use anyhow::Ok;
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, prelude::BASE64_STANDARD, Engine};

use crate::cli::base64::Base64Format;

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut buf = Vec::new();
    let mut reader = get_reader(input)?;
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

// pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
//     let decoded = URL_SAFE.decode(input)?;
//     let decoded = String::from_utf8(decoded)?;
//     println!("{}", decoded);
//     Ok(())
// }
pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    // let mut reader: Box<dyn Read> = if input == "-" {
    //     Box::new(std::io::stdin())
    // } else {
    //     Box::new(File::open(input)?)
    // };

    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // TODO: decoded data might not be string (but for this example, we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
        let input = "fixtures/b64_urlsafe.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_encode(input, format).is_ok());
    }
}
