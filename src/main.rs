use bevy::prelude::*;
use pig::PigPlugin;
use player::PlayerPlugin;
use resource::{Money, StarSpawnTimer};
use star::StarPlugin;

mod pig;
mod player;
mod resource;
mod star;
mod system;

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
        .init_resource::<Money>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, system::setup)
        .add_plugins((PlayerPlugin, PigPlugin, StarPlugin))
        .run();
}
