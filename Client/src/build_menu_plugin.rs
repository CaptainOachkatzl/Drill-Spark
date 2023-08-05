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
            .add_systems(Startup, initilize_build_icons)
            .add_systems(Update, handle_resource_message)
            .add_systems(Update, render_resource_text);
    }
}

fn initilize_build_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    // menu node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(MENU_WIDTH),
                height: Val::Auto,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // border
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },

                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // fill
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(spawn_resource_text(&asset_server));
                            // insert menu icons here
                        });
                });
        });
}
