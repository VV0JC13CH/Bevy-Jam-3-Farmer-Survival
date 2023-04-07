use bevy::app::AppExit;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,

};

use crate::AppState;
use crate::DebugState;

use crate::TextDebug;

pub fn change_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    debug_state: Res<State<DebugState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F9)
        && app_state.0 != AppState::Intro
        && debug_state.0 == DebugState::Develop
    {
        app_state_next_state.set(AppState::Intro);
        println!("State changed: intro");
    }
    if keyboard_input.just_pressed(KeyCode::F10)
        && app_state.0 != AppState::Menu
        && debug_state.0 == DebugState::Develop
    {
        app_state_next_state.set(AppState::Menu);
        println!("State changed: menu");
    }
    if keyboard_input.just_pressed(KeyCode::Escape)
        // When in debug ESC closes game, when in release opens menu
        && app_state.0 != AppState::Menu
        && debug_state.0 == DebugState::Release
    {
        app_state_next_state.set(AppState::Menu);
        println!("State changed: menu");
    }
    if keyboard_input.just_pressed(KeyCode::F11)
        && app_state.0 != AppState::Game
        && debug_state.0 == DebugState::Develop
    {
        app_state_next_state.set(AppState::Game);
        println!("State changed: game");
    }
    if keyboard_input.just_pressed(KeyCode::F12)
        && app_state.0 != AppState::GameOver
        && debug_state.0 == DebugState::Develop
    {
        app_state_next_state.set(AppState::GameOver);
        println!("State changed: gameover");
    }
}

pub fn change_debug(
    keyboard_input: Res<Input<KeyCode>>,
    debug_state: Res<State<DebugState>>,
    mut debug_state_next_state: ResMut<NextState<DebugState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) && debug_state.0 != DebugState::Develop {
        debug_state_next_state.set(DebugState::Develop);

        println!("Debug level: develop");
    }
    if keyboard_input.just_pressed(KeyCode::F1) && debug_state.0 != DebugState::Release {
        debug_state_next_state.set(DebugState::Release);
        println!("Debug level: release");
    }
}

pub fn exit_game(
    // When in debug ESC closes game, when in release opens menu
    keyboard_input: Res<Input<KeyCode>>,
    debug_state: Res<State<DebugState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && debug_state.0 == DebugState::Develop {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn debug_show(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/EduNSWACTFoundation-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "BOTTOM RIGHT DEBUG:",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "\n",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::RED,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::ORANGE_RED,
            }),
            TextSection::new(
                " fps, ",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color: Color::GREEN,
            }),
            TextSection::new(
                " ms/frame",
                TextStyle {
                    font: font,
                    font_size: 30.0,
                    color: Color::BLUE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        TextDebug,
    ));
}

pub fn debug_run(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextDebug>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[0].value = format!("{fps:.1} fps, {frame_time:.3} ms/frame",);

        text.sections[2].value = format!("{fps:.1}");

        text.sections[4].value = format!("{frame_time:.3}");
    }
}

pub fn debug_hide(mut commands: Commands, mut query: Query<Entity, With<TextDebug>>) {
    for text in &mut query {
        commands.entity(text).despawn();
    }
}
