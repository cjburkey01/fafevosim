//! What is it? What isn't it? :shrug:
//! I just love my Jadie :)
//!
//! - With love and care, CJ

// my babies
mod ecs;
mod net;
mod simworld;

// ~~ Imports ~~ //
use bevy::{
    log::{Level, LogSettings},
    prelude::*,
    render::camera::ScalingMode,
};
use ecs::*;
use net::*;
use simworld::*;

/// Start le simulation
fn main() {
    App::new()
        // Enable debug logging but disable for "loud" crates
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: "wgpu=warn,bevy_ecs=info,naga=info".to_owned(),
        })
        // Background color
        .insert_resource(ClearColor(Color::BLACK))
        // Window configuration
        .insert_resource(WindowDescriptor {
            title: format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            resizable: true,
            ..default()
        })
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkEcsPlugin)
        .add_plugin(SimWorldPlugin)
        // Spawn the camera and essential scene stuff
        .add_startup_system(init_scene_system)
        // And go!
        .run();
}

/// Spawn the essentials into the scene.
fn init_scene_system(mut commands: Commands, assets: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(8.0),
            ..default()
        },
        ..default()
    });

    // Spawn a sample Smitty
    commands.spawn_bundle(SmittyBundle {
        brain: SimEntityBrain::random(),
        pos: SimEntityPosRot(default(), 0.0),
        inputs: SimEntityBrainInputs {},
        outputs: SimEntityBrainOutputs {
            move_amt: 1.0,
            rot_amt: 1.0,
        },
        traits: SimEntityTraits {
            max_move_speed: SMITTY_MAX_MOVE_SPEED / 4.0,
            max_rot_speed: SMITTY_MAX_ROT_SPEED / 4.0,
        },
        sprite: SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(SMITTY_SCALE)),
            texture: assets.load("smitty.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            ..default()
        },
    });
}
