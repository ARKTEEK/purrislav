use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub enum HexColorError {
    InvalidLength,
    InvalidHex,
}

impl fmt::Display for HexColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HexColorError::InvalidLength => "Invalid hex color length.",
                HexColorError::InvalidHex => "Invalid hex color string.",
            }
        )
    }
}

pub struct ColorUtils;

impl ColorUtils {
    pub fn validate_hex_color(hex: &str) -> Result<(), HexColorError> {
        let hex_regex = Regex::new(r"^#?([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();

        if !hex_regex.is_match(hex) {
            return Err(HexColorError::InvalidHex);
        }

        if hex.len() != 7 && hex.len() != 6 {
            return Err(HexColorError::InvalidLength);
        }

        Ok(())
    }

    pub fn hex_to_colour(hex: &str) -> Result<(u8, u8, u8), HexColorError> {
        let hex = if hex.starts_with('#') { &hex[1..] } else { hex };

        ColorUtils::validate_hex_color(hex)?;

        let hex = if hex.len() == 3 {
            let expanded: String = hex.chars().flat_map(|c| vec![c, c]).collect();
            expanded
        } else {
            hex.to_string()
        };

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| HexColorError::InvalidHex)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| HexColorError::InvalidHex)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| HexColorError::InvalidHex)?;

        Ok((r, g, b))
    }
}
