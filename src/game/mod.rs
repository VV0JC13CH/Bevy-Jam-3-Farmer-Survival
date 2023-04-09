use bevy::prelude::*;

mod friends;
mod menu;
mod player;
mod score;
pub mod states;
mod systems;
mod terrain;
mod events;

use crate::AppState;
use friends::FriendsPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use states::*;
use systems::*;
use terrain::TerrainPlugin;
use score::ScorePlugin;
use events::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_state::<CurrentMission>()
            .add_state::<PreviousMission>()
            .add_event::<GameOver>()
            // OnEnter Systems
            .add_startup_system(spawn_camera)
            // .add_startup_system(setup_music)
            .add_plugin(PlayerPlugin)
            .add_plugin(FriendsPlugin)
            .add_plugin(TerrainPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(MenuPlugin)
            .add_system(detect_waterflowers)
            .add_system(detect_getworms)
            .add_system(detect_catchmouses)
            .add_system(detect_catchfishes)
            .add_system(detect_feedfishes)
            .add_system(detect_feedthebears)
            .add_system(detect_choptrees)
            .add_system(detect_catchbutterflies)
            .add_system(detect_feeddonkeys)
            .add_system(detect_givebonestodogs)
            .add_system(detect_takesheepsback)
            .add_system(detect_takehoney)
            .add_system(detect_milkthecow)
            .add_state::<CurrentSong>()
            .add_system(setup_music.in_schedule(OnEnter(AppState::Menu)))
            .add_system(toggle_simulation.in_set(OnUpdate(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
            .add_system(music_change.in_schedule(OnExit(AppState::LevelUp)))
            .add_system(music_stop.in_schedule(OnEnter(AppState::GameOver)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
