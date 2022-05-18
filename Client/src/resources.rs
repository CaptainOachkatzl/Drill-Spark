use bevy::prelude::*;
use bevy_spicy_networking::NetworkData;
use drillspark_common_lib::{ResourceMessage, ResourceStore};

pub fn handle_resource_message(
  mut new_messages: EventReader<NetworkData<ResourceMessage>>,
  mut resource_store: ResMut<ResourceStore>,
) {
  let new_message_count = new_messages.len();
  if new_message_count <= 0 {
    return;
  }

  for (i, msg) in new_messages.iter().enumerate() {
    // skip all messages but the last one
    if i != (new_message_count - 1) {
      continue;
    }

    resource_store.set_resources(msg.resources.clone());
  }
}
