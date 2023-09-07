use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
};

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
    pub rate: f32,
}

impl Pig {
    pub fn new(life: f32, rate: f32) -> Self {
        let lifetime = Timer::from_seconds(life, TimerMode::Once);
        Pig { lifetime, rate }
    }
}
