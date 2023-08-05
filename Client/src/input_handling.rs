use bevy::prelude::*;
use bevy_spicy_networking::NetworkClient;
use drillspark_common_lib::{blueprints::*, game_component::*, *};

use crate::{board_plugin::ScreenTranslation, buildings::allowed_to_build};

pub fn handle_input(
    input: ResMut<Input<MouseButton>>,
    screen_translation: Res<ScreenTranslation>,
    resource_store: Res<ResourceStore>,
    windows: Query<&Window>,
    net: Res<NetworkClient>,
    grid: Res<GameGrid>,
    mut tiles: Query<(&Position, &mut TileStatus, Entity, &mut MineTag), With<Tile>>,
) {
    let left_click = input.just_pressed(MouseButton::Left);
    let right_click = input.just_pressed(MouseButton::Right);

    if !left_click && !right_click {
        return;
    }
    let window = windows.single();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    println!("cursor x: {} - cursor y: {}", cursor_pos.x, cursor_pos.y);
    let Some(selected_pos) = screen_translation
        .0
        .get_logical_position(cursor_pos.x as usize, cursor_pos.y as usize)
    else {
        return;
    };

    println!("position x: {} - position y: {}", selected_pos.x, selected_pos.y);
    let selected_entity = grid
        .0
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
