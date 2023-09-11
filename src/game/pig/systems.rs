use crate::game::pig::{Pig, PIG_NUMS, PIG_SIZE};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::PIG_SPEED;

pub fn spawn_pigs(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let width = window.width();
    let height = window.height();

    for _ in 0..PIG_NUMS {
        let mut rng = thread_rng();

        let rand_x = rng.gen_range(0.0..width);
        let rand_y = rng.gen_range(0.0..height);

        let pig = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PIG_SIZE.0, PIG_SIZE.1)),
                ..default()
            },
            texture: assert_server.load("pig.png"),
            transform: Transform::from_xyz(rand_x, rand_y, 0.0),
            ..default()
        };

        commands.spawn((pig, Pig::new(2.0, 1.5)));
    }

    info!("Spent {} pigs", PIG_NUMS);
}

pub fn pigs_movement(mut pig_query: Query<(&mut Transform, &Pig), With<Pig>>, time: Res<Time>) {
    for (mut transform, pig) in pig_query.iter_mut() {
        transform.translation += pig.direction * PIG_SPEED * time.delta_seconds();
    }
}

pub fn confine_pigs_movement(
    mut pig_query: Query<(&mut Transform, &mut Pig), With<Pig>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let x_min = 0.0 + PIG_SIZE.0 / 2.0;
    let x_max = window.width() - PIG_SIZE.0 / 2.0;
    let y_min = 0.0 + PIG_SIZE.1 / 2.0;
    let y_max = window.height() - PIG_SIZE.1 / 2.0;

    for (mut transform, mut pig) in pig_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
            pig.direction.x = -pig.direction.x;
        } else if translation.x > x_max {
            translation.x = x_max;
            pig.direction.x = -pig.direction.x;
        }

        if translation.y < y_min {
            translation.y = y_min;
            pig.direction.y = -pig.direction.y;
        } else if translation.y > y_max {
            translation.y = y_max;
            pig.direction.y = -pig.direction.y;
        }

        transform.translation = translation;
    }
}

pub fn despawn_pigs(mut commands: Commands, pig_query: Query<Entity, With<Pig>>) {
    for pig in pig_query.iter() {
        commands.entity(pig).despawn();
    }
}
