use std::collections::HashMap;

use bevy::prelude::Entity;
use bevy_spicy_networking::ConnectionId;

pub struct ConnectionIdLookup(pub HashMap<ConnectionId, Entity>);
