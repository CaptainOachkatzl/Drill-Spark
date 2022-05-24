use bevy::prelude::*;
use bevy_spicy_networking::AppNetworkClientMessage;
use drillspark_common_lib::{game_component::*, *};
use xs_bevy_core_2d::*;

use crate::input_handling::handle_input;
use crate::rendering::*;
use crate::resources::handle_resource_message;
use crate::revealing::RevealStatus;
use crate::{revealing::revealing, settings::*};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
  fn build(&self, app: &mut App) {
    let size = Size2D {
      height: WINDOW_HEIGHT as usize,
      width: (WINDOW_WIDTH - MENU_WIDTH) as usize,
    };

    let screen_view = Field {
      size,
      offset: Position::new(MENU_WIDTH as i64, 0),
    };

    let translation = ScreenTranslation::new(screen_view, WORLD_SIZE);

    app
      .listen_for_client_message::<TileUpdateMessage>()
      .listen_for_client_message::<ResourceMessage>()
      .insert_resource(translation)
      .insert_resource(ResourceStore::new())
      .add_startup_system(initialize_world)
      .add_system(handle_input)
      .add_system(update_tile_color)
      .add_system(update_screen_translation)
      .add_system(revealing)
      .add_system(handle_resource_message)
      .add_system(render_resource_text);
  }
}

pub fn initialize_world(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  spawn_resource_text(&mut commands, WINDOW_WIDTH, WINDOW_HEIGHT, &asset_server);
  spawn_tiles(&mut commands, &asset_server);
}

fn spawn_tiles(commands: &mut Commands, _asset_server: &Res<AssetServer>) {
  let mut ids = Box::new([Entity::from_raw(0); WORLD_HEIGHT * WORLD_WIDTH]);

  let offset = get_tile_offset(WORLD_SIZE, TILE_SIZE, MENU_WIDTH, 0.);

  for coords in WORLD_SIZE.iter() {
    let transform = get_tile_transform(offset, coords, TILE_SIZE);

    let tile_status = TileStatus::new(TileType::Block(Ores::Stone), false);
    let reveal_status = RevealStatus::from(false);
    let id = commands
      .spawn()
      .insert(Tile)
      .insert(coords)
      .insert(tile_status)
      .insert(MineTag::from(false))
      .insert(reveal_status)
      .insert_bundle(create_tile_sprite(tile_status, reveal_status, BLOCK_SIZE, transform))
      .id();

    ids[index_translation::to_index(coords, WORLD_SIZE)] = id;
  }

  commands.insert_resource(Grid::new(WORLD_WIDTH, WORLD_HEIGHT, ids))
}
