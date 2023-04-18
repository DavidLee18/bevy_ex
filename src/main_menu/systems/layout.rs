use bevy::prelude::*;

use crate::main_menu::{components::*, styles::*};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Image 1
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..Default::default()
                    });
                    // Text
                    parent.spawn(get_text_bundle(&asset_server, "Bevy Ball Game", 64.0));
                    // Image 2
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..Default::default()
                    });
                });
            // Play Button
            parent
                .spawn((
                    ButtonBundle {
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        style: BUTTON_STYLE,
                        ..Default::default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(get_text_bundle(&asset_server, "Play", 32.0));
                });
            // Quit Button
            parent
                .spawn((
                    ButtonBundle {
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        style: BUTTON_STYLE,
                        ..Default::default()
                    },
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(get_text_bundle(&asset_server, "Quit", 32.0));
                });
        });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
