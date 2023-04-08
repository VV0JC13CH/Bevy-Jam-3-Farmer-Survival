use bevy::prelude::*;

mod friends;
mod menu;
mod player;
mod systems;
mod terrain;

use crate::AppState;
use friends::FriendsPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use systems::*;
use terrain::TerrainPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // OnEnter Systems
            .add_startup_system(spawn_camera)
            .add_plugin(PlayerPlugin)
            .add_plugin(FriendsPlugin)
            .add_plugin(TerrainPlugin)
            .add_plugin(MenuPlugin)
            .add_system(toggle_simulation.in_set(OnUpdate(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
