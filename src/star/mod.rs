use bevy::prelude::*;

pub const STAR_SIZE: (f32, f32) = (30.0 / 2.0, 30.0 / 2.0);
pub const STAR_NUMS: i32 = 10;

mod system;

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
        app.add_systems(Startup, system::spawn_stars).add_systems(
            Update,
            (system::spawn_star_over_time, system::tick_star_spawn_timer),
        );
    }
}
