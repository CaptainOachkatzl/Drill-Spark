use bevy::prelude::*;
use bevy_spicy_networking::NetworkData;
use drillspark_common_lib::{Tile, TileStatus, TileUpdateMessage};
use xs_bevy_core_2d::Grid;

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
  mut q_reveal_status: Query<(&mut RevealStatus, &mut TileStatus), With<Tile>>,
) {
  new_messages.iter().for_each(|msg| {
    if let Some(entity) = grid.get_value_by_position(msg.position) {
      let (mut reveal_status, mut tile_status) = q_reveal_status.get_mut(entity).unwrap();
      if let Some(new_tile_status) = msg.tile_status {
        (*reveal_status).0 = true;
        *tile_status = new_tile_status;
      } else {
        (*reveal_status).0 = false;
      }
    }
  });
}
