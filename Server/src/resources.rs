use bevy::prelude::*;
use bevy_spicy_networking::NetworkServer;
use drillspark_common_lib::{ResourceMessage, ResourceStore};

use crate::player::Player;

pub fn update_player_resource_counter(
    net: Res<NetworkServer>,
    q_resource_stores: Query<(&Player, &ResourceStore), (With<Player>, Changed<ResourceStore>)>,
) {
    q_resource_stores.for_each(|(player, res_store)| {
        let msg = ResourceMessage {
            resources: res_store.clone_resources(),
        };
        if net.send_message(player.0, msg).is_err() {
            error!("could not send resource message to player with connection ID {}", player.0)
        }
    });
}
