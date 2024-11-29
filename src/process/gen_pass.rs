use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    if length < 4 {
        return Err(anyhow::anyhow!("Password length must be greater than 4"));
    }
    let mut rng = rand::thread_rng();
    let mut password: Vec<u8> = Vec::new();
    let mut chars: Vec<u8> = Vec::new();

    // Define character sets and their inclusion conditions
    let char_sets = [
        (upper, UPPER),
        (lower, LOWER),
        (number, NUMBER),
        (symbol, SYMBOL),
    ];

    // Collect enabled character sets and add one character from each
    for &(enabled, set) in &char_sets {
        if enabled {
            chars.extend_from_slice(set);
            if let Some(&c) = set.choose(&mut rng) {
                password.push(c); // Ensure at least one character from this set
            }
        }
    }

    if chars.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one character type must be enabled"
        ));
    }

    while password.len() < length as usize {
        if let Some(&c) = chars.choose(&mut rng) {
            password.push(c);
        }
    }

    password.shuffle(&mut rng);
    let final_password =
        String::from_utf8(password).expect("Generated password contains valid UTF-8");

    println!("{}", final_password);
    let estimaet = zxcvbn(&final_password, &[]);
    eprintln!("Password strength: {}", estimaet.score());
    Ok(final_password)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_password(
        password: &str,
        length: usize,
        upper: bool,
        lower: bool,
        number: bool,
        symbol: bool,
    ) {
        assert_eq!(password.len(), length, "Password length mismatch");

        if upper {
            assert!(
                password.chars().any(|c| UPPER.contains(&(c as u8))),
                "Password missing uppercase letter"
            );
        }
        if lower {
            assert!(
                password.chars().any(|c| LOWER.contains(&(c as u8))),
                "Password missing lowercase letter"
            );
        }
        if number {
            assert!(
                password.chars().any(|c| NUMBER.contains(&(c as u8))),
                "Password missing number"
            );
        }
        if symbol {
            assert!(
                password.chars().any(|c| SYMBOL.contains(&(c as u8))),
                "Password missing symbol"
            );
        }
    }

    #[test]
    fn test_all_char_types() {
        let length = 12;
        let password = process_genpass(length, true, true, true, true).unwrap();
        validate_password(&password, length as usize, true, true, true, true);
    }

    #[test]
    fn test_only_uppercase() {
        let length = 8;
        let password = process_genpass(length, true, false, false, false).unwrap();
        validate_password(&password, length as usize, true, false, false, false);
    }

    #[test]
    fn test_only_lowercase() {
        let length = 10;
        let password = process_genpass(length, false, true, false, false).unwrap();
        validate_password(&password, length as usize, false, true, false, false);
    }

    #[test]
    fn test_uppercase_and_numbers() {
        let length = 14;
        let password = process_genpass(length, true, false, true, false).unwrap();
        validate_password(&password, length as usize, true, false, true, false);
    }

    #[test]
    fn test_lowercase_and_symbols() {
        let length = 16;
        let password = process_genpass(length, false, true, false, true).unwrap();
        validate_password(&password, length as usize, false, true, false, true);
    }

    #[test]
    fn test_no_char_types_enabled() {
        let result = process_genpass(8, false, false, false, false);
        assert!(
            result.is_err(),
            "Password generation should fail with no types enabled"
        );
    }

    #[test]
    fn test_zero_length() {
        let result = process_genpass(0, true, true, true, true);
        assert!(
            result.is_err(),
            "Password generation should fail with zero length"
        );
    }
}
