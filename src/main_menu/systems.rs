use bevy::{app::AppExit, prelude::*};

use crate::AppState;

#[derive(Component)]
pub struct MainMenuLayout;

#[derive(Component, Debug)]
pub enum ButtonType {
    QuitButton,
    PlayButton,
}

const NORMAL_BUTTON: Color = Color::YELLOW_GREEN;
const HOVERED_BUTTON: Color = Color::LIME_GREEN;
const PRESSED_BUTTON: Color = Color::LIME_GREEN;

pub fn spawn_mainmenu_layout(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // fill the entire window
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(16.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            MainMenuLayout {},
        ))
        .with_children(|builder| {
            spawn_title(builder, "Collect Star");
            spawn_button(builder, "Play", ButtonType::PlayButton);
            spawn_button(builder, "Quit", ButtonType::QuitButton);
        });
}

fn spawn_button(builder: &mut ChildBuilder, text: &'static str, button_type: ButtonType) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    // fill the entire window
                    width: Val::Px(100.),
                    height: Val::Px(30.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(NORMAL_BUTTON),
                ..Default::default()
            },
            button_type,
        ))
        .with_children(|builder| {
            spawn_button_text(builder, text);
        });
}

fn spawn_button_text(builder: &mut ChildBuilder, text: &'static str) {
    builder.spawn(TextBundle::from_section(
        text.to_string(),
        TextStyle {
            font_size: 24.0,
            color: Color::BLACK,
            ..default()
        },
    ));
}

fn spawn_title(builder: &mut ChildBuilder, text: &'static str) {
    builder.spawn(TextBundle::from_section(
        text.to_string(),
        TextStyle {
            font_size: 36.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}

pub fn despawn_mainmenu_layout(
    mut commands: Commands,
    layout_query: Query<Entity, With<MainMenuLayout>>,
) {
    let entity = layout_query.single();
    commands.entity(entity).despawn_recursive();
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_writer: EventWriter<AppExit>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut bg_color, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Pressed");
                *bg_color = BackgroundColor(PRESSED_BUTTON);
                match button_type {
                    ButtonType::QuitButton => {
                        app_exit_writer.send(AppExit);
                    }
                    ButtonType::PlayButton => {
                        app_state.set(AppState::Game);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(HOVERED_BUTTON);
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    ..default()
                });
            }
            Interaction::None => {
                *bg_color = BackgroundColor(NORMAL_BUTTON);
            }
        }
    }
}
