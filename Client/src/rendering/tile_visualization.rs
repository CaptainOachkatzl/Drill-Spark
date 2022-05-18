use bevy::prelude::*;
use drillspark_common_lib::game_component::*;
use xs_bevy_core_2d::*;

use crate::revealing::RevealStatus;

const COLOR_MINED: Color = Color::CYAN;
const COLOR_MINE_TAGGED: Color = Color::rgb(0., 0.8, 0.8);

pub fn update_tile_color(
  mut blocks: Query<
    (&mut Sprite, &TileType, &MineTag, &RevealStatus),
    (With<Tile>, Or<(Changed<TileType>, Changed<MineTag>, Changed<RevealStatus>)>),
  >,
) {
  blocks.for_each_mut(|(mut sprite, &tile_type, &mine_tag, reveal_status)| {
     let mined = false;
     sprite.color = get_tile_color(mined, tile_type, mine_tag, *reveal_status);
  });
}

pub fn update_screen_translation(windows: Res<Windows>, mut screen_translation: ResMut<ScreenTranslation>) {
  if let Some(window) = windows.get_primary() {
    let size = screen_translation.screen_view.size;

    if (window.width() as usize) < size.width + 100 || (window.height() as usize) < size.height {
      return;
    }

    let offset_x = (window.width() as usize - 100 - size.width) / 2 + 100;
    let offset_y = (window.height() as usize - size.height) / 2;

    screen_translation.screen_view.offset = Position::from((offset_x, offset_y));
  }
}

pub fn create_tile_sprite(tile_type: TileType, reveal_status: RevealStatus, tile_size: f32, transform: Transform) -> SpriteBundle {
  SpriteBundle {
    sprite: Sprite {
      color: get_tile_color(false, tile_type, MineTag::from(false), reveal_status),
      custom_size: Some(Vec2::splat(tile_size)),
      ..Default::default()
    },
    transform,
    ..Default::default()
  }
}

pub fn get_tile_offset(world_size: Size2D, tile_size: f32, offset_x: f32, offset_y: f32) -> Vec3 {
  Vec3::new(
    (world_size.width as f32 / 2. * -tile_size) + (tile_size / 2.) + (offset_x / 2.),
    (world_size.height as f32 / 2. * -tile_size) + (tile_size / 2.) + (offset_y / 2.),
    0.0,
  )
}

pub fn get_tile_transform(offset: Vec3, position: Position, tile_size: f32) -> Transform {
  Transform::from_translation(Vec3::new(
    tile_size * position.x as f32 + offset.x,
    tile_size * position.y as f32 + offset.y,
    10.,
  ))
}

fn get_tile_color(mined: bool, tile_type: TileType, mine_tag: MineTag, reveal_status: RevealStatus) -> Color {
  if !reveal_status.0 {
    if mine_tag.into() {
      return COLOR_MINE_TAGGED;
    } else {
      return Color::DARK_GRAY;
    }
  }

  if mined && mine_tag.into() {
    return COLOR_MINED;
  }

  #[allow(unreachable_patterns)]
  match tile_type {
    TileType::Block(ore_type) => { 
      if mine_tag.into() {
        COLOR_MINE_TAGGED
      } else {
        get_color_from_ore_type(ore_type)
      }
    },
    TileType::Ground => Color::ANTIQUE_WHITE,
    TileType::PlayerGround => Color::RED,
    TileType::Building(building_type) => match building_type {
      Buildings::WarpGate => Color::PURPLE,
      Buildings::Tower => Color::GREEN,
      Buildings::DrillDepot => Color::ORANGE,
    },
    _ => Color::PINK, // debug
  }
}

#[allow(unreachable_patterns)]
fn get_color_from_ore_type(ore_type: Ores) -> Color {
  return match ore_type {
    Ores::Stone => Color::rgba_u8(0x51, 0x36, 0x1a, 0xff),
    Ores::Iron => Color::SILVER,
    Ores::Gold => Color::GOLD,
    _ => Color::PINK, //debug
  };
}
