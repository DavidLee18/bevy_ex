pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use ui::GameUiPlugin;

use crate::{
    events::GameOver,
    AppState
};

use systems::toggle_simulation;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_plugin(GameUiPlugin)
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, PartialEq, Eq, Clone, Copy, Hash, Default, Debug)]
pub enum SimulationState {
    Running,
    #[default] Paused
}