use bevy::prelude::*;
use drillspark_common_lib::*;

const FONT_SIZE: f32 = 20.;
const TEXT_INSPACING: f32 = 10.;

pub fn render_resource_text(resource_store: Res<ResourceStore>, mut q_text_position: Query<&mut Text>) {
    let mut text = q_text_position.single_mut();
    text.sections[0].value = get_resource_text(&resource_store);
}

pub fn spawn_resource_text(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(TEXT_INSPACING)),
            ..default()
        },
        text: Text::with_section(
            get_resource_text(&ResourceStore::new()),
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Light.ttf"),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..default()
    }
}

fn get_resource_text(resource_store: &ResourceStore) -> String {
    format!(
        "iron: {}\ngold: {}",
        resource_store.get_count(Resources::Iron),
        resource_store.get_count(Resources::Gold)
    )
}
