use bevy::prelude::*;
use drillspark_common_lib::*;

const FONT_SIZE: f32 = 20.;
const TEXT_INSPACING: f32 = 10.;

pub fn render_resource_text(windows: Res<Windows>, resource_store: Res<ResourceStore>, mut q_text_position: Query<(&mut Transform, &mut Text), With<Text>>) {
  if let Some(window) = windows.get_primary() {
    let (mut transform, mut text) = q_text_position.single_mut();
    *transform = get_text_position(window.width(), window.height());
    text.sections[0].value = get_resource_text(&resource_store);
  }
}

pub fn spawn_resource_text(commands: &mut Commands, window_width: f32, window_height: f32, asset_server: &Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Light.ttf");
  let text_style = TextStyle {
    font,
    font_size: FONT_SIZE,
    color: Color::WHITE,
  };
  let text_alignment = TextAlignment {
    vertical: VerticalAlign::Top,
    horizontal: HorizontalAlign::Left,
  };

  let transform = get_text_position(window_width, window_height);
  commands.spawn_bundle(Text2dBundle {
    text: Text::with_section(get_resource_text(&ResourceStore::new()), text_style, text_alignment),
    transform,
    ..default()
  });
}

fn get_resource_text(resource_store: &ResourceStore) -> String {
  format!(
    "iron: {}\ngold: {}",
    resource_store.get_count(Resources::Iron),
    resource_store.get_count(Resources::Gold)
  )
}

fn get_text_position(screen_width: f32, screen_height: f32) -> Transform {
  Transform::from_translation(Vec3::new(
    -screen_width / 2. + TEXT_INSPACING,
    screen_height / 2. - FONT_SIZE / 2.,
    0.,
  ))
}
