use crate::{
    events::GameOver,
    game::SimulationState,
    resources::{Graphis, HighScores},
    AppState,
};
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
    mut app_state_writer: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        info!("Booom!!! Game over!");
        info!("Your final score is {}.", event.score);
        app_state_writer.set(AppState::MainMenu);
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
    kb_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_writer: ResMut<NextState<AppState>>,
    mut simulation_state_writer: ResMut<NextState<SimulationState>>,
) {
    if kb_input.just_pressed(KeyCode::G) && app_state.get() != &AppState::Game {
        app_state_writer.set(AppState::Game);
        simulation_state_writer.set(SimulationState::Running);
        info!("Entered AppState::Game");
    }
}
pub fn transition_to_main_menu_state(
    kb_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_writer: ResMut<NextState<AppState>>,
) {
    if kb_input.just_pressed(KeyCode::M) && app_state.get() != &AppState::MainMenu {
        app_state_writer.set(AppState::MainMenu);
        info!("Entered AppState::MainMenu");
    }
}

pub fn load_graphis_system(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut texture_altas: ResMut<Assets<TextureAtlas>>,
) {
    let texture = assert_server.load("gabe-idle-run.png");
    let altas = TextureAtlas::from_grid(texture, Vec2::splat(24.0), 7, 1, None, None);

    let handle_texture_atlas = texture_altas.add(altas);

    let graphis = Graphis {
        texture_altas: handle_texture_atlas,
    };

    commands.insert_resource(graphis);
}
