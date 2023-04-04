use bevy::prelude::*;

mod player;
mod systems;

use player::PlayerPlugin;
use systems::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgba(0.29, 0.6, 0.32, 1.0)))
            .add_state::<SimulationState>()
            // OnEnter Systems
            .add_startup_system(spawn_camera)
            .add_plugin(PlayerPlugin)
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // My Plugins
            // Systems
            .add_system(toggle_simulation.in_set(OnUpdate(AppState::Game)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
