use bevy::{prelude::*, window::Windows};
use bevy_spicy_networking::NetworkClient;
use drillspark_common_lib::{blueprints::*, game_component::*, *};
use xs_bevy_core_2d::{translation::ScreenTranslation, *};

use crate::buildings::allowed_to_build;

pub fn handle_input(
    input: ResMut<Input<MouseButton>>,
    screen_translation: Res<ScreenTranslation>,
    resource_store: Res<ResourceStore>,
    windows: Res<Windows>,
    net: Res<NetworkClient>,
    grid: Res<Grid<Entity>>,
    mut tiles: Query<(&Position, &mut TileStatus, Entity, &mut MineTag), With<Tile>>,
) {
    let left_click = input.just_pressed(MouseButton::Left);
    let right_click = input.just_pressed(MouseButton::Right);

    if !left_click && !right_click {
        return;
    }
    let Some(window) = windows.get_primary() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Some(selected_pos) = screen_translation.get_logical_position(cursor_pos.x as usize, cursor_pos.y as usize) else {
        return;
    };

    let selected_entity = grid
        .get_value_by_position(selected_pos)
        .expect("screen translation returned invalid logical position");

    let selected_tile_type = tiles.get(selected_entity).unwrap().1.tile_type;

    if selected_tile_type.is_tile_type_minable() {
        let (_, _, _, mut minetag) = tiles.get_mut(selected_entity).unwrap();
        let changed_minetag = MineTag::from(!minetag.0);
        let msg = MineTagMessage {
            mine_tags: vec![(selected_pos, changed_minetag)],
        };
        if net.send_message(msg).is_ok() {
            *minetag = MineTag::from(!minetag.0);
        }
    } else if selected_tile_type == TileType::Ground {
        let blueprint: Box<dyn BuildingBlueprint> = match left_click {
            true => Box::new(TowerBlueprint::new()),
            false => Box::new(DrillDepotBlueprint::new()),
        };

        let get_tile_type = |entity| {
            return &tiles.get(entity).unwrap().1.tile_type;
        };

        if !allowed_to_build(&*blueprint, selected_pos, &*grid, &*resource_store, get_tile_type) {
            return;
        }

        let msg = BuildRequestMessage {
            center: selected_pos,
            blueprint: *blueprint.get_type(),
        };
        if net.send_message(msg).is_err() {
            error!("could not send build request to server");
        }
    }
}
