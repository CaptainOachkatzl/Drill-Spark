use bevy::prelude::*;
use bevy_spicy_networking::AppNetworkClientMessage;
use drillspark_common_lib::{game_component::*, *};
use xs_bevy_core_2d::{index_translation, translation, Field, Grid, Size2D};

use crate::input_handling::handle_input;
use crate::rendering::*;
use crate::revealing::RevealStatus;
use crate::{revealing::revealing, settings::*};

pub struct BoardPlugin;

#[derive(Resource, Deref)]
pub struct ScreenTranslation(pub translation::ScreenTranslation);

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        let size = Size2D {
            height: WINDOW_HEIGHT as usize,
            width: (WINDOW_WIDTH - MENU_WIDTH) as usize,
        };

        let screen_view = Field {
            size,
            offset: xs_bevy_core_2d::Position::new(MENU_WIDTH as i64, 0),
        };

        let translation = ScreenTranslation(translation::ScreenTranslation::new(screen_view, WORLD_SIZE));

        app.listen_for_client_message::<TileUpdateMessage>()
            .insert_resource(translation)
            .add_systems(Startup, initialize_world)
            .add_systems(Update, handle_input)
            .add_systems(Update, update_tile_color)
            .add_systems(Update, update_screen_translation)
            .add_systems(Update, revealing);
    }
}

pub fn initialize_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn_tiles(&mut commands, &asset_server);
}

fn spawn_tiles(commands: &mut Commands, _asset_server: &Res<AssetServer>) {
    let mut ids = Box::new([Entity::from_raw(0); WORLD_HEIGHT * WORLD_WIDTH]);

    let offset = get_tile_offset(WORLD_SIZE, TILE_SIZE, MENU_WIDTH, 0.);

    for coords in WORLD_SIZE.iter() {
        let transform = get_tile_transform(offset, coords, TILE_SIZE);

        let tile_status = TileStatus::new(TileType::Block(Ores::Stone), false, None);
        let reveal_status = RevealStatus::from(false);
        let id = commands
            .spawn_empty()
            .insert(Tile)
            .insert(Position(coords))
            .insert(tile_status)
            .insert(MineTag::from(false))
            .insert(reveal_status)
            .insert(create_tile_sprite(tile_status, reveal_status, BLOCK_SIZE, transform))
            .id();

        ids[index_translation::to_index(coords, WORLD_SIZE)] = id;
    }

    commands.insert_resource(GameGrid(Grid::new(WORLD_WIDTH, WORLD_HEIGHT, ids)));
}
