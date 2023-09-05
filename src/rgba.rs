use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Rgba {
    pub fn get_u8_values(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

impl ToString for Rgba {
    fn to_string(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}",
            self.red, self.green, self.blue, self.alpha,
        )
    }
}

impl FromStr for Rgba {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err("incorrect string length for parsing into a color".into());
        }

        let s_chars_vec = s.chars().collect::<Vec<char>>();
        let color_chunks = s_chars_vec.chunks(2).map(|x| x.iter().collect::<String>());

        let hex = "0123456789abcdef".to_string();

        let mut color_components: Vec<u8> = Vec::new();

        for chunk in color_chunks {
            let chunk = chunk.to_lowercase().chars().collect::<Vec<char>>();

            let Some(first) = hex.find(chunk[1]) else {
                return Err("string doesn't only contain values parsable into a hex number so it could not be parsed into a color".into());
            };

            let Some(second) = hex.find(chunk[0]) else {
                return Err("string doesn't only contain values parsable into a hex number so it could not be parsed into a color".into());
            };

            color_components.push((first + second * 16) as u8);
        }

        Ok(Self {
            red: color_components[0],
            green: color_components[1],
            blue: color_components[2],
            alpha: color_components[3],
        })
    }
}
