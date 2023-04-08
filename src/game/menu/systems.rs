use bevy::{
    input::{mouse::MouseWheel, Input},
    math::Vec3,
    prelude::*,
    render::camera::Camera,
};

use super::components::*;

use crate::game::player::PlayerCharacter;
use crate::game::AppState;
use crate::game::SimulationState;

pub fn choose_item() {}
pub fn item_despawn() {}

pub fn intro_spawn(
    audio: Res<Audio>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let camera = camera_query.get_single().unwrap();

    let texture_handle = asset_server.load("sprites/menu_tilemap.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(400.0, 400.0), 17, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { index: 1 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.index),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(camera.translation.x, camera.translation.y, -0.5),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(1.3, TimerMode::Repeating)),
        IntroElement {},
    ));
    let sound_intro = asset_server.load("audio/intro.ogg");
    audio.play(sound_intro);
    simulation_state_next_state.set(SimulationState::Paused);
}
pub fn menu_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    println!("Im menu");
    let camera = camera_query.get_single().unwrap();

    let texture_handle = asset_server.load("sprites/menu_tilemap.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(400.0, 400.0), 17, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { index: 0 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.index),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(camera.translation.x, camera.translation.y, -0.5),
            ..default()
        },
        MenuElement {},
    ));
}
pub fn item_spawn() {}

pub fn play_intro(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,

    mut query: Query<
        (&mut Transform, &mut AnimationTimer, &mut TextureAtlasSprite),
        (With<IntroElement>, Without<Camera>),
    >,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<IntroElement>)>,
) {
    for (mut transform, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index == 16 {
                sprite.index = 15
            } else {
                sprite.index += 1;
            }
        };
        for mut transform_camera in camera_query.iter_mut() {
            transform_camera.translation.x += 1.0;
            transform_camera.translation.y += 1.0;
            transform.translation = transform_camera.translation;
        }
    }
    if keyboard_input.any_just_pressed([KeyCode::Escape, KeyCode::Space, KeyCode::Return])
        && app_state.0 == AppState::Intro
    {
        app_state_next_state.set(AppState::Menu);
        println!("Going to menu from intro.");
    }
}
pub fn choose_character(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut choosen_character: ResMut<NextState<PlayerCharacter>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,

    mut query: Query<&mut TextureAtlasSprite, With<MenuElement>>,
) {
    for mut sprite in &mut query {
        if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::D, KeyCode::A])
        {
            if sprite.index == 1 {
                sprite.index = 0;
                choosen_character.set(PlayerCharacter::Female);
                println!("Female");
            } else {
                sprite.index = 1;
                choosen_character.set(PlayerCharacter::Male);
                println!("Male");
            }
        };
        if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Return])
            && app_state.0 == AppState::Menu
        {
            simulation_next_state.set(SimulationState::Running);
            app_state_next_state.set(AppState::Game);
            println!("Going to game from menu.");
        }
    }
}
pub fn intro_despawn(mut commands: Commands, intro_query: Query<Entity, With<IntroElement>>) {
    for intro_element in intro_query.iter() {
        commands.entity(intro_element).despawn();
    }
}
pub fn menu_despawn(mut commands: Commands, intro_query: Query<Entity, With<MenuElement>>) {
    for menu_element in intro_query.iter() {
        commands.entity(menu_element).despawn();
    }
}
