use crate::{
    events::GameOver,
    resources::{HighScores, StarSpawnTimer},
    AppState,
};
use bevy::prelude::*;

mod pig;
mod player;
mod star;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .init_resource::<HighScores>()
            .init_resource::<StarSpawnTimer>()
            .add_plugins((player::PlayerPlugin, pig::PigPlugin, star::StarPlugin))
            .add_systems(OnEnter(AppState::Game), systems::insert_score)
            .add_systems(
                Update,
                systems::toggle_simulation.run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), systems::remove_score);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
