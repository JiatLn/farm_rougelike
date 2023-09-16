use bevy::{
    prelude::*,
    sprite::TextureAtlas,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct Score(pub i32);

impl Default for Score {
    fn default() -> Self {
        Score(0)
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<i32>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: vec![] }
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

#[derive(Resource)]
pub struct Graphis {
    pub texture_altas: Handle<TextureAtlas>,
}
