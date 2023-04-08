use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::SimulationState;
use crate::DebugState;

use super::states::CurrentMission;
use crate::game::friends::friends_states::*;

use crate::game::friends::items_states::*;
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
        ..default()
    });
}

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    debug_state: Res<State<DebugState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) && debug_state.0 == DebugState::Develop {
        if simulation_state.0 == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if simulation_state.0 == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
            println!("Simulation Running.");
        }
    }
}

pub fn detect_waterflowers(
    current_mission: Res<State<CurrentMission>>,
    mut default_tool_next_state: ResMut<NextState<UnlockedWaterCan>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFlower>>,
) {
    if current_mission.0 == CurrentMission::WaterFlowers {
        default_tool_next_state.set(UnlockedWaterCan::Enabled);
        default_friend_next_state.set(UnlockedFlower::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWaterCan::Blocked);
    }
}
pub fn detect_getworms(
    current_mission: Res<State<CurrentMission>>,
    mut default_tool_next_state: ResMut<NextState<UnlockedHoe>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedWorm>>,
) {
    if current_mission.0 == CurrentMission::GetWorms {
        default_tool_next_state.set(UnlockedHoe::Enabled);
        default_friend_next_state.set(UnlockedWorm::Spawn);
    } else {
        default_tool_next_state.set(UnlockedHoe::Blocked);
    }
}

pub fn detect_catchmouses(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedCatItem>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedMouse>>,
) {
    if current_mission.0 == CurrentMission::CatchMouses {
        default_tool_next_state.set(UnlockedCatItem::Enabled);
        default_friend_next_state.set(UnlockedMouse::Spawn);
    } else {
        default_tool_next_state.set(UnlockedCatItem::Blocked);
    }
}
pub fn detect_catchfishes(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedRod>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFish>>,
) {
    if current_mission.0 == CurrentMission::CatchFishes {
        default_tool_next_state.set(UnlockedRod::Enabled);
        default_friend_next_state.set(UnlockedFish::Spawn);
    } else {
        default_tool_next_state.set(UnlockedRod::Blocked);
    }
}
pub fn detect_feedfishes(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedWormItem>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFish>>,
) {
    if current_mission.0 == CurrentMission::FeedFishes {
        default_tool_next_state.set(UnlockedWormItem::Enabled);
        default_friend_next_state.set(UnlockedFish::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWormItem::Blocked);
    }
}
pub fn detect_feedthebears(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedHoney>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedBear>>,
) {
    if current_mission.0 == CurrentMission::FeedTheBears {
        default_tool_next_state.set(UnlockedHoney::Enabled);
        default_friend_next_state.set(UnlockedBear::Spawn);
    } else {
        default_tool_next_state.set(UnlockedHoney::Blocked);
    }
}
pub fn detect_choptrees(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedAxe>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedTree>>,
) {
    if current_mission.0 == CurrentMission::ChopTrees {
        default_tool_next_state.set(UnlockedAxe::Enabled);
        default_friend_next_state.set(UnlockedTree::Spawn);
    } else {
        default_tool_next_state.set(UnlockedAxe::Blocked);
    }
}
pub fn detect_catchbutterflies(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedWaterCan>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFlower>>,
) {
    if current_mission.0 == CurrentMission::WaterFlowers {
        default_tool_next_state.set(UnlockedWaterCan::Enabled);
        default_friend_next_state.set(UnlockedFlower::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWaterCan::Blocked);
    }
}
pub fn detect_feeddonkeys(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedWaterCan>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFlower>>,
) {
    if current_mission.0 == CurrentMission::WaterFlowers {
        default_tool_next_state.set(UnlockedWaterCan::Enabled);
        default_friend_next_state.set(UnlockedFlower::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWaterCan::Blocked);
    }
}
pub fn detect_givebonestodogs(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedWaterCan>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFlower>>,
) {
    if current_mission.0 == CurrentMission::WaterFlowers {
        default_tool_next_state.set(UnlockedWaterCan::Enabled);
        default_friend_next_state.set(UnlockedFlower::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWaterCan::Blocked);
    }
}
pub fn detect_takesheepsback(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedWaterCan>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFlower>>,
) {
    if current_mission.0 == CurrentMission::WaterFlowers {
        default_tool_next_state.set(UnlockedWaterCan::Enabled);
        default_friend_next_state.set(UnlockedFlower::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWaterCan::Blocked);
    }
}
pub fn detect_takehoney(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedWaterCan>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFlower>>,
) {
    if current_mission.0 == CurrentMission::WaterFlowers {
        default_tool_next_state.set(UnlockedWaterCan::Enabled);
        default_friend_next_state.set(UnlockedFlower::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWaterCan::Blocked);
    }
}
pub fn detect_milkthecow(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedWaterCan>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFlower>>,
) {
    if current_mission.0 == CurrentMission::WaterFlowers {
        default_tool_next_state.set(UnlockedWaterCan::Enabled);
        default_friend_next_state.set(UnlockedFlower::Spawn);
    } else {
        default_tool_next_state.set(UnlockedWaterCan::Blocked);
    }
}
