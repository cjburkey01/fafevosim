//! What is it? What isn't it? :shrug:
//! I just love my Jadie :)
//!
//! - With love and care, CJ

// my babies
mod ecs;
mod gui;
mod net;
mod simworld;

// ~~ Imports ~~ //
use crate::gui::EvoSimGuiPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    render::camera::ScalingMode,
};
use ecs::*;
use net::*;
use simworld::*;

/// Start le simulation
fn main() {
    App::new()
        // Background color & antialiasing (can use FXAA with bevy 0.9)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        // Plugins
        .add_plugins(
            DefaultPlugins
                // Enable debug logging but disable for "loud" crates
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=warn,bevy_ecs=info,naga=info".to_owned(),
                })
                // Window configuration
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
                        resizable: true,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(NetworkEcsPlugin)
        .add_plugin(SimWorldPlugin)
        .add_plugin(EvoSimGuiPlugin)
        // Spawn the camera and essential scene stuff
        .add_startup_system(init_scene_system)
        // And go!
        .run();
}

/// Spawn the essentials into the scene.
fn init_scene_system(mut commands: Commands, assets: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WORLD_SIZE.0 as f32 * 0.5, WORLD_SIZE.1 as f32 * 0.5, 900.0),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Auto {
                min_width: WORLD_SIZE.0 as f32,
                min_height: WORLD_SIZE.1 as f32,
            },
            ..default()
        },
        ..default()
    });

    // Spawn a sample Smitty
    commands.spawn(SmittyBundle {
        brain: SimEntityBrain::random(),
        pos: SimEntityPosRot(
            Vec2::new(WORLD_SIZE.0 as f32 / 2.0, WORLD_SIZE.1 as f32 / 2.0),
            0.0,
        ),
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

/// Resource to keep track of the cursor position.
#[derive(Resource)]
pub struct CursorState {
    cursor_screen_pos: Vec2,
    cursor_world_pos: Vec2,
}

fn update_cursor_system(mut cursor_state: ResMut<CursorState>) {}
