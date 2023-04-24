mod components;
mod systems;
pub mod style;

use bevy::prelude::*;

use crate::{AppState, game::SimulationState};

use self::systems::{layout::*, interactions::*};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
        .add_systems((update_score, update_enemy_count)
            .in_set(OnUpdate(SimulationState::Running)))
        .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}