mod components;
mod states;
mod systems;

use states::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<ItemChoice>()
            // Configure System Sets
            // On Enter State
            .add_system(intro_spawn.in_schedule(OnEnter(AppState::Intro)))
            .add_system(score_spawn.in_schedule(OnExit(AppState::Menu)))
            .add_system(menu_spawn.in_schedule(OnEnter(AppState::Menu)))
            .add_system(item_spawn.in_schedule(OnEnter(AppState::LevelUp)))
            .add_system(setup_lives_icons.in_schedule(OnEnter(AppState::Game)))
            .add_system(show_game_icons.in_set(OnUpdate(AppState::Game)))
            .add_system(show_score.in_set(OnUpdate(AppState::Game)))
            .add_systems(
                (
                    play_intro,
                )
                    .in_set(OnUpdate(AppState::Intro))
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            .add_system(
                (
                    choose_character
                )
                    .in_set(OnUpdate(AppState::Menu))
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            .add_system(
                (
                    choose_item
                )
                    .in_set(OnUpdate(AppState::LevelUp))
            )
            // On Exit State
            .add_system(intro_despawn.in_schedule(OnExit(AppState::Intro)))
            .add_system(menu_despawn.in_schedule(OnExit(AppState::Menu)))
            .add_system(item_despawn.in_schedule(OnExit(AppState::LevelUp)));
    }
}
