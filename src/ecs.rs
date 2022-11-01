//! The ECS components and systems.

use crate::net::*;
use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;
use std::time::Duration;

/// Neural network update systems fixed timestep name.
pub const FT_NEURAL_UPDATE: &str = "fixed_timestep_start_neural_update";
pub const NETWORK_UPDATE_PERIOD: Duration = Duration::from_millis(1000);

/// A 32-bit float neural network.
#[derive(Component)]
pub struct NeuralNetwork {
    pub network: NN<f64>,
    pub activate_func: Box<dyn ActivationFunction<f64>>,
}

/// System to collect information for neural network inputs.
fn neural_network_collect_system() {
    debug!("collecting data");
}

/// Perform the network update (feed-forward the previously collected inputs.
fn neural_network_update_system() {
    debug!("updating neural networks");
}

/// Update the entity based on its neural network outputs.
fn neural_network_perform_system() {
    debug!("executing network outputs");
}

pub struct NetworkEcsPlugin;

impl Plugin for NetworkEcsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add the neural update fixed timestep systems
            .add_fixed_timestep_after_stage(
                CoreStage::PreUpdate,
                NETWORK_UPDATE_PERIOD,
                FT_NEURAL_UPDATE,
            )
            .add_fixed_timestep_child_stage(FT_NEURAL_UPDATE)
            .add_fixed_timestep_child_stage(FT_NEURAL_UPDATE)
            .add_fixed_timestep_system(FT_NEURAL_UPDATE, 0, neural_network_collect_system)
            .add_fixed_timestep_system(FT_NEURAL_UPDATE, 1, neural_network_update_system)
            .add_fixed_timestep_system(FT_NEURAL_UPDATE, 2, neural_network_perform_system);
    }
}
