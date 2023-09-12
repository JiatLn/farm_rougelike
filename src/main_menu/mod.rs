use bevy::prelude::*;

use crate::AppState;

mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu_init)
            .add_systems(OnEnter(AppState::MainMenu), systems::spawn_mainmenu_layout)
            .add_systems(
                Update,
                systems::button_system.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), systems::despawn_mainmenu_layout);
    }
}

pub fn main_menu_init() {
    info!("Main menu init!");
}
