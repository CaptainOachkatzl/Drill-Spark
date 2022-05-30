use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_spicy_networking::NetworkData;
use drillspark_common_lib::{game_component::*, *};
use xs_bevy_core_2d::{patterns::surrounding_pattern, *};

use super::{IsMineableParams, MiningQueue};

use crate::{
  networking::ConnectionIdLookup,
  player::{Player, PlayerId},
  revealing::{reveal_area, RevealStatus},
};

pub fn handle_minetag_messages(
  mut new_messages: EventReader<NetworkData<MineTagMessage>>,
  lookup: Res<ConnectionIdLookup>,
  grid: Res<Grid<Entity>>,
  mut q_players: Query<(&Player, &mut MiningQueue)>,
) {
  for msg in new_messages.iter() {
    let Some(&player_entity) = lookup.0.get(&msg.source()) else {
      error!("could not find player with connection ID {} in lookup table", msg.source());
      continue;
    };

    if let Ok((_, mut mining_queue)) = q_players.get_mut(player_entity) {
      for (pos, tag) in msg.mine_tags.iter() {
        if let Some(entity) = grid.get_value_by_position(*pos) {
          if tag == true {
            mining_queue.schedule_workload(entity)
          } else {
            mining_queue.remove_from_schedule(entity)
          }
        } else {
          error!("player with connection ID {} sent invalid block position", msg.source());
        }
      }
    } else {
      error!("player with connection ID {} not found", msg.source());
    }
  }
}

pub fn update_mine_scheduler(
  time: Res<Time>,
  grid: Res<Grid<Entity>>,
  mut q_player_systems: Query<(Entity, &mut MiningQueue, &mut ResourceStore, &Position, &PlayerId), With<Player>>,
  mut tiles: Query<(&mut TileStatus, &Position, &mut RevealStatus), With<Tile>>,
) {
  let mut mined_blocks = BTreeMap::new();

  q_player_systems.for_each_mut(
    |(player_entity, mut mining_queue, mut resource_store, &spawn_point, &id)| {
      let get_position = |entity| *tiles.get(entity).unwrap().1;
      let get_tile_type = |entity| tiles.get(entity).unwrap().0.tile_type;
      let is_revealed = |entity| tiles.get(entity).unwrap().2 .0.contains(&player_entity);
      let is_owned = |entity| tiles.get(entity).unwrap().0.owner == Some(id.0);
      let mineable_params: IsMineableParams = (
        &grid,
        spawn_point,
        &get_position,
        &get_tile_type,
        &is_revealed,
        &is_owned,
      );
      if let Some(finished_tile) = mining_queue.update(time.delta(), mineable_params) {
        let mut finished_tile = tiles.get_mut(finished_tile).unwrap();
        add_mined_resource(finished_tile.0.tile_type, &mut resource_store);
        *finished_tile.0 = TileStatus::new(TileType::Ground, false, Some(id.0));
        mined_blocks.insert(*finished_tile.1, player_entity);
      }

      if let Some(mined_tile) = mining_queue.get_current_mining_location() {
        let mut mined_tile = tiles.get_mut(mined_tile).unwrap();
        if !mined_tile.0.currently_mined {
          mined_tile.0.currently_mined = true;
        }
      }
    },
  );

  mined_blocks.iter().for_each(|(pos, player)| {
    let get_reveal_status = |entity| unsafe { tiles.get_unchecked(entity).unwrap().2 };
    reveal_area(*player, &*grid, &*surrounding_pattern(1), *pos, get_reveal_status);
  });
}

fn add_mined_resource(tile_type: TileType, resource_store: &mut ResourceStore) {
  match tile_type {
    TileType::Block(Ores::Iron) => {
      resource_store.add_resource(&Transaction::new_single(Resources::Iron, 10));
    }
    TileType::Block(Ores::Gold) => {
      resource_store.add_resource(&Transaction::new_single(Resources::Gold, 10));
    }
    TileType::Building(Buildings::WarpGate) => {
      resource_store.add_resource(&Transaction::new_single(Resources::Gold, 1000));
    }
    _ => {}
  }
}
