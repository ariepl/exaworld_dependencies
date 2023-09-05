use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{DatabaseError, COORDS_SEPARATOR};

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: f32,
    pub z: f32,
}

impl FromStr for Coords {
    type Err = DatabaseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split_string: Vec<_> = string.split(COORDS_SEPARATOR).collect();

        if split_string.len() != 2 {
            return Err(DatabaseError::EntryCouldNotBeParsed(format!(
                "The coordinates in an entry could not be parsed. The coordinates were '{}'",
                string
            )));
        }

        if let Ok(x) = split_string[0].parse::<f32>() {
            if let Ok(z) = split_string[1].parse::<f32>() {
                return Ok(Coords { x, z });
            }
        }

        Err(DatabaseError::EntryCouldNotBeParsed(format!(
            "The coordinates in an entry could not be parsed. The coordinates were '{}'",
            string
        )))
    }
}

impl ToString for Coords {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.x, COORDS_SEPARATOR, self.z)
    }
}
