use bevy::prelude::*;
use drillspark_common_lib::{blueprints::BuildingBlueprint, ResourceStore, TileType};
use xs_bevy_core_2d::*;

pub fn allowed_to_build<'a>(
    blueprint: &dyn BuildingBlueprint,
    place_at: Position,
    grid: &Grid<Entity>,
    resource_store: &ResourceStore,
    get_tile: impl Fn(Entity) -> &'a TileType,
) -> bool {
    let blueprint_tile_map = blueprint.get_tile_map();

    let Some(building_area) = get_building_area(blueprint, grid, place_at) else {
        return false;
    };

    if !resource_store.has_resource(blueprint.get_cost()) {
        return false;
    }

    if !can_place_blueprint(blueprint_tile_map, &building_area, get_tile) {
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
    get_tile: impl Fn(Entity) -> &'a TileType,
) -> bool {
    for blueprint_entry in blueprint_tile_map.iter_with_position() {
        if let (pos, Some(_)) = blueprint_entry {
            let tile_type = get_tile(building_area.get_value_by_position(pos).unwrap());
            if *tile_type != TileType::Ground {
                return false;
            }
        }
    }

    return true;
}
