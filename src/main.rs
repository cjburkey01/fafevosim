// Allow component derive for the `NN` type (replace with optional component)
#![feature(trivial_bounds)]

mod ecs;
mod net;

use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};
use ecs::*;

fn main() {
    App::new()
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: "wgpu=warn,bevy_ecs=info,naga=info".to_string(),
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            resizable: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkEcsPlugin)
        .add_startup_system(init_scene)
        .run();
}

fn init_scene(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
