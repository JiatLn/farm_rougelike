use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

mod systems;

pub const PLAYER_SIZE: (f32, f32) = (320.0 / 5.0, 370.0 / 5.0);

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new(speed: f32) -> Self {
        Player { speed }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), systems::spawn_player)
            .add_systems(OnExit(AppState::Game), systems::despawn_player)
            .add_systems(
                Update,
                (systems::collect_star, systems::pig_hit_player)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (systems::player_movement, systems::confine_player_movement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                    .chain(),
            );
    }
}
