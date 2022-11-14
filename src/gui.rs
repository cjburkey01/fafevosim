use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

/// Plugin to organize UI systems for the simulator.
pub struct EvoSimGuiPlugin;

impl Plugin for EvoSimGuiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add the resource to keep track of the currently selected smitty
            .init_resource::<SelectedSmitty>()
            // Add the EGui plugin until Bevy's UI handler is better :/
            .add_plugin(EguiPlugin)
            // Add the inspector window for Smitty
            .add_system(smitty_inspector_egui_system);
    }
}

/// Resource to keep track of the currently selected smitty's entity ID, or
/// `None` if none is selected.
#[derive(Default, Resource)]
pub struct SelectedSmitty(Option<Entity>);

/// System to update the inspector window for selected smittys (if one is selected).
fn smitty_inspector_egui_system(
    selected_smitty: Res<SelectedSmitty>,
    mut egui_context: ResMut<EguiContext>,
) {
    // The entity inspector window
    egui::Window::new("Inspect Smitty").show(egui_context.ctx_mut(), |ui| {
        // Check if a smitty is selected
        if let Some(selected) = selected_smitty.0 {
            ui.label(format!("Entity: {:?}", selected));
        } else {
            ui.label("No entity selected");
        }
    });
}
