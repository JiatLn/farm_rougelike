use bevy::{
    prelude::{Component, Vec3},
    time::{Timer, TimerMode},
};
use rand::prelude::*;

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
