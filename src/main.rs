use bevy::prelude::*;
use events::GameOver;
use pig::PigPlugin;
use player::PlayerPlugin;
use resources::{HighScores, Score, StarSpawnTimer};
use star::StarPlugin;

mod events;
mod pig;
mod player;
mod resources;
mod star;
mod systems;

fn main() {
    let default_plugins = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Logic Farming Rougelike".into(),
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins(default_plugins)
        .add_event::<GameOver>()
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (systems::handler_game_over, systems::update_high_scores),
        )
        .add_plugins((PlayerPlugin, PigPlugin, StarPlugin))
        .run();
}
