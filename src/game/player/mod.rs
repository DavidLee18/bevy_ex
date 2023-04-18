mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0; // this is the player sprite size

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_systems((
            player_movement,
            confine_player_movement.after(player_movement),
            enemy_hit_player,
            player_hit_star
        )
        .in_set(OnUpdate(AppState::Game))
        .in_set(OnUpdate(SimulationState::Running))
    )
    .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}