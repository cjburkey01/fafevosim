mod net;

use bevy::log::{Level, LogSettings};
use bevy::prelude::*;

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
        .add_startup_system(init_scene)
        .run();
}

fn init_scene(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
