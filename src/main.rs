pub mod components;
pub mod resources;
pub mod systems;

use bevy::{prelude::*, log::{LogPlugin, Level}};
use resources::*;
use systems::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct Movement;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct AfterMovement;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
        level: Level::INFO,
        ..Default::default()
    }))
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .init_resource::<FoodTimer>()
    .init_resource::<SnakeTimer>()
    .configure_set(Movement.before(AfterMovement))
    .add_startup_systems((init_win, setup_cam, spawn_snake))
    .add_systems((
        snake_movement,
        tick_food_timer,
        spawn_food,
        snake_movement_input.before(snake_movement)
    ).in_set(Movement))
    .add_systems((size_scaling, position_translation)
        .in_set(AfterMovement))
    .run();
}

