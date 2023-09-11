use bevy::prelude::*;

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
        app.add_systems(Startup, systems::spawn_stars).add_systems(
            Update,
            (
                systems::spawn_star_over_time,
                systems::tick_star_spawn_timer,
            ),
        );
    }
}
