use std::sync::atomic::{AtomicU8, Ordering};

use bevy::prelude::{Component, Entity, Resource};
use bevy_spicy_networking::ConnectionId;

#[derive(Component, Clone, Copy)]
pub struct Player(pub ConnectionId);

#[derive(Component, Clone, Copy)]
pub struct PlayerId(pub u8);

#[derive(Resource)]
pub struct IdGenerator(AtomicU8);
impl IdGenerator {
    pub fn new() -> Self {
        Self { 0: AtomicU8::new(0) }
    }
    pub fn next(&self) -> u8 {
        self.0.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ownership(pub Option<Entity>);
