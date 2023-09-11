use super::SimulationState;
use crate::resources::Score;
use bevy::prelude::*;

pub fn toggle_simulation(
    mut commands: Commands,
    kb_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if kb_input.just_pressed(KeyCode::Space) {
        if simulation_state.get() == &SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            info!("Simulation Paused!");
        }

        if simulation_state.get() == &SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            info!("Simulation Running!");
        }
    }
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}
