use bevy::prelude::*;
use bevy_spicy_networking::NetworkData;
use drillspark_common_lib::{blueprints::*, ResourceStore, *};
use xs_bevy_core_2d::{patterns::surrounding_pattern, *};

use crate::{
  networking::ConnectionIdLookup,
  player::{Ownership, Player},
  revealing::{reveal_area, RevealStatus},
};

pub fn handle_build_requests(
  mut new_messages: EventReader<NetworkData<BuildRequestMessage>>,
  lookup: Res<ConnectionIdLookup>,
  grid: Res<Grid<Entity>>,
  mut q_players: Query<&mut ResourceStore, With<Player>>,
  q_tile_type: Query<(&mut TileType, &mut Ownership, &mut RevealStatus), With<Tile>>,
) {
  for msg in new_messages.iter() {
    let Some(&player_entity) = lookup.0.get(&msg.source()) else {
      error!("could not find player with connection ID {} in lookup table", msg.source());
      continue;
    };

    let Ok(mut resource_store) = q_players.get_mut(player_entity) else {
      error!("resource store for player with connection ID {} not found", msg.source());
      continue;
    };
    let blueprint: Box<dyn BuildingBlueprint> = match msg.blueprint {
      Buildings::Tower => Box::new(TowerBlueprint::new()),
      Buildings::DrillDepot => Box::new(DrillDepotBlueprint::new()),
      _ => {
        error!("blueprint of type {:?} not available to build", msg.blueprint);
        continue;
      }
    };

    let mut get_tile_type = |entity| unsafe {
      let tile = q_tile_type.get_unchecked(entity).unwrap();
      (tile.0, tile.1)
    };
    if build_from_blueprint(
      player_entity,
      &*blueprint,
      msg.center,
      &*grid,
      resource_store.as_mut(),
      &mut get_tile_type,
    ) {
      match msg.blueprint {
        Buildings::Tower => {
          let mut get_reveal_status = |entity| unsafe { q_tile_type.get_unchecked(entity).unwrap().2 };
          reveal_area(player_entity, &*grid, &*surrounding_pattern(3), msg.center, &mut get_reveal_status);
        }
        Buildings::DrillDepot => {
          let mut get_reveal_status = |entity| unsafe { q_tile_type.get_unchecked(entity).unwrap().2 };
          reveal_area(player_entity, &*grid, &*surrounding_pattern(8), msg.center, &mut get_reveal_status);
        }
        _ => {}
      }
    } else {
      error!("player with connection ID {} tried to build under illegal conditions", msg.source());
    }
  }
}

pub fn build_from_blueprint<'a>(
  owner: Entity,
  blueprint: &dyn BuildingBlueprint,
  place_at: Position,
  grid: &Grid<Entity>,
  resource_store: &mut ResourceStore,
  get_tile: &mut impl FnMut(Entity) -> (Mut<'a, TileType>, Mut<'a, Ownership>),
) -> bool {
  let blueprint_tile_map = blueprint.get_tile_map();

  let Some(building_area) = get_building_area(blueprint, grid, place_at) else {
    return false;
  };

  if !can_place_blueprint(blueprint_tile_map, &building_area, get_tile) {
    return false;
  }

  if resource_store.take_resource(blueprint.get_cost()) {
    place_building(owner, blueprint_tile_map, &building_area, get_tile);
  } else {
    return false;
  }

  return true;
}

fn get_building_area(blueprint: &dyn BuildingBlueprint, grid: &Grid<Entity>, place_at: Position) -> Option<Grid<Entity>> {
  let blueprint_center = blueprint.get_placement_center();
  let blueprint_tile_map = blueprint.get_tile_map();

  let offset = place_at - *blueprint_center;

  if offset.x < 0 || offset.y < 0 {
    return None;
  }

  grid.get_sub_grid(offset, *blueprint_tile_map.get_size())
}

fn can_place_blueprint<'a>(
  blueprint_tile_map: &Grid<Option<TileType>>,
  building_area: &Grid<Entity>,
  get_tile: &mut impl FnMut(Entity) -> (Mut<'a, TileType>, Mut<'a, Ownership>),
) -> bool {
  for blueprint_entry in blueprint_tile_map.iter_with_position() {
    if let (pos, Some(_)) = blueprint_entry {
      let tile_type = get_tile(building_area.get_value_by_position(pos).unwrap()).0;
      if *tile_type != TileType::Ground {
        return false;
      }
    }
  }

  return true;
}

pub fn place_building<'a>(
  owner: Entity,
  blueprint_tile_map: &Grid<Option<TileType>>,
  building_area: &Grid<Entity>,
  get_tile: &mut impl FnMut(Entity) -> (Mut<'a, TileType>, Mut<'a, Ownership>),
) {
  for (pos, &building_tile) in blueprint_tile_map.iter_with_position() {
    if let Some(building_tile) = building_tile {
      if let Some(entity) = building_area.get_value_by_position(pos) {
        let (mut tile_type, mut tile_owner) = get_tile(entity);
        *tile_type = building_tile;
        *tile_owner = Ownership { 0: Some(owner) };
      }
    }
  }
}

pub fn build_unchecked<'a>(
  owner: Entity,
  blueprint: &dyn BuildingBlueprint,
  grid: &Grid<Entity>,
  place_at: Position,
  get_tile: &mut impl FnMut(Entity) -> (Mut<'a, TileType>, Mut<'a, Ownership>),
) -> bool {
  let blueprint_tile_map = blueprint.get_tile_map();

  let Some(building_area) = get_building_area(blueprint, grid, place_at) else {
    return false;
  };

  place_building(owner, blueprint_tile_map, &building_area, get_tile);
  return true;
}
