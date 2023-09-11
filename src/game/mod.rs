use bevy::prelude::*;

mod pig;
mod player;
mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, pig::PigPlugin, star::StarPlugin));
    }
}
