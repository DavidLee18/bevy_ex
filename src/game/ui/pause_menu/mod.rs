mod systems;
mod components;
mod styles;

use bevy::prelude::*;

use systems::interactions::*;

use crate::{game::SimulationState, AppState};

use systems::layout::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
        .add_system(hide_pause_menu.in_schedule(OnExit(AppState::Game)))
        .add_system(show_pause_menu.in_schedule(OnEnter(AppState::Game)))
        .add_systems((interact_with_resume_button, interact_with_main_menu_button, interact_with_quit_button)
            .in_set(OnUpdate(SimulationState::Paused)))
        .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}