use crate::{events::GameOver, game::SimulationState, resources::HighScores, AppState};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::PrimaryWindow};

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::hex("#282a36").unwrap()),
        },
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    };
    commands.spawn(camera);
}

pub fn handler_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut commands: Commands,
) {
    for event in game_over_event_reader.iter() {
        info!("Booom!!! Game over!");
        info!("Your final score is {}.", event.score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(event.score);
    }
    if high_scores.is_changed() {
        info!("{:?}", high_scores);
    }
}

pub fn transition_to_game_state(
    mut commands: Commands,
    kb_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if kb_input.just_pressed(KeyCode::G) && app_state.get() != &AppState::Game {
        commands.insert_resource(NextState(Some(AppState::Game)));
        commands.insert_resource(NextState(Some(SimulationState::Running)));
        info!("Entered AppState::Game");
    }
}
pub fn transition_to_main_menu_state(
    mut commands: Commands,
    kb_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if kb_input.just_pressed(KeyCode::M) && app_state.get() != &AppState::MainMenu {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
        info!("Entered AppState::MainMenu");
    }
}
