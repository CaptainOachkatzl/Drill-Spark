use xs_bevy_core_2d::*;

use crate::*;

use super::*;

pub struct WarpGateBlueprint {
  tile_map: Grid<Option<TileType>>,
  cost: Transaction,
  placement_center: Position,
}

impl WarpGateBlueprint {
  pub fn new() -> Self {
    let cost = Transaction::new_single(Resources::Iron, 10);
    let tile_map = create_tile_map();
    Grid::new(1, 1, Box::new([Some(TileType::Building(Buildings::Tower))]));

    WarpGateBlueprint {
      tile_map,
      cost,
      placement_center: Position::new(1, 1),
    }
  }
}

impl BuildingBlueprint for WarpGateBlueprint {
  fn get_type(&self) -> &Buildings {
    const TYPE: Buildings = Buildings::WarpGate;
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

pub fn create_tile_map() -> Grid<Option<TileType>> {
  let mut values = Box::new([Some(TileType::Ground); 9]);

  values[4] = Some(TileType::Building(Buildings::WarpGate));

  Grid::new(3, 3, values)
}
