#![feature(async_closure)]

use bevy::{
    app::{App, ScheduleRunnerPlugin},
    log::LogPlugin,
    prelude::*,
};
use bevy_spicy_networking::*;
use settings::FRAME_TIME;
use std::{net::SocketAddr, time::Duration};

mod buildings;
mod host_world_plugin;
mod mining;
mod networking;
mod player;
mod resources;
mod revealing;
mod settings;

use crate::host_world_plugin::HostWorldPlugin;

fn main() {
    host_map();
}

fn setup_networking(mut net: ResMut<NetworkServer>) {
    let ip_address = "0.0.0.0".parse().expect("Could not parse ip address");

    info!("Address of the server: {}", ip_address);

    let socket_address = SocketAddr::new(ip_address, 7777);

    match net.listen(socket_address) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }

    info!("Started listening for new connections!");
}

fn host_map() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(FRAME_TIME))))
        .add_plugins(LogPlugin { ..Default::default() })
        .add_plugins(ServerPlugin)
        .add_systems(Startup, setup_networking)
        .add_plugins(HostWorldPlugin)
        .run();
}
