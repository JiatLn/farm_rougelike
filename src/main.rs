use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use player::Player;
use resource::Money;

use crate::pig::Pig;

mod pig;
mod player;
mod resource;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Rougelike".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_pig, pig_lifetime))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::hex("#282a36").unwrap()),
        },
        ..default()
    };
    commands.spawn(camera);

    let texture = asset_server.load("player.png");

    let sprite = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    };

    commands.spawn((sprite, Player::new(100.0)));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
    }
}

pub fn spawn_pig(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player = player.single();
    let cost = 10.0;

    if money.0 >= cost {
        money.0 -= cost;

        info!(
            "Spent ${:?} on a pig, remaining money: ${:?}",
            cost, money.0
        );

        let texture = assert_server.load("pig.png");

        let pig = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(90.0, 70.0)),
                ..default()
            },
            texture,
            transform: *player,
            ..default()
        };

        commands.spawn((pig, Pig::new(2.0, 1.5)));
    }
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 10.0 * pig.rate;

            commands.entity(pig_entity).despawn();

            info!(
                "Pig sold for ${}! Current Money: ${:?}",
                10.0 * pig.rate,
                money.0
            )
        }
    }
}
