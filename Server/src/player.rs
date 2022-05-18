use bevy::prelude::{Component, Entity};
use bevy_spicy_networking::ConnectionId;

#[derive(Component, Clone, Copy)]
pub struct Player(pub ConnectionId);

#[derive(Component)]
pub struct Ownership(pub Option<Entity>);
