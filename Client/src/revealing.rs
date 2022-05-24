use bevy::prelude::*;
use bevy_spicy_networking::NetworkData;
use drillspark_common_lib::{Tile, TileStatus, TileUpdateMessage};
use xs_bevy_core_2d::{Grid, Position};

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct RevealStatus(pub bool);

impl From<bool> for RevealStatus {
  fn from(val: bool) -> Self {
    Self { 0: val }
  }
}

impl Into<bool> for RevealStatus {
  fn into(self) -> bool {
    self.0
  }
}

pub fn revealing(
  grid: Res<Grid<Entity>>,
  mut new_messages: EventReader<NetworkData<TileUpdateMessage>>,
  mut q_tiles: Query<(&mut RevealStatus, &mut TileStatus), With<Tile>>,
) {
  for msg in new_messages.iter() {
    for tile in msg.tiles.iter() {
      update_tile(&*grid, tile.0, tile.1, &mut q_tiles);
    }
  }
}

fn update_tile(
  grid: &Grid<Entity>,
  position: Position,
  new_tile_status: Option<TileStatus>,
  q_tiles: &mut Query<(&mut RevealStatus, &mut TileStatus), With<Tile>>,
) {
  if let Some(entity) = grid.get_value_by_position(position) {
    let (mut reveal_status, mut tile_status) = q_tiles.get_mut(entity).unwrap();
    if let Some(new_tile_status) = new_tile_status {
      (*reveal_status).0 = true;
      *tile_status = new_tile_status;
    } else {
      (*reveal_status).0 = false;
    }
  }
}
