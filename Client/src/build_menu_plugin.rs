use bevy::prelude::*;
use bevy_spicy_networking::AppNetworkClientMessage;
use drillspark_common_lib::{ResourceMessage, ResourceStore};

use crate::{
    rendering::{render_resource_text, spawn_resource_text},
    resources::handle_resource_message,
    settings::MENU_WIDTH,
};

pub struct BuildMenuPlugin;

impl Plugin for BuildMenuPlugin {
    fn build(&self, app: &mut App) {
        app.listen_for_client_message::<ResourceMessage>()
            .insert_resource(ResourceStore::new())
            .add_startup_system(initilize_build_icons)
            .add_system(handle_resource_message)
            .add_system(render_resource_text);
    }
}

fn initilize_build_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    // menu node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(MENU_WIDTH), Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // border
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // fill
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(spawn_resource_text(&asset_server));
                            // insert menu icons here
                        });
                });
        });
}
