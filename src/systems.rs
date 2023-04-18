use bevy::{
    prelude::*,
    window::PrimaryWindow,
    app::AppExit
};

use crate::{
    events::GameOver,
    AppState,
    game::SimulationState
};


pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_state(keycode: KeyCode, state: AppState)
-> impl for<'a, 'b, 'c, 'd> Fn(
    Res<'a, Input<KeyCode>>,
    Res<'b, State<AppState>>,
    ResMut<'c, NextState<AppState>>,
    ResMut<'d, NextState<SimulationState>>
) {
    move | keyboard_input,
           app_state,
           mut next_state,
           mut next_simulation
    | {
        if keyboard_input.just_pressed(keycode) && app_state.0 != state {
            println!("Entered {:?}", state);
            next_state.set(state);
            next_simulation.set(SimulationState::Paused);
        }
    }
}

// fn transition_to_game_state(
//     keyboard_input: Res<Input<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     mut next_state: ResMut<NextState<AppState>>
// ) {
//     if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::Game {
//         println!("Entered AppState::Game");
//         next_state.set(AppState::Game);
//     }
// }

// fn transition_to_main_menu_state(
//     keyboard_input: Res<Input<KeyCode>>,
//     app_state: Res<State<AppState>>,
//     mut next_state: ResMut<NextState<AppState>>
// ) {
//     if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::MainMenu {
//         println!("Entered AppState::MainMenu");
//         next_state.set(AppState::MainMenu);
//     }
// }

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>, mut next_state: ResMut<NextState<AppState>>) {
    for event in &mut game_over_event_reader {
        println!("Your final score is: {}", event.score);
        next_state.set(AppState::GameOver);
    }
}