pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // this is the enemy sprite size

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
        .add_systems((
            enemy_movement
            // .run_if(in_state(AppState::Game))
            // .run_if(in_state(SimulationState::Running))
            ,
            update_enemy_direction,
            confine_enemy_movement,
            tick_enemy_spawn_timer,
            spawn_enemies_over_time
        )
        .in_set(OnUpdate(AppState::Game))
        .in_set(OnUpdate(SimulationState::Running))
        )
        .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}