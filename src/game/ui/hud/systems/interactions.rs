use bevy::prelude::*;

use crate::game::{
    enemy::components::Enemy,
    score::resources::Score,
    ui::hud::components::*
};

pub fn update_score(
    mut query2: Query<&mut Text, With<StarScore>>,
    score: Option<Res<Score>>
) {
    if let Some(score) = score {
        if !score.is_changed() { return; }

        if let Ok(mut text) = query2.get_single_mut() {
            if let Some(section) = text.sections.get_mut(0) {
                section.value = format!("{}", score.value);
            }
        }
    }
}

pub fn update_enemy_count(
    query: Query<&Enemy>,
    mut query2: Query<&mut Text, With<EnemyCount>>,
) {
    let enemy_count = query.iter().len();

    if let Ok(mut text) = query2.get_single_mut() {
        let section = text.sections.get_mut(0).unwrap();
        section.value = format!("{}", enemy_count);
    }
}