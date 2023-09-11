use bevy::prelude::*;
use events::GameOver;
use game::GamePlugin;
use resources::{HighScores, Score, StarSpawnTimer};

mod events;
mod game;
mod resources;
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
        .add_plugins(GamePlugin)
        .run();
}
