pub mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_plugin(MainMenuPlugin)
    .add_plugin(GamePlugin)
    .add_startup_system(spawn_camera)
    .add_systems((
        transition_to_state(KeyCode::G, AppState::Game),
        transition_to_state(KeyCode::M, AppState::MainMenu),
        exit_game,
        handle_game_over
    ))
    .run()
}

#[derive(States, PartialEq, Eq, Clone, Copy, Default, Debug, Hash)]
pub enum AppState {
    #[default] MainMenu,
    Game,
    GameOver
}