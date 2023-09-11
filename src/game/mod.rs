use bevy::prelude::*;

use crate::resources::{HighScores, Score, StarSpawnTimer};

mod pig;
mod player;
mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .init_resource::<StarSpawnTimer>()
            .add_plugins((player::PlayerPlugin, pig::PigPlugin, star::StarPlugin));
    }
}
