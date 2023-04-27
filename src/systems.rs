use bevy::{
    prelude::*,
    window::{
        PrimaryWindow,
        WindowResolution
    }
};
use rand::random;
use crate::{
    SNAKE_HEAD_COLOR,
    ARENA_WIDTH,
    ARENA_HEIGHT,
    components::{*, self}, resources::*, FOOD_COLOR,
};

pub fn init_win(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = query.get_single_mut() {
        window.title = "Snake!".to_string();
        window.resolution = WindowResolution::new(500.0, 500.0);
        window.position = WindowPosition::Centered(MonitorSelection::Primary);
    }
}

pub fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_snake(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Default::default()
        },
        ..Default::default()
    },
    SnakeHead { direction: components::Direction::Up },
    Position { x: 3, y: 3 },
    components::Size::square(0.8)));
}

pub fn spawn_food(mut commands: Commands, timer: Res<FoodTimer>) {
    if timer.0.finished() {
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
                },
                ..Default::default()
            },
            Food,
            Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            },
            components::Size::square(0.8)
        ));
    }
}

pub fn snake_movement(
    mut heads: Query<(&mut Position, &SnakeHead)>,
    timer: Res<SnakeTimer>
) {
    if timer.0.finished() {
        for (mut pos, head) in &mut heads {
            match head.direction {
                components::Direction::Left => { pos.x -= 1; }
                components::Direction::Right => { pos.x += 1; },
                components::Direction::Up => { pos.y += 1; },
                components::Direction::Down => { pos.y -= 1; },
            }
        }
    }
}

pub fn snake_movement_input(
    keyboard: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>
) {
    for mut head in &mut heads {
        let dir = if keyboard.pressed(KeyCode::Left) {
            {
                info!("Left Key pressed.");
                components::Direction::Left
            }
        } else if keyboard.pressed(KeyCode::Right) {
            {
                info!("Right Key pressed.");
                components::Direction::Right
            }
        } else if keyboard.pressed(KeyCode::Up) {
            {
                info!("Up Key pressed.");
                components::Direction::Up
            }
        } else if keyboard.pressed(KeyCode::Down) {
            {
                info!("Down Key pressed.");
                components::Direction::Down
            }
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn size_scaling(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&components::Size, &mut Transform)>,
) {
    if let Ok(window) = window.get_single() {
        for (sprite_size, mut transform) in &mut query {
            transform.scale = Vec3::new(
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0
            );
        }
    }
}

pub fn position_translation(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }

    if let Ok(window) = window.get_single() {
        for (pos, mut transform) in &mut query {
            transform.translation += Vec3::new(
                convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
                0.0
            );
        }
    }
}

pub fn tick_food_timer(mut food_timer: ResMut<FoodTimer>, time: Res<Time>) {
    food_timer.0.tick(time.delta());
}

pub fn tick_snake_timer(mut snake_timer: ResMut<SnakeTimer>, time: Res<Time>) {
    snake_timer.0.tick(time.delta());
}