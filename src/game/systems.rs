use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::AppState;
use crate::game::SimulationState;
use crate::DebugState;

use super::states::{CurrentMission, PreviousMission};
use crate::game::friends::friends_states::*;

use crate::game::friends::items_states::*;
use crate::game::score::resources::Score;
use crate::game::score::resources::Stage;
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

pub fn levelup(score: Res<Score>, mut app_next_state: ResMut<NextState<AppState>>,
    mut stage: ResMut<Stage>,


    ) {
    if score.is_changed() {
        if score.value <= 12 {
            if (score.value % 3) == 0 && score.value != 0 {
                app_next_state.set(AppState::LevelUp);
                stage.value += 1;
            };
        } else {
            if (score.value % 6) == 0 && score.value != 0 {
                app_next_state.set(AppState::LevelUp);
                stage.value += 1;

            };
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

    previous_mission: Res<State<PreviousMission>>,
    mut default_tool_next_state: ResMut<NextState<UnlockedRod>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedFish>>,
    mut extra_friend_next_state: ResMut<NextState<UnlockedCat>>,
) {
    if current_mission.0 == CurrentMission::CatchFishes {
        default_tool_next_state.set(UnlockedRod::Enabled);
        default_friend_next_state.set(UnlockedFish::Spawn);
    } else {
        default_tool_next_state.set(UnlockedRod::Blocked);
    }
    if previous_mission.0 == PreviousMission::CatchFishes {
        extra_friend_next_state.set(UnlockedCat::Spawn)
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
    previous_mission: Res<State<PreviousMission>>,
    mut default_tool_next_state: ResMut<NextState<UnlockedAxe>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedTree>>,
    mut extra_friend_next_state: ResMut<NextState<UnlockedBeaver>>,
) {
    if current_mission.0 == CurrentMission::ChopTrees {
        default_tool_next_state.set(UnlockedAxe::Enabled);
        default_friend_next_state.set(UnlockedTree::Spawn);
    } else {
        default_tool_next_state.set(UnlockedAxe::Blocked);
    }
    if previous_mission.0 == PreviousMission::ChopTrees {
        extra_friend_next_state.set(UnlockedBeaver::Spawn)
    }
}
pub fn detect_catchbutterflies(
    current_mission: Res<State<CurrentMission>>,
    previous_mission: Res<State<PreviousMission>>,
    mut default_tool_next_state: ResMut<NextState<UnlockedBugNet>>,
    mut default_friend_1_next_state: ResMut<NextState<UnlockedButterfly>>,
    mut default_friend_2_next_state: ResMut<NextState<UnlockedBee>>,
    mut extra_friend_next_state: ResMut<NextState<UnlockedSpider>>,
) {
    if current_mission.0 == CurrentMission::CatchButterflies {
        default_tool_next_state.set(UnlockedBugNet::Enabled);
        default_friend_1_next_state.set(UnlockedButterfly::Spawn);
        default_friend_2_next_state.set(UnlockedBee::Spawn);
    } else {
        default_tool_next_state.set(UnlockedBugNet::Blocked);
    }
    if previous_mission.0 == PreviousMission::CatchButterflies {
        extra_friend_next_state.set(UnlockedSpider::Spawn)
    }
}
pub fn detect_feeddonkeys(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedApple>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedDonkey>>,
) {
    if current_mission.0 == CurrentMission::FeedDonkeys {
        default_tool_next_state.set(UnlockedApple::Enabled);
        default_friend_next_state.set(UnlockedDonkey::Spawn);
    } else {
        default_tool_next_state.set(UnlockedApple::Blocked);
    }
}
pub fn detect_givebonestodogs(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedBone>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedDog>>,
) {
    if current_mission.0 == CurrentMission::GiveBonesToDogs {
        default_tool_next_state.set(UnlockedBone::Enabled);
        default_friend_next_state.set(UnlockedDog::Spawn);
    } else {
        default_tool_next_state.set(UnlockedBone::Blocked);
    }
}
pub fn detect_takesheepsback(
    current_mission: Res<State<CurrentMission>>,

    mut default_tool_next_state: ResMut<NextState<UnlockedDogItem>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedSheep>>,
) {
    if current_mission.0 == CurrentMission::TakeSheepsBack {
        default_tool_next_state.set(UnlockedDogItem::Enabled);
        default_friend_next_state.set(UnlockedSheep::Spawn);
    } else {
        default_tool_next_state.set(UnlockedDogItem::Blocked);
    }
}
pub fn detect_takehoney(
    current_mission: Res<State<CurrentMission>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedBeeBox>>,
    mut extra_friend_next_state: ResMut<NextState<UnlockedBee>>,
) {
    if current_mission.0 == CurrentMission::TakeHoney {
        default_friend_next_state.set(UnlockedBeeBox::Spawn);
        extra_friend_next_state.set(UnlockedBee::Spawn);
    } else {
    }
}
pub fn detect_milkthecow(
    current_mission: Res<State<CurrentMission>>,
    mut default_tool_next_state: ResMut<NextState<UnlockedMilk>>,
    mut default_friend_next_state: ResMut<NextState<UnlockedCow>>,
) {
    if current_mission.0 == CurrentMission::MilkTheCow {
        default_tool_next_state.set(UnlockedMilk::Enabled);
        default_friend_next_state.set(UnlockedCow::Spawn);
    } else {
        default_tool_next_state.set(UnlockedMilk::Blocked);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CurrentSong {
    #[default]
    TrackZero,
    TrackOne,
    TrackTwo,
    TrackThree,
    TrackFour,
    TrackFive,
    TrackSix,
}
#[derive(Resource)]
pub struct BeautifulMusic(Handle<AudioSink>);

pub fn setup_music(
    asset_server: Res<AssetServer>,
    mut audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    mut commands: Commands,
    current_track: Res<State<CurrentSong>>,
) {
    if current_track.0 == CurrentSong::TrackZero {
        let music = asset_server.load("audio/music_tier1.ogg");
        // play audio and upgrade to a strong handle
        let sink_handle = audio_sinks
            .get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.75)));

        commands.insert_resource(BeautifulMusic(sink_handle));
    }
}

pub fn music_stop(
    music: Res<BeautifulMusic>,
    mut audio_sinks: ResMut<Assets<AudioSink>>,
    mut commands: Commands,
) {
    if let Some(sink) = audio_sinks.get(&music.0) {
        commands.remove_resource::<BeautifulMusic>();
    }
}
pub fn music_change(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut commands: Commands,
    music: Res<BeautifulMusic>,
    audio_sinks: ResMut<Assets<AudioSink>>,
    current_track: Res<State<CurrentSong>>,
    mut next_track: ResMut<NextState<CurrentSong>>,
) {
    if let Some(sink) = audio_sinks.get(&music.0) {
        // pause playback
        sink.pause();
    }

    let next_song = match current_track.0 {
        CurrentSong::TrackZero => CurrentSong::TrackOne,
        CurrentSong::TrackOne => CurrentSong::TrackTwo,
        CurrentSong::TrackTwo => CurrentSong::TrackThree,
        CurrentSong::TrackThree => CurrentSong::TrackFour,
        CurrentSong::TrackFour => CurrentSong::TrackFive,
        CurrentSong::TrackFive => CurrentSong::TrackSix,
        _ => CurrentSong::TrackSix,
    };
    next_track.set(next_song);
    let track = match current_track.0 {
        CurrentSong::TrackZero => "audio/music_tier1.ogg".to_string(),
        CurrentSong::TrackOne => "audio/music_tier1.ogg".to_string(),
        CurrentSong::TrackTwo => "audio/music_tier2.ogg".to_string(),
        CurrentSong::TrackThree => "audio/music_tier3.ogg".to_string(),
        CurrentSong::TrackFour => "audio/music_tier4.ogg".to_string(),
        CurrentSong::TrackFive => "audio/music_tier5.ogg".to_string(),
        CurrentSong::TrackSix => "audio/music_tier6.ogg".to_string(),
    };
    let music = asset_server.load(track);

    //    let music = asset_server.load(song.to_string());
    // play audio and upgrade to a strong handle
    let sink_handle = audio_sinks
        .get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.75)));

    commands.insert_resource(BeautifulMusic(sink_handle));
}
