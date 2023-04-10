use bevy::prelude::*;

pub mod resources;
mod systems;

use crate::AppState;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<HighScores>()
            // On Enter State
            .add_system(insert_score.in_schedule(OnEnter(AppState::Menu)))
            .add_system(insert_lives.in_schedule(OnEnter(AppState::Intro)))
            .add_system(insert_stage.in_schedule(OnEnter(AppState::Intro)))
            // Systems
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_lives.in_set(OnUpdate(AppState::Game)))
            .add_system(update_stage.in_set(OnUpdate(AppState::LevelUp)))
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            // On Exit State
            .add_system(remove_score.in_schedule(OnExit(AppState::GameOver)))
            .add_system(remove_lives.in_schedule(OnEnter(AppState::GameOver)))
                    .add_system(remove_stage.in_schedule(OnEnter(AppState::GameOver)));

    }
}
