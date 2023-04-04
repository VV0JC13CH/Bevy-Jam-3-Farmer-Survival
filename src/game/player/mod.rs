mod components;
mod states;
mod systems;

use states::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<PlayerCharacter>()
            .add_state::<PlayerAnimationState>()
            .add_state::<PlayerOrientationState>()
            // Configure System Sets
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            .add_system(player_spawn.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    player_animate.in_set(MovementSystemSet),
                    move_camera.in_set(MovementSystemSet),
                    // confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(player_despawn.in_schedule(OnExit(AppState::Game)));
    }
}