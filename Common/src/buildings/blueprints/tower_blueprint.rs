use xs_bevy_core_2d::{Grid, Position};

use crate::{resources::Transaction, *};

use super::*;

pub struct TowerBlueprint {
  tile_map: Grid<Option<TileType>>,
  cost: Transaction,
  placement_center: Position,
}

impl TowerBlueprint {
  pub fn new() -> Self {
    let cost = Transaction::new_single(Resources::Iron, 10);
    TowerBlueprint {
      tile_map: Grid::new(1, 1, Box::new([Some(TileType::Building(Buildings::Tower))])),
      cost,
      placement_center: Position::zero(),
    }
  }
}

impl BuildingBlueprint for TowerBlueprint {
  fn get_type(&self) -> &Buildings {
    const TYPE: Buildings = Buildings::Tower;
    &TYPE
  }

  fn get_tile_map(&self) -> &Grid<Option<TileType>> {
    &self.tile_map
  }

  fn get_cost(&self) -> &Transaction {
    &self.cost
  }

  fn get_placement_center(&self) -> &Position {
    &self.placement_center
  }
}
