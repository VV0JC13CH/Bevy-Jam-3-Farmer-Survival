mod components;
mod friends_states;
mod friends_systems;
mod items_states;
mod items_systems;

use friends_states::*;
use friends_systems::*;
use items_states::*;
use items_systems::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FriendsMovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct FriendsSpawnSystemSet;

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
            .add_state::<UnlockedWaterCan>()
            .add_state::<UnlockedHoe>()
            .add_state::<UnlockedAxe>()
            .add_state::<UnlockedCatItem>()
            .add_state::<UnlockedHoney>()
            .add_state::<UnlockedMilk>()
            .add_state::<UnlockedBugNet>()
            .add_state::<UnlockedRod>()
            .add_state::<UnlockedApple>()
            .add_state::<UnlockedBone>()
            .add_state::<UnlockedDogItem>()
            // Configure System Sets
            .configure_set(FriendsSpawnSystemSet.before(FriendsMovementSystemSet))
            // On Enter State
            .add_system(adjust_states_to_character.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    mouse_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedMouse::Spawn)),
                    cat_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedCat::Spawn)),
                    dog_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedDog::Spawn)),
                    cow_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedCow::Spawn)),
                    flower_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedFlower::Spawn)),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    bee_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedBee::Spawn)),
                    butterfly_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedButterfly::Spawn)),
                    spider_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedSpider::Spawn)),
                    beaver_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedBeaver::Spawn)),
                    fish_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedFish::Spawn)),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    bear_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedBear::Spawn)),
                    tree_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedTree::Spawn)),
                    donkey_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedDonkey::Spawn)),
                    worm_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedWorm::Spawn)),
                    sheep_spawn
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedSheep::Spawn)),
                    friends_movement.in_set(FriendsMovementSystemSet),
                    friends_animate.in_set(FriendsMovementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    action_water_can
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedWaterCan::Enabled)),
                    action_hoe
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedHoe::Enabled)),
                    action_axe
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedAxe::Enabled)),
                    action_honey
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedHoney::Enabled)),
                    friends_movement.in_set(FriendsMovementSystemSet),
                    friends_animate.in_set(FriendsMovementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    action_milk
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedMilk::Enabled)),
                    action_bug_net
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedBugNet::Enabled)),
                    action_rod
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedRod::Enabled)),
                    action_apple
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedApple::Enabled)),
                    action_bone
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedBone::Enabled)),
                    friends_movement.in_set(FriendsMovementSystemSet),
                    friends_animate.in_set(FriendsMovementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    action_dog_item
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedMilk::Enabled)),
                    action_cat_item
                        .in_set(FriendsSpawnSystemSet)
                        .in_set(OnUpdate(UnlockedBugNet::Enabled)),
                    friends_movement.in_set(FriendsMovementSystemSet),
                    friends_animate.in_set(FriendsMovementSystemSet),
                    items_animate
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(friends_despawn.in_schedule(OnExit(AppState::Game)))
            .add_system(reset_states.in_schedule(OnExit(AppState::Game)));
    }
}
