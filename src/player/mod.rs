use bevy::prelude::*;

mod systems;

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
        app.add_systems(Startup, systems::spawn_player)
            // .add_systems(Update, systems::player_movement)
            .add_systems(Update, systems::collect_star)
            .add_systems(Update, systems::pig_hit_player)
            .add_systems(
                Update,
                (systems::player_movement, systems::confine_player_movement).chain(),
            );
    }
}
