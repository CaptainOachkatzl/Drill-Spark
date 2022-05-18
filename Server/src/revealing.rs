use bevy::prelude::*;
use bevy_spicy_networking::NetworkServer;
use drillspark_common_lib::{game_component::*, RevealMessage};
use xs_bevy_core_2d::{patterns::*, Grid, Position, TodoList};

use crate::player::Player;

#[derive(Component)]
pub struct RevealStatus(pub TodoList<Entity>);

pub fn reveal_area<'a>(
  player: Entity,
  grid: &Grid<Entity>,
  pattern: &dyn PatternPositions,
  center: Position,
  mut get_reveal_status: impl FnMut(Entity) -> Mut<'a, RevealStatus>,
) {
  let mut reveal_pattern = Vec::from(pattern.get_pattern_positions(center));
  reveal_pattern.push(center);
  reveal_pattern.iter().for_each(|&pos| {
    if let Some(entity) = grid.get_value_by_position(pos) {
      get_reveal_status(entity).0.push(player);
    }
  });
}

pub fn send_newly_revealed_tiles(
  net: Res<NetworkServer>,
  mut q_tiles: Query<(&Position, &TileType, &mut RevealStatus), (With<Tile>, Changed<RevealStatus>)>,
  q_players: Query<&Player>
) {
  q_tiles.for_each_mut(|mut tile| {
    for player in tile.2.0.get_new().iter() {
      let msg = RevealMessage { position: *tile.0, tile_type: Some(*tile.1) };
      let Ok(player_id) = q_players.get(*player) else {
        error!("could not get connection ID for player");
        return;
      };
      match net.send_message(player_id.0, msg) {
        Ok(_) => info!("sent tile revealed message"),
        Err(_) => error!("could not send reveal message"),
      }
    }
  });
}

pub fn send_updated_tiles(
  net: Res<NetworkServer>,
  q_tiles: Query<(&Position, &TileType, &RevealStatus), (With<Tile>, Changed<TileType>)>,
  q_players: Query<&Player>
) {
  q_tiles.for_each(|tile| {
    for &player_entity in tile.2.0.get_all().iter() {
      let msg = RevealMessage { position: *tile.0, tile_type: Some(*tile.1) };
      let Ok(player_id) = q_players.get(player_entity) else {
        error!("could not get connection ID for player");
        return;
      };
      match net.send_message(player_id.0, msg) {
        Ok(_) => info!("sent tile update message"),
        Err(_) => error!("could not send reveal message"),
      }
    }
  });
}
