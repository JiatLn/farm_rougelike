use super::{Player, PLAYER_SIZE};
use crate::{
    events::GameOver,
    game::pig::{Pig, PIG_SIZE},
    game::star::{Star, STAR_SIZE},
    resources::Score,
};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let player = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
            ..default()
        },
        texture: asset_server.load("player.png"),
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    };

    commands.spawn((player, Player::new(300.0)));
}

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * player.speed * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let x_min = 0.0 + PLAYER_SIZE.0 / 2.0;
    let x_max = window.width() - PLAYER_SIZE.0 / 2.0;
    let y_min = 0.0 + PLAYER_SIZE.1 / 2.0;
    let y_max = window.height() - PLAYER_SIZE.1 / 2.0;

    if let Ok(mut palyer_transform) = player_query.get_single_mut() {
        let mut transition = palyer_transform.translation;

        if transition.x < x_min {
            transition.x = x_min
        } else if transition.x > x_max {
            transition.x = x_max
        }

        if transition.y < y_min {
            transition.y = y_min
        } else if transition.y > y_max {
            transition.y = y_max
        }

        palyer_transform.translation = transition;
    }
}

pub fn collect_star(
    player_query: Query<&Transform, With<Player>>,
    stars_query: Query<(Entity, &Transform), With<Star>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player) = player_query.get_single() {
        for (star_entity, star) in stars_query.iter() {
            let distance = player.translation.distance(star.translation);
            if distance < (PLAYER_SIZE.0 + STAR_SIZE.0) / 2.0 {
                commands.entity(star_entity).despawn();
                score.0 += 1;
                info!("Collected a star! Current score: {}", score.0);
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    ..default()
                });
            }
        }
    }
}

pub fn pig_hit_player(
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut pigs_query: Query<(&Transform, &mut Pig), With<Pig>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>,
) {
    // for protect player first init dead immediately.
    if score.0 == 0 {
        return;
    }

    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for (pig_transform, mut pig) in pigs_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(pig_transform.translation);
            if distance < (PLAYER_SIZE.0 + PIG_SIZE.0) / 2.0 {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_002.ogg"),
                    ..default()
                });
                pig.direction = -pig.direction;
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.0 });
                break;
            }
        }
    }
}
