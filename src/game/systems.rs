use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.0 = match simulation_state.0 {
            SimulationState::Running => {
                info!("Simulation Paused.");
                Some(SimulationState::Paused)
            }
            SimulationState::Paused => {
                info!("Simulation Running.");
                Some(SimulationState::Running)
            },
        }
    }
}