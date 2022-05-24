use std::collections::HashMap;

use bevy::prelude::*;
use bevy_spicy_networking::AppNetworkServerMessage;
use drillspark_common_lib::*;
use xs_bevy_core_2d::*;

use crate::{
  mining::{handle_minetag_messages, update_mine_scheduler},
  networking::{handle_connection_events, ConnectionIdLookup},
  player::Ownership,
  revealing::*,
  settings::*, buildings::building_process::handle_build_requests,
};

pub struct HostWorldPlugin;

impl Plugin for HostWorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ConnectionIdLookup { 0: HashMap::new() })
      .add_startup_system(initialize_world)
      .add_system_to_stage(CoreStage::PreUpdate, handle_connection_events)
      .listen_for_server_message::<MineTagMessage>()
      .listen_for_server_message::<BuildRequestMessage>()
      .add_system(handle_minetag_messages)
      .add_system(update_mine_scheduler)
      .add_system(handle_build_requests)
      .add_system(send_newly_revealed_tiles)
      .add_system(send_updated_tiles);
  }
}

fn initialize_world(mut commands: Commands) {
  spawn_tiles(&mut commands);
}

fn spawn_tiles(commands: &mut Commands) {
  let mut ids = Box::new([Entity::from_raw(0); WORLD_HEIGHT * WORLD_WIDTH]);

  for coords in WORLD_SIZE.iter() {
    let tile_status = TileStatus::new(create_ore_tile(), false);

    let id = commands
      .spawn()
      .insert(Tile)
      .insert(coords)
      .insert(tile_status)
      .insert(RevealStatus { 0: TodoList::new() })
      .insert(Ownership { 0: None })
      .id();

    ids[index_translation::to_index(coords, WORLD_SIZE)] = id;
  }

  commands.insert_resource(Grid::new(WORLD_WIDTH, WORLD_HEIGHT, ids))
}

fn create_ore_tile() -> TileType {
  let ore_type = get_random_ore_type();
  TileType::Block(ore_type)
}
