use std::collections::HashMap;

use bevy::prelude::*;
use bevy_spicy_networking::{ConnectionId, NetworkServer};
use drillspark_common_lib::{game_component::*, TileUpdateMessage};
use xs_bevy_core_2d::{patterns::*, Grid, Position, TodoList};

use crate::{mining::MiningQueue, player::Player};

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
  q_players: Query<(&Player, &MiningQueue)>,
) {
  let mut tile_updates = HashMap::new();
  q_tiles.for_each_mut(|mut tile| {
    tile.3 .0.get_new().iter().for_each(|&player| {
      collect_updated_tiles(&mut tile_updates, &q_players, player, (*tile.1, Some(*tile.2)));
    });
  });

  send_batched_tile_updates(&*net, tile_updates);
}

pub fn send_updated_tiles(
  net: Res<NetworkServer>,
  q_tiles: Query<(Entity, &Position, &TileStatus, &RevealStatus), (With<Tile>, Changed<TileStatus>)>,
  q_players: Query<(&Player, &MiningQueue)>,
) {
  let mut tile_updates = HashMap::new();
  q_tiles.for_each(|tile| {
    tile.3 .0.get_all().iter().for_each(|&player| {
      collect_updated_tiles(&mut tile_updates, &q_players, player, (*tile.1, Some(*tile.2)));
    });
  });

  send_batched_tile_updates(&*net, tile_updates);
}

fn collect_updated_tiles(
  player_updates: &mut HashMap<ConnectionId, Vec<(Position, Option<TileStatus>)>>,
  q_players: &Query<(&Player, &MiningQueue)>,
  player: Entity,
  tile: (Position, Option<TileStatus>),
) {
  let Ok((player, _)) = q_players.get(player) else {
    error!("could not get connection ID for player");
    return;
  };

  player_updates.entry(player.0).or_insert(vec![]).push(tile);
}

fn send_batched_tile_updates(
  net: &NetworkServer,
  tile_updates: HashMap<ConnectionId, Vec<(Position, Option<TileStatus>)>>,
) {
  for (conn_id, updates) in tile_updates.into_iter() {
    send_tile_update_message(&*net, conn_id, TileUpdateMessage { tiles: updates });
  }
}

fn send_tile_update_message(net: &NetworkServer, conn_id: ConnectionId, msg: TileUpdateMessage) {
  match net.send_message(conn_id, msg) {
    Ok(_) => info!("sent tile update message"),
    Err(_) => error!("could not tile update message"),
  }
}
