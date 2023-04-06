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
pub struct TargetingSystemSet;

pub struct FriendsPlugin;

impl Plugin for FriendsPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<UnlockedMouse>()
            .add_state::<UnlockedCat>()
            .add_state::<UnlockedDog>()
            .add_state::<UnlockedCow>()
            .add_state::<UnlockedFlower>()
            .add_state::<UnlockedBee>()
            .add_state::<UnlockedButterfly>()
            .add_state::<UnlockedSpider>()
            .add_state::<UnlockedBeaver>()
            .add_state::<UnlockedFish>()
            .add_state::<UnlockedBear>()
            .add_state::<UnlockedTree>()
            .add_state::<UnlockedDonkey>()
            .add_state::<UnlockedWorm>()
            .add_state::<UnlockedSheep>()
            // Configure System Sets
            .configure_set(MovementSystemSet.before(TargetingSystemSet))
            // On Enter State
            .add_system(adjust_states_to_character.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    friends_spawn.in_set(OnUpdate(AppState::Game)),
                    friends_movement.in_set(MovementSystemSet),
                    friends_animate.in_set(MovementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(friends_despawn.in_schedule(OnExit(AppState::Game)))
            .add_system(reset_states.in_schedule(OnExit(AppState::Game)));
    }
}
