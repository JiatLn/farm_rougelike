use super::SimulationState;
use crate::resources::Score;
use bevy::prelude::*;

pub fn toggle_simulation(
    kb_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_writer: ResMut<NextState<SimulationState>>,
) {
    if kb_input.just_pressed(KeyCode::Space) {
        if simulation_state.get() == &SimulationState::Running {
            simulation_state_writer.set(SimulationState::Paused);
            info!("Simulation Paused!");
        }

        if simulation_state.get() == &SimulationState::Paused {
            simulation_state_writer.set(SimulationState::Running);
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
