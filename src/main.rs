use bevy::prelude::*;
use events::GameOver;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

mod events;
mod game;
mod main_menu;
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
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_event::<GameOver>()
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (systems::handler_game_over, systems::update_high_scores),
        )
        .run();
}
