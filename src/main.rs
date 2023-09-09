use crate::{pig::Pig, star::Star};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::PrimaryWindow};
use player::Player;
use rand::prelude::*;
use resource::{Money, StarSpawnTimer};

mod pig;
mod player;
mod resource;
mod star;

pub const PLAYER_SIZE: (f32, f32) = (320.0 / 5.0, 370.0 / 5.0);
pub const PIG_SIZE: (f32, f32) = (947.0 / 16.0, 772.0 / 16.0);
pub const STAR_SIZE: (f32, f32) = (30.0 / 2.0, 30.0 / 2.0);
pub const PIG_NUMS: i32 = 5;
pub const STAR_NUMS: i32 = 10;
pub const PIG_SPEED: f32 = 40.0;

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
        .add_systems(Startup, (setup, spawn_pigs, spawn_stars))
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                pigs_movement,
                confine_pigs_movement,
                pigs_hit_player,
                get_star,
                tick_star_spawn_timer,
                spawn_star_over_time,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::hex("#282a36").unwrap()),
        },
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    };
    commands.spawn(camera);

    let texture = asset_server.load("player.png");

    let sprite = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
            ..default()
        },
        texture,
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    };

    commands.spawn((sprite, Player::new(300.0)));
}

fn player_movement(
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

fn confine_player_movement(
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

fn spawn_pigs(
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

fn spawn_stars(
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

fn pigs_movement(mut pig_query: Query<(&mut Transform, &Pig), With<Pig>>, time: Res<Time>) {
    for (mut transform, pig) in pig_query.iter_mut() {
        transform.translation += pig.direction * PIG_SPEED * time.delta_seconds();
    }
}

fn confine_pigs_movement(
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

fn pigs_hit_player(
    player_query: Query<&Transform, With<Player>>,
    mut pigs_query: Query<(&Transform, &mut Pig), With<Pig>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player) = player_query.get_single() {
        for (pig_transform, mut pig) in pigs_query.iter_mut() {
            let distance = player.translation.distance(pig_transform.translation);
            if distance < (PLAYER_SIZE.0 + PIG_SIZE.0) / 2.0 {
                info!("booom");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_002.ogg"),
                    ..default()
                });
                pig.direction = -pig.direction;
            }
        }
    }
}

fn get_star(
    player_query: Query<&Transform, With<Player>>,
    stars_query: Query<(Entity, &Transform), With<Star>>,
    mut commands: Commands,
    mut money: ResMut<Money>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player) = player_query.get_single() {
        for (star_entity, star) in stars_query.iter() {
            let distance = player.translation.distance(star.translation);
            if distance < (PLAYER_SIZE.0 + STAR_SIZE.0) / 2.0 {
                commands.entity(star_entity).despawn();
                money.0 += 10.0;
                info!("Got a star! Current Money: ${:?}", money.0);
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    ..default()
                });
            }
        }
    }
}

fn spawn_star_over_time(
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

fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}
