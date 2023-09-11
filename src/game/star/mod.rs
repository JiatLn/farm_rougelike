use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub const STAR_SIZE: (f32, f32) = (30.0 / 2.0, 30.0 / 2.0);
pub const STAR_NUMS: i32 = 10;

mod systems;

#[derive(Component)]
pub struct Star {}

impl Star {
    pub fn new() -> Self {
        Star {}
    }
}

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), systems::spawn_stars)
            .add_systems(OnExit(AppState::Game), systems::despawn_stars)
            .add_systems(
                Update,
                (
                    systems::spawn_star_over_time,
                    systems::tick_star_spawn_timer,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
