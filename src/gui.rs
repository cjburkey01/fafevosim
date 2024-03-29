use crate::{
    ecs::{SimTime, SimulationMode, SimulationState},
    simworld::{SimTile, SimWorld, WORLD_SIZE},
};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_egui::{
    egui,
    egui::{FontId, RichText, Widget},
    EguiContext, EguiPlugin,
};
use iyes_loopless::state::{CurrentState, NextState};
use num_format::{Locale, ToFormattedString};

const NUM_LOCAL: Locale = Locale::en;

/// Plugin to organize UI systems for the simulator.
pub struct EvoSimGuiPlugin;

impl Plugin for EvoSimGuiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add the resource to keep track of the currently selected smitty
            .init_resource::<SelectedSmitty>()
            .init_resource::<CursorState>()
            // Add the EGui plugin until Bevy's UI handler is better :/
            .add_plugin(EguiPlugin)
            // Add cursor update system
            .add_system_to_stage(CoreStage::First, update_cursor_pos)
            // Add the inspector window for Smitty
            .add_system(smitty_inspector_egui_system);
    }
}

#[derive(Default, Resource)]
pub struct CursorState {
    pub screen_pos: Vec2,
    pub world_pos: Vec2,
    pub tile_pos: Option<(usize, usize)>,
}

/// Resource to keep track of the currently selected smitty's entity ID, or
/// `None` if none is selected.
#[derive(Default, Resource)]
pub struct SelectedSmitty(pub Option<Entity>);

pub struct SmittyRaycastSet;

/// System to update the raycast sender stuff and things and stuff im high idk and idc.
fn update_cursor_pos(
    mut cursor: EventReader<CursorMoved>,
    mut cursor_state: ResMut<CursorState>,
    source_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };
    // Pull the first (and should be ONLY `RaycastSource` in the scene
    let (camera, cam_transform) = source_query.iter().next().unwrap();

    // Update the cursor state
    cursor_state.screen_pos = cursor_position;
    let wp = camera
        .viewport_to_world(cam_transform, cursor_position)
        .unwrap()
        .origin
        .xy();
    cursor_state.world_pos = wp;
    cursor_state.tile_pos =
        if wp.x >= 0.0 && wp.y >= 0.0 && wp.x < WORLD_SIZE.0 as f32 && wp.y < WORLD_SIZE.0 as f32 {
            let f = wp.floor().as_uvec2();
            Some((f.x as usize, f.y as usize))
        } else {
            None
        };
}

/// System to update the inspector window for selected smittys (if one is selected).
fn smitty_inspector_egui_system(
    selected_smitty: Res<SelectedSmitty>,
    cursor_state: Res<CursorState>,
    sim_time: Res<SimTime>,
    sim_world: Res<SimWorld>,
    sim_state: Res<CurrentState<SimulationState>>,
    mut egui_context: ResMut<EguiContext>,
    mut commands: Commands,
) {
    // The simulation window
    egui::Window::new("Simulation")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            let s = sim_state.0;

            // State
            ui.label(format!("Current state: {:?}", s));
            //ui.separator();

            // Simulation time
            ui.label(format!(
                "World Frame: {}",
                sim_time.world_frame.to_formatted_string(&NUM_LOCAL)
            ));
            ui.label(format!(
                "Neural Frame: {}",
                sim_time.neural_frame.to_formatted_string(&NUM_LOCAL)
            ));
            //ui.separator();

            // Add buttons
            ui.horizontal(|ui| {
                // Start button
                if ui
                    .add_enabled(s == SimulationState::Stop, egui::Button::new("Start"))
                    .clicked()
                {
                    info!("resuming simulation");

                    commands.insert_resource(NextState(SimulationState::Run));
                }

                // Stop button
                if ui
                    .add_enabled(s == SimulationState::Run, egui::Button::new("Stop"))
                    .clicked()
                {
                    info!("pausing simulation");

                    commands.insert_resource(NextState(SimulationState::Stop));
                }

                // Step frame button
                if ui
                    .add_enabled(s == SimulationState::Stop, egui::Button::new("Step Frame"))
                    .clicked()
                {
                    info!("stepping simulation by one frame");

                    commands.insert_resource(NextState(SimulationMode::Single));
                }

                // Step neural frame button
                /* if ui
                    .add_enabled(
                        s == SimulationState::Stop,
                        egui::Button::new("Step Neural Frame"),
                    )
                    .clicked()
                {
                    info!("stepping simulation by one frame");

                    commands.insert_resource(NextState(SimulationMode::Brain));
                } */
            });
        });

    // Cursor info window
    egui::Window::new("Cursor Info")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            let csp = cursor_state.screen_pos;
            let cwp = cursor_state.world_pos;
            ui.label(format!("Screen pos: ({:.2}, {:.2})", csp.x, csp.y));
            ui.label(format!("World pos: ({:.2}, {:.2})", cwp.x, cwp.y));
            ui.label(format!(
                "Tile pos: {}",
                match cursor_state.tile_pos {
                    Some((x, y)) => format!("({}, {})", x, y),
                    None => "None".to_owned(),
                }
            ));
        });

    // The entity inspector window
    egui::Window::new("Inspect Smitty")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            // Check if a smitty is selected
            if let Some(selected) = selected_smitty.0 {
                ui.label(format!("Entity: {:?}", selected));
            } else {
                ui.label("No entity selected");
            }
        });

    // The tile inspector window
    egui::Window::new("Inspect Tile")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            if let Some(pos) = cursor_state.tile_pos {
                let tile = sim_world.tile(pos).unwrap();
                ui.label(format!("Type: {:?}", tile.tile_type));
                ui.label(format!("Current food: {:.4}", tile.food));
                ui.label(format!("Max food: {:.4}", tile.max_food));
            } else {
                ui.label("No tile under cursor");
            }
        });
}
