// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod components;
mod states;
mod systems;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

// internal
use components::*;
use states::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Farmer Survival".into(),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_state::<DebugState>()
        .add_startup_system(spawn_camera)
        .add_system(change_state)
        .add_system(change_debug)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(debug_show.in_schedule(OnEnter(DebugState::Develop)))
        .add_system(debug_run.in_set(OnUpdate(DebugState::Develop)))
        .add_system(debug_hide.in_schedule(OnExit(DebugState::Develop)))
        .add_system(exit_game)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });
}
