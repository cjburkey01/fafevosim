//! The ECS components and systems.

use crate::{net::*, simworld::WORLD_SIZE};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use iyes_loopless::prelude::*;
use std::{f32::consts::PI, time::Duration};

/// Neural network update systems fixed timestep name.
pub const FT_NEURAL_UPDATE: &str = "fixed_timestep_start_neural_update";
/// The number of world tick frames between each feed-forward execution through
/// the neural networks.
pub const NETWORK_UPDATE_PERIOD: u32 = 60;

/// The scale of a Smitty.
pub const SMITTY_SCALE: f32 = 1.0;
/// The maximum allowed speed in meters per second a Smitty may move.
pub const SMITTY_MAX_MOVE_SPEED: f32 = 4.0;
/// The maximum radians per second a smitty may rotate.
pub const SMITTY_MAX_ROT_SPEED: f32 = 8.0 * PI; // 4 rot/s

/// The stages within a frame update
#[derive(Debug, Copy, Clone, StageLabel)]
pub enum FrameUpdateStage {
    /// Update the simulation time resource.
    UpdateTiming,
    /// Perform the neural updates if applicable.
    UpdateNeural,
    /// Move entities according to their last brain outputs.
    UpdateEntities,
}

/// The stages within the update stage for the neural network simulation.
#[derive(Debug, Copy, Clone, StageLabel)]
pub enum NeuralUpdateStage {
    /// The stage in which neural network updates run.
    Collect,
    /// The stage in which entities move and interact.
    Update,
    /// The stage in which the world is updated.
    Perform,
}

/// Component containing 32-bit float neural network for a simulation entity
/// (Smitty).
#[derive(Debug, Component)]
pub struct SimEntityBrain {
    /// The neural network.
    pub network: NN<f32>,
}

impl SimEntityBrain {
    pub fn random() -> Self {
        Self {
            // Network with 1 input and 2 outputs
            network: NN::random(&[2, 3, 2]).unwrap(),
        }
    }
}

/// Component containing position and rotation of the entity (Smitty) in the
/// simulation world.
/// The position should be bound to the limited world (probably just clamped).
#[derive(Debug, Component)]
pub struct SimEntityPosRot(pub Vec2, pub f32);

/// The inputs for the Smitty's brain.
#[derive(Debug, Component)]
pub struct SimEntityBrainInputs {}

/// The requested move & rotation speeds.
#[derive(Debug, Component)]
pub struct SimEntityBrainOutputs {
    /// The percentage of this entity's max speed that it wishes to travel.
    pub move_amt: f32,
    /// The percentage of this entity's max rotation speed that it wishes to rotate.
    pub rot_amt: f32,
}

/// Component containing inherited traits for entities in the simulation.
#[derive(Debug, Component)]
pub struct SimEntityTraits {
    /// The maximum speed of this entity (in meters per second).
    pub max_move_speed: f32,
    /// The maximum speed the entity can rotate (in radians per second).
    pub max_rot_speed: f32,
}

/// A single simulation entity.
#[derive(Bundle)]
pub struct SmittyBundle {
    /// The entity's brain.
    pub brain: SimEntityBrain,
    /// The entity's position.
    pub pos: SimEntityPosRot,
    /// The inputs to the entity's brain.
    pub inputs: SimEntityBrainInputs,
    /// The entity's desired movement & rotation velocities.
    pub outputs: SimEntityBrainOutputs,
    /// The entity's traits.
    pub traits: SimEntityTraits,
    /// The entity's sprite
    #[bundle]
    pub sprite: SpriteBundle,
}

/// System to rotate and move the Smittys by their requested amounts.
///
/// This system does not verify that these values are reasonable or allowed!
fn move_smittys_system(
    time: Res<Time>,
    mut query: Query<(
        &mut SimEntityPosRot,
        &SimEntityBrainOutputs,
        &SimEntityTraits,
        &mut Transform,
    )>,
) {
    // Loop through the Smittys
    for (mut pos, request, traits, mut transform) in query.iter_mut() {
        // Get the new rotation
        let rot_req = request.rot_amt * 2.0 - 1.0;
        let mut new_rot = pos.1 + rot_req * traits.max_rot_speed * time.delta_seconds();
        // Wrap between 0 and 1 radian
        let rad = 2.0 * PI;
        if new_rot < 0.0 {
            new_rot += rad;
        } else if new_rot > rad {
            new_rot -= rad;
        }

        // Update the position based on the new rotation
        pos.0 += Vec2::new(new_rot.cos(), new_rot.sin())
            * request.move_amt
            * traits.max_move_speed
            * time.delta_seconds();
        // Wrap the position
        let (w, h) = (WORLD_SIZE.0 as f32, WORLD_SIZE.1 as f32);
        if pos.0.x < 0.0 {
            pos.0.x += w;
        } else if pos.0.x > w {
            pos.0.x -= w;
        }
        if pos.0.y < 0.0 {
            pos.0.y += h;
        } else if pos.0.y > h {
            pos.0.y -= h;
        }
        // Update the rotation
        pos.1 = new_rot;

        // Update the transform component so the sprite moves
        let xy = Vec2::new(pos.0.x, pos.0.y);
        let z = transform.translation.z;
        transform.translation = Vec3::from((xy, z));
    }
}

/// System to collect information for neural network inputs.
fn neural_network_collect_system() {
    debug!("collecting data");
}

/// Perform the network update (feed-forward the previously collected inputs.
fn neural_network_update_system(
    mut brains: Query<(
        &SimEntityBrain,
        &SimEntityBrainInputs,
        &mut SimEntityBrainOutputs,
    )>,
) {
    debug!("updating neural networks");

    // Loop through all the brains in the world
    for (brain, inputs, mut outputs) in brains.iter_mut() {
        // Feed-forward
        let output_results = brain
            .network
            .run(NNActivation::Sigmoid, &[0.5, 0.5])
            .unwrap();

        // Update the output
        outputs.move_amt = output_results[0];
        outputs.rot_amt = output_results[1];
    }
}

/// Update the entity based on its neural network outputs.
fn neural_network_perform_system() {
    debug!("executing network outputs");
}

/// The state of the simulation (i.e. whether it is running, running for a
/// single update, etc.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SimulationState {
    /// The simulation is not currently running.
    Stop,
    /// The simulation is running.
    Run,
}

/// The way the simulation behaves.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SimulationMode {
    /// The engine will perform one tick and switch the `SimulationState` back
    /// to `Stop`.
    Single,
    /* /// The engine will perform however many ticks are between neural updates
       /// and switch the `SimulationState` back to `Stop` afterwards.
    Brain, */
    /// The simulation will run until the `SimulationState` is changed to
    /// `Stop`.
    Auto,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Resource)]
pub struct SimTime {
    /// The current world execution frame count.
    pub world_frame: u32, // Should be good up to 2.2 years of constant running, right?
    /// The current neural tick count.
    pub neural_frame: u32,
    /// The world frame count when the last neural network update was
    /// performed.
    pub last_neural_tick_frame: u32,
    /// Whether this frame is a neural update frame.
    pub is_neural_tick_frame: bool,
}

/// System to update the simulation time resource.
/// When this system is called, the frame is ticked.
fn update_simulation_time_system(mut sim_time: ResMut<SimTime>) {
    // Increment the frame and determine how many frames have passed since the
    // last neural update
    let frame = sim_time.world_frame + 1;
    let delta = frame - sim_time.last_neural_tick_frame;
    sim_time.world_frame = frame;

    // Check if enough frames have passed to perform a neural update
    sim_time.is_neural_tick_frame = delta >= NETWORK_UPDATE_PERIOD;
    if sim_time.is_neural_tick_frame {
        sim_time.neural_frame += 1;
        sim_time.last_neural_tick_frame = frame;
    }
}

/// Simple system to check whether the simulation should run a neural network
/// update this frame.
fn is_neural_update_frame_system(sim_time: Res<SimTime>) -> bool {
    sim_time.is_neural_tick_frame
}

/// The main plugin, the blood & guts so to speak.
pub struct NetworkEcsPlugin;

impl Plugin for NetworkEcsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add resources
            .init_resource::<SimTime>()
            // Add states
            .add_loopless_state(SimulationState::Stop)
            .add_loopless_state(SimulationMode::Single)
            // Add the frame stages
            .add_stage_before(
                CoreStage::Update,
                FrameUpdateStage::UpdateTiming,
                SystemStage::parallel(),
            )
            .add_stage_after(
                FrameUpdateStage::UpdateTiming,
                FrameUpdateStage::UpdateNeural,
                SystemStage::parallel(),
            )
            .add_stage_after(
                FrameUpdateStage::UpdateNeural,
                FrameUpdateStage::UpdateEntities,
                SystemStage::parallel(),
            )
            // Add the brain stages
            .add_stage_after(
                FrameUpdateStage::UpdateNeural,
                NeuralUpdateStage::Collect,
                SystemStage::parallel(),
            )
            .add_stage_after(
                NeuralUpdateStage::Collect,
                NeuralUpdateStage::Update,
                SystemStage::parallel(),
            )
            .add_stage_after(
                NeuralUpdateStage::Update,
                NeuralUpdateStage::Perform,
                SystemStage::parallel(),
            )
            // Add the neural update systems
            .add_system_to_stage(
                NeuralUpdateStage::Collect,
                neural_network_collect_system
                    .run_in_state(SimulationState::Run)
                    .run_if(is_neural_update_frame_system),
            )
            .add_system_to_stage(
                NeuralUpdateStage::Update,
                neural_network_update_system
                    .run_in_state(SimulationState::Run)
                    .run_if(is_neural_update_frame_system),
            )
            .add_system_to_stage(
                NeuralUpdateStage::Perform,
                neural_network_perform_system
                    .run_in_state(SimulationState::Run)
                    .run_if(is_neural_update_frame_system),
            )
            // Add the per-frame systems for when the simulation is running
            .add_system_set_to_stage(
                FrameUpdateStage::UpdateEntities,
                ConditionSet::new()
                    .run_in_state(SimulationState::Run)
                    .with_system(move_smittys_system)
                    .with_system(update_simulation_time_system)
                    .into(),
            );
    }
}
