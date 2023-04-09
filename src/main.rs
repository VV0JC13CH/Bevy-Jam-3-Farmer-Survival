// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod components;
mod game;
mod states;
mod systems;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

// internal
use components::*;
use states::*;
use systems::*;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Farmer Survival".into(),
                     //   resolution: (800., 600.).into(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), //prevents blurry sprites
        )
        .add_state::<AppState>()
        .add_state::<DebugState>()
        .insert_resource(ClearColor(Color::rgba(0.29, 0.6, 0.32, 1.0)))
        .add_system(change_state)
        .add_system(change_debug)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(debug_show.in_schedule(OnEnter(DebugState::Develop)))
        .add_system(debug_run.in_set(OnUpdate(DebugState::Develop)))
        .add_system(debug_hide.in_schedule(OnExit(DebugState::Develop)))
        .add_plugin(GamePlugin)
        .add_system(exit_game)
        .run();
}
