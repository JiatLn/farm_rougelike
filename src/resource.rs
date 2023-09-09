use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct Money(pub f32);

impl Default for Money {
    fn default() -> Self {
        Money(0.0)
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        let timer = Timer::from_seconds(1.0, TimerMode::Repeating);
        Self { timer }
    }
}
