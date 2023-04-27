use bevy::prelude::*;

#[derive(Resource)]
pub struct FoodTimer(pub Timer);

impl Default for FoodTimer {
    fn default() -> Self {
        Self (Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct SnakeTimer(pub Timer);

impl Default for SnakeTimer {
    fn default() -> Self {
        Self (Timer::from_seconds(0.150, TimerMode::Repeating))
    }
}