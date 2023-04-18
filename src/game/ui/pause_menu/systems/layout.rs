use bevy::prelude::*;

use crate::game::ui::{
    pause_menu::{
        components::*,
        styles::PAUSE_MENU_STYLE
    }, TITLE_STYLE, get_text_bundle, BUTTON_STYLE, NORMAL_BUTTON_COLOR
};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((NodeBundle {
        style: PAUSE_MENU_STYLE,
        background_color: Color::GRAY.into(),
        ..Default::default()
    }, PauseMenu))
    .with_children(|parent| {
        // Title
        parent.spawn(NodeBundle {
            style: TITLE_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(get_text_bundle(&asset_server, "Pause Menu", 64.0));
        });

        // Resume Button
        parent.spawn((ButtonBundle {
            background_color: NORMAL_BUTTON_COLOR.into(),
            style: BUTTON_STYLE,
            ..Default::default()
        }, ResumeButton))
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

pub fn hide_pause_menu(mut query: Query<&mut Style, With<PauseMenu>>) {
    if let Ok(mut style) = query.get_single_mut() {
        style.display = Display::None;
    }
}

pub fn show_pause_menu(mut query: Query<&mut Style, With<PauseMenu>>) {
    if let Ok(mut style) = query.get_single_mut() {
        style.display = Display::Flex;
    }
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}