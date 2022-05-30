#![feature(let_else)]
#![feature(async_closure)]

use bevy::{app::{App, ScheduleRunnerSettings}, log::LogPlugin, prelude::*};
use bevy_spicy_networking::*;
use settings::FRAME_TIME;
use std::{time::Duration, net::SocketAddr};

mod host_world_plugin;
mod mining;
mod buildings;
mod revealing;
mod settings;
mod networking;
mod player;
mod resources;

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
    .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(FRAME_TIME)))
    .add_plugins(MinimalPlugins)
    .add_plugin(LogPlugin)
    .add_plugin(ServerPlugin)
    .add_startup_system(setup_networking)
    .add_plugin(HostWorldPlugin)
    .run();
}
