use bevy::prelude::*;
use xs_bevy_core_2d::Grid;

#[derive(Resource, Clone, Deref)]
pub struct GameGrid(pub Grid<Entity>);
