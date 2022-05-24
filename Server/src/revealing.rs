use bevy::prelude::*;
use bevy_spicy_networking::{NetworkServer, ConnectionId};
use drillspark_common_lib::{game_component::*, TileUpdateMessage};
use xs_bevy_core_2d::{patterns::*, Grid, Position, TodoList};

use crate::{player::Player, mining::MiningQueue};

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
  mut q_tiles: Query<(Entity, &Position, &TileStatus, &mut RevealStatus), (With<Tile>, Changed<RevealStatus>)>,
  q_players: Query<(&Player, &MiningQueue)>
) {
  q_tiles.for_each_mut(|mut tile| {
    for player in tile.3.0.get_new().iter() {
      let msg = TileUpdateMessage { position: *tile.1, tile_status: Some(*tile.2) };
      let Ok((player, _)) = q_players.get(*player) else {
        error!("could not get connection ID for player");
        return;
      };
      match net.send_message(player.0, msg) {
        Ok(_) => info!("sent tile revealed message"),
        Err(_) => error!("could not send reveal message"),
      }
    }
  });
}

pub fn send_updated_tiles(
  net: Res<NetworkServer>,
  q_tiles: Query<(Entity, &Position, &TileStatus, &RevealStatus), (With<Tile>, Changed<TileStatus>)>,
  q_players: Query<(&Player, &MiningQueue)>
) {
  q_tiles.for_each(|tile| {
    for &player_entity in tile.3.0.get_all().iter() {

      let Ok((player, _)) = q_players.get(player_entity) else {
        error!("could not get connection ID for player");
        return;
      };

      send_tile_update_message(&*net, player.0, *tile.1, Some(*tile.2));
    
    }
  });
}

pub fn send_tile_update_message(net: &NetworkServer, conn_id: ConnectionId, position: Position, tile_status: Option<TileStatus>) {
  let msg = TileUpdateMessage { position, tile_status };
  match net.send_message(conn_id, msg) {
    Ok(_) => info!("sent tile update message"),
    Err(_) => error!("could not send reveal message"),
  }
}
