use bevy::prelude::*;
use bevy_spicy_networking::*;
use drillspark_common_lib::{
  blueprints::{BuildingBlueprint, WarpGateBlueprint},
  ResourceStore, *,
};
use nameof::name_of;
use rand::random;
use xs_bevy_core_2d::{patterns::surrounding_pattern, *};

use crate::{
  buildings::building_process::build_unchecked,
  mining::MiningQueue,
  player::{Ownership, Player},
  revealing::{reveal_area, RevealStatus},
  settings::WORLD_SIZE,
};

use super::ConnectionIdLookup;

pub fn handle_connection_events(
  mut commands: Commands,
  mut lookup: ResMut<ConnectionIdLookup>,
  mut network_events: EventReader<ServerNetworkEvent>,
  grid: Res<Grid<Entity>>,
  tile_type_query: Query<(&mut TileType, &mut Ownership), With<Tile>>,
  reveal_query: Query<&mut RevealStatus, With<Tile>>,
) {
  for event in network_events.iter() {
    match event {
      ServerNetworkEvent::Connected(conn_id) => {
        info!("New player connected: {}", conn_id);

        let spawn_blueprint = WarpGateBlueprint::new();
        let spawn_point = randomize_spawn_center(&spawn_blueprint, WORLD_SIZE);
        let player_entity = setup_player(&mut commands, conn_id, spawn_point);

        lookup.0.insert(*conn_id, player_entity);

        let mut get_tile_type = |entity| unsafe { tile_type_query.get_unchecked(entity).unwrap() };
        create_player_spawn(player_entity, &spawn_blueprint, spawn_point, &*grid, &mut get_tile_type);

        let mut get_reveal_status = |entity| unsafe { reveal_query.get_unchecked(entity).unwrap() };
        reveal_area(
          player_entity,
          &*grid,
          &*surrounding_pattern(2),
          spawn_point,
          &mut get_reveal_status,
        );
      }
      ServerNetworkEvent::Disconnected(conn_id) => {
        info!("Player disconnected: {}", conn_id);
      }
      ServerNetworkEvent::Error(error) => {
        error!("Network error: {}", name_of!(error));
      }
    }
  }
}

fn setup_player<'a>(commands: &mut Commands, conn_id: &ConnectionId, spawn_point: Position) -> Entity {
  commands
    .spawn()
    .insert(Player(*conn_id))
    .insert(MiningQueue::new())
    .insert(ResourceStore::new())
    .insert(spawn_point)
    .id()
}

fn create_player_spawn<'a>(
  player: Entity,
  spawn_blueprint: &dyn BuildingBlueprint,
  spawn_point: Position,
  grid: &Grid<Entity>,
  get_tile_type: &mut impl FnMut(Entity) -> (Mut<'a, TileType>, Mut<'a, Ownership>),
) {
  build_unchecked(player, spawn_blueprint, grid, spawn_point, get_tile_type);
}

fn randomize_spawn_center(blueprint: &dyn BuildingBlueprint, world_size: Size2D) -> Position {
  let x: usize = random::<usize>() % (world_size.width - blueprint.get_tile_map().get_width());
  let y: usize = random::<usize>() % (world_size.height - blueprint.get_tile_map().get_height());
  Position::from((x, y)) + *blueprint.get_placement_center()
}