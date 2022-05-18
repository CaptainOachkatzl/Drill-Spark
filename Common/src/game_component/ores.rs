use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum Ores {
  Stone,
  Iron,
  Gold,
}

pub fn get_random_ore_type() -> Ores {
  match random::<u32>() % 2000 {
    1..=11 => Ores::Gold,
    12..=62 => Ores::Iron,
    _ => Ores::Stone,
  }
}

pub fn get_string(ore_type: Ores) -> &'static str {
  match ore_type {
    Ores::Stone => "stone",
    Ores::Iron => "iron",
    Ores::Gold => "gold",
  }
}
