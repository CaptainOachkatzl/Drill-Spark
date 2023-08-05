use crate::settings::*;
use bevy::{
    prelude::*,
    window::{WindowResizeConstraints, WindowResolution},
};
use bevy_spicy_networking::{ClientPlugin, NetworkClient, NetworkSettings};
use board_plugin::BoardPlugin;
use build_menu_plugin::BuildMenuPlugin;
use std::{fs, net::SocketAddr};

pub mod board_plugin;
mod build_menu_plugin;
mod buildings;
mod input_handling;
mod rendering;
mod resources;
mod revealing;
mod settings;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Drill Spark".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resize_constraints: WindowResizeConstraints {
                        min_width: WINDOW_WIDTH,
                        min_height: WINDOW_HEIGHT,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ClientPlugin,
            BuildMenuPlugin,
            BoardPlugin,
        ))
        .add_systems(Startup, connect)
        .run();
}

fn connect(mut net: ResMut<NetworkClient>) {
    let mut ip_address = "127.0.0.1".parse().unwrap();
    if let Ok(file_content) = fs::read_to_string("server.config") {
        if let Ok(file_address) = file_content.parse() {
            ip_address = file_address;
        }
    }

    info!("Address of the server: {}", ip_address);

    let socket_address = SocketAddr::new(ip_address, 7777);

    net.connect(
        socket_address,
        NetworkSettings {
            max_packet_length: 10 * 1024 * 1024,
        },
    );
}
