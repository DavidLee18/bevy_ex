use bevy::prelude::*;

use crate::game::{ui::{
    game_over_menu::{
        components::*, 
        styles::*
    },
    TITLE_STYLE,
    NORMAL_BUTTON_COLOR,
    BUTTON_STYLE
}, score::resources::HighScores};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    final_score: Res<HighScores>
) {
    commands.spawn((NodeBundle {
        style: GAME_OVER_MENU_STYLE,
        background_color: Color::GRAY.into(),
        ..Default::default()
    }, GameOverMenu))
    .with_children(|parent| {
        // Title
        parent.spawn(NodeBundle {
            style: TITLE_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(get_text_bundle(&asset_server, "Game Over", 64.0));
        });

        // Score Display
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(280.0), Val::Px(100.0)),
                ..TITLE_STYLE
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(get_text_bundle(&asset_server, "Your Final Score is:", 64.0));
        });

        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(280.0), Val::Px(100.0)),
                ..TITLE_STYLE
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                get_text_bundle(&asset_server, format!("{}", final_score.scores.last().unwrap().1).as_str(), 60.0),
                FinalScoreBoard));
        });

        // Restart Button
        parent.spawn((ButtonBundle {
            background_color: NORMAL_BUTTON_COLOR.into(),
            style: BUTTON_STYLE,
            ..Default::default()
        }, RestartButton))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(&asset_server, "Resume", 32.0));
        });

        // Main Menu Button
        parent.spawn((ButtonBundle {
            background_color: NORMAL_BUTTON_COLOR.into(),
            style: BUTTON_STYLE,
            ..Default::default()
        }, MainMenuButton))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(&asset_server, "Main Menu", 32.0));
        });

        // Quit Button
        parent.spawn((ButtonBundle {
            background_color: NORMAL_BUTTON_COLOR.into(),
            style: BUTTON_STYLE,
            ..Default::default()
        }, QuitButton))
        .with_children(|parent| {
            parent.spawn(get_text_bundle(&asset_server, "Quit", 32.0));
        });
    });
}

pub fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}