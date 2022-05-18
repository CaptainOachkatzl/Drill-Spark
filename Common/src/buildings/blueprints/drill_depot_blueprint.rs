use xs_bevy_core_2d::{Grid, Position};

use crate::{resources::Transaction, *};

use super::*;

pub struct DrillDepotBlueprint {
  tile_map: Grid<Option<TileType>>,
  cost: Transaction,
  placement_center: Position,
}

impl DrillDepotBlueprint {
  pub fn new() -> Self {
    let mut cost = Transaction::new();
    cost.add_cost(Resources::Iron, 30);
    cost.add_cost(Resources::Gold, 10);
    DrillDepotBlueprint {
      tile_map: Grid::new(3, 3, Box::new([Some(TileType::Building(Buildings::DrillDepot)); 9])),
      cost,
      placement_center: Position::new(1, 1),
    }
  }
}

impl BuildingBlueprint for DrillDepotBlueprint {
  fn get_type(&self) -> &Buildings {
    const TYPE: Buildings = Buildings::DrillDepot;
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
