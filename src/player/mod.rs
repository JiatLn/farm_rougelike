use bevy::prelude::*;

mod system;

pub const PLAYER_SIZE: (f32, f32) = (320.0 / 5.0, 370.0 / 5.0);

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Player {
    pub fn new(speed: f32) -> Self {
        Player { speed }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system::spawn_player)
            .add_systems(Update, system::confine_player_movement)
            .add_systems(Update, system::player_movement)
            .add_systems(Update, system::get_star)
            .add_systems(Update, system::pigs_hit_player)
            .add_systems(Update, system::confine_player_movement);
    }
}
