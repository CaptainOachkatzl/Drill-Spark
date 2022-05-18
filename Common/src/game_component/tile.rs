use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::{Buildings, Ores};

#[derive(Serialize, Deserialize, Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
  Ground,
  PlayerGround,
  Block(Ores),
  Building(Buildings),
}

#[derive(Component, Clone, Copy)]
pub struct Tile;

impl TileType {
  pub fn is_tile_type_minable(&self) -> bool {
    match self {
      TileType::Building(b) => *b == Buildings::WarpGate,
      TileType::Block(_) => true,
      _ => false,
    }
  }
}
