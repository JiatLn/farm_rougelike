use bevy::prelude::*;
use rand::prelude::*;

pub const PIG_SIZE: (f32, f32) = (947.0 / 16.0, 772.0 / 16.0);
pub const PIG_NUMS: i32 = 5;
pub const PIG_SPEED: f32 = 40.0;

mod systems;

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
    pub rate: f32,
    pub direction: Vec3,
}

impl Pig {
    pub fn new(life: f32, rate: f32) -> Self {
        let lifetime = Timer::from_seconds(life, TimerMode::Once);
        let mut rng = thread_rng();

        let rand_x = rng.gen_range(-1.0..1.0);
        let rand_y = rng.gen_range(-1.0..1.0);
        let mut direction = Vec3::new(rand_x, rand_y, 0.0);
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        Pig {
            lifetime,
            rate,
            direction,
        }
    }
}

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_pigs).add_systems(
            Update,
            (systems::pigs_movement, systems::confine_pigs_movement),
        );
    }
}
