#![allow(dead_code)]

use xs_bevy_core_2d::Size2D;

pub const FPS: f64 = 60.;
pub const FRAME_TIME: f64 = 1. / FPS;

pub const WORLD_WIDTH: usize = 80;
pub const WORLD_HEIGHT: usize = 40;
pub const WORLD_SIZE: Size2D = Size2D {
  width: WORLD_WIDTH,
  height: WORLD_HEIGHT,
};

pub const BLOCK_SIZE: f32 = 16.;
pub const BLOCK_DISTANCE: f32 = 1.;
pub const TILE_SIZE: f32 = BLOCK_SIZE + BLOCK_DISTANCE;

pub const MENU_WIDTH: f32 = 100.;

pub const WINDOW_WIDTH: f32 = WORLD_WIDTH as f32 * TILE_SIZE + MENU_WIDTH;
pub const WINDOW_HEIGHT: f32 = WORLD_HEIGHT as f32 * TILE_SIZE;
