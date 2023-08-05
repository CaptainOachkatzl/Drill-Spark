use std::collections::HashMap;

use bevy::prelude::*;
use bevy_spicy_networking::AppNetworkServerMessage;
use drillspark_common_lib::*;
use xs_bevy_core_2d::*;

use crate::{
    buildings::building_process::handle_build_requests,
    mining::{handle_minetag_messages, update_mine_scheduler},
    networking::{handle_connection_events, ConnectionIdLookup},
    player::IdGenerator,
    resources::update_player_resource_counter,
    revealing::*,
    settings::*,
};

pub struct HostWorldPlugin;

impl Plugin for HostWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IdGenerator::new())
            .insert_resource(ConnectionIdLookup { 0: HashMap::new() })
            .add_systems(Startup, initialize_world)
            .add_systems(PreUpdate, handle_connection_events)
            .listen_for_server_message::<MineTagMessage>()
            .listen_for_server_message::<BuildRequestMessage>()
            .add_systems(
                Update,
                (
                    handle_minetag_messages,
                    update_mine_scheduler,
                    handle_build_requests,
                    update_player_resource_counter,
                    send_newly_revealed_tiles,
                    send_updated_tiles,
                ),
            );
    }
}

fn initialize_world(mut commands: Commands) {
    spawn_tiles(&mut commands);
}

fn spawn_tiles(commands: &mut Commands) {
    let mut ids = Box::new([Entity::from_raw(0); WORLD_HEIGHT * WORLD_WIDTH]);

    for coords in WORLD_SIZE.iter() {
        let tile_status = TileStatus::new(create_ore_tile(), false, None);

        let id = commands
            .spawn_empty()
            .insert(Tile)
            .insert(Position(coords))
            .insert(tile_status)
            .insert(RevealStatus { 0: TodoList::default() })
            .id();

        ids[index_translation::to_index(coords, WORLD_SIZE)] = id;
    }

    commands.insert_resource(GameGrid(Grid::new(WORLD_WIDTH, WORLD_HEIGHT, ids)))
}

fn create_ore_tile() -> TileType {
    let ore_type = get_random_ore_type();
    TileType::Block(ore_type)
}
