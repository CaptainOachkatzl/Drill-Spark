use bevy::prelude::*;
use bevy_spicy_networking::*;
use drillspark_common_lib::{
    blueprints::{BuildingBlueprint, WarpGateBlueprint},
    ResourceStore, *,
};
use nameof::name_of;
use rand::random;
use xs_bevy_core_2d::{patterns::surrounding_pattern, Size2D};

use crate::{
    buildings::building_process::build_unchecked,
    mining::MiningQueue,
    player::{IdGenerator, Player, PlayerId},
    revealing::{reveal_area, RevealStatus},
    settings::WORLD_SIZE,
};

use super::ConnectionIdLookup;

pub fn handle_connection_events(
    mut commands: Commands,
    id_generator: Res<IdGenerator>,
    mut lookup: ResMut<ConnectionIdLookup>,
    mut network_events: EventReader<ServerNetworkEvent>,
    grid: Res<GameGrid>,
    q_tiles: Query<&mut TileStatus, With<Tile>>,
    reveal_query: Query<&mut RevealStatus, With<Tile>>,
) {
    for event in network_events.iter() {
        match event {
            ServerNetworkEvent::Connected(conn_id) => {
                info!("New player connected: {}", conn_id);

                let player_id = PlayerId(id_generator.next());

                let spawn_blueprint = WarpGateBlueprint::new();
                let spawn_point = randomize_spawn_center(&spawn_blueprint, WORLD_SIZE);
                let player_entity = setup_player(&mut commands, player_id, conn_id, spawn_point);

                lookup.0.insert(*conn_id, player_entity);

                let mut get_tile_type = |entity| unsafe { q_tiles.get_unchecked(entity).unwrap() };
                create_player_spawn(player_id, &spawn_blueprint, spawn_point, &*grid, &mut get_tile_type);

                let mut get_reveal_status = |entity| unsafe { reveal_query.get_unchecked(entity).unwrap() };
                reveal_area(player_entity, &*grid, &*surrounding_pattern(2), spawn_point, &mut get_reveal_status);
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

fn setup_player<'a>(
    commands: &mut Commands,
    player_id: PlayerId,
    conn_id: &ConnectionId,
    spawn_point: xs_bevy_core_2d::Position,
) -> Entity {
    commands
        .spawn_empty()
        .insert(Player(*conn_id))
        .insert(player_id)
        .insert(MiningQueue::new())
        .insert(ResourceStore::new())
        .insert(Position(spawn_point))
        .id()
}

fn create_player_spawn<'a>(
    id: PlayerId,
    spawn_blueprint: &dyn BuildingBlueprint,
    spawn_point: xs_bevy_core_2d::Position,
    grid: &GameGrid,
    get_tile: &mut impl FnMut(Entity) -> Mut<'a, TileStatus>,
) {
    build_unchecked(id, spawn_blueprint, grid, spawn_point, get_tile);
}

fn randomize_spawn_center(blueprint: &dyn BuildingBlueprint, world_size: Size2D) -> xs_bevy_core_2d::Position {
    let x: usize = random::<usize>() % (world_size.width - blueprint.get_tile_map().get_width());
    let y: usize = random::<usize>() % (world_size.height - blueprint.get_tile_map().get_height());
    xs_bevy_core_2d::Position::from((x, y)) + *blueprint.get_placement_center()
}
