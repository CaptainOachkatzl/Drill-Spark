use std::collections::HashMap;

use bevy::prelude::{Entity, Resource};
use bevy_spicy_networking::ConnectionId;

#[derive(Resource)]
pub struct ConnectionIdLookup(pub HashMap<ConnectionId, Entity>);
