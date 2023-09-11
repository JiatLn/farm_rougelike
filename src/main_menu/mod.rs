use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu_init);
    }
}

pub fn main_menu_init() {
    info!("Main menu init!");
}
