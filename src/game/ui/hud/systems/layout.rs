use bevy::prelude::*;

use crate::game::{
    enemy::components::Enemy,
    star::components::Star,
    ui::hud::{
        components::*,
        style::*
    }
};

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Star>,
    query2: Query<&Enemy>
) {
    let (star_count, enemy_count) = (query.iter().len(), query2.iter().len());

    commands
    .spawn((NodeBundle {
        background_color: Color::NONE.into(),
        style: HUD_STYLE,
        ..Default::default()
    }, Hud))
    .with_children(|parent| {

        // Scoreboard for Star
        parent
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            style: STAR_BOARD_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            // Star Image
            parent.spawn(ImageBundle {
                image: UiImage { texture: asset_server.load("sprites/star.png"), ..Default::default() },
                style: Style {
                    size: Size::all(Val::Percent(2.0)),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Score for Star
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("{}", star_count),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 64.0,
                                color: Color::WHITE
                            }
                        }
                    ],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            }, StarScore));
        });


        // Board for enemy count
        parent
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            style: Style {
                size: Size::all(Val::Percent(4.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Enemy Image
            parent.spawn(ImageBundle {
                image: UiImage { texture: asset_server.load("sprites/ball_red_large.png"), ..Default::default() },
                style: Style {
                    size: Size::all(Val::Percent(2.0)),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Enemy Count
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("{}", enemy_count),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 64.0,
                                color: Color::WHITE
                            }
                        }
                    ],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            }, EnemyCount));
        });
    });
}

pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<Hud>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}