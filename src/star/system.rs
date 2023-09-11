use super::{Star, STAR_NUMS, STAR_SIZE};
use crate::resource::StarSpawnTimer;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub fn spawn_stars(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let width = window.width();
    let height = window.height();

    for _ in 0..STAR_NUMS {
        let mut rng = thread_rng();

        let rand_x = rng.gen_range(STAR_SIZE.0 / 2.0..width - STAR_SIZE.0 / 2.0);
        let rand_y = rng.gen_range(STAR_SIZE.1 / 2.0..height - STAR_SIZE.1 / 2.0);

        let star = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(STAR_SIZE.0, STAR_SIZE.1)),
                ..default()
            },
            texture: assert_server.load("star.png"),
            transform: Transform::from_xyz(rand_x, rand_y, 0.0),
            ..default()
        };

        commands.spawn((star, Star::new()));
    }
}

pub fn spawn_star_over_time(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if !star_spawn_timer.timer.finished() {
        return;
    }

    let window = window_query.single();
    let width = window.width();
    let height = window.height();

    let mut rng = thread_rng();

    let rand_x = rng.gen_range(STAR_SIZE.0 / 2.0..width - STAR_SIZE.0 / 2.0);
    let rand_y = rng.gen_range(STAR_SIZE.1 / 2.0..height - STAR_SIZE.1 / 2.0);

    let star = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(STAR_SIZE.0, STAR_SIZE.1)),
            ..default()
        },
        texture: assert_server.load("star.png"),
        transform: Transform::from_xyz(rand_x, rand_y, 0.0),
        ..default()
    };

    commands.spawn((star, Star::new()));
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}
