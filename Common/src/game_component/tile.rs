use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::{Buildings, Ores};

#[derive(Component, Clone, Copy)]
pub struct Tile;

#[derive(Serialize, Deserialize, Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
  Ground,
  PlayerGround,
  Block(Ores),
  Building(Buildings),
}

impl TileType {
  pub fn is_tile_type_minable(&self) -> bool {
    match self {
      TileType::Building(b) => *b == Buildings::WarpGate,
      TileType::Block(_) => true,
      _ => false,
    }
  }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileStatus {
  pub tile_type: TileType,
  pub currently_mined: bool,
}

impl TileStatus {
  pub fn new(tile_type: TileType, currently_mined: bool) -> Self {
    Self {
      tile_type,
      currently_mined,
    }
  }
}
