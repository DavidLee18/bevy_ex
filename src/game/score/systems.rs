use bevy::prelude::*;

use crate::events::GameOver;

use super::resources::{Score, HighScores};

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        info!("score: {}", score.value);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>
) {
    for event in &mut game_over_event_reader {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        info!("High scores: {:?}", high_scores);
    }
}