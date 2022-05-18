use xs_bevy_core_2d::{Grid, Position};

use crate::{resources::Transaction, TileType, Buildings};

pub trait BuildingBlueprint {
  fn get_type(&self) -> &Buildings;
  fn get_tile_map(&self) -> &Grid<Option<TileType>>;
  fn get_cost(&self) -> &Transaction;
  fn get_placement_center(&self) -> &Position;
}
