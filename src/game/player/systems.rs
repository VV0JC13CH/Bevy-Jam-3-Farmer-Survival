use std::thread::current;

use bevy::{
    input::{mouse::MouseWheel, Input},
    math::Vec3,
    prelude::*,
    render::camera::Camera,
};

use super::components::*;
use super::states::PlayerAnimationState;
use super::states::PlayerCharacter;
use super::states::PlayerOrientationState;
use crate::game::menu::components::GameElement;
use crate::game::score::resources::Score;
use crate::game::states::{CurrentMission, PreviousMission};

pub const PLAYER_SPEED: f32 = 300.0;

pub fn player_spawn(
    mut commands: Commands,
    player_char: Res<State<PlayerCharacter>>,
    debug_state: Res<State<DebugState>>,
    mut next_mission: ResMut<NextState<CurrentMission>>,
    mut previous_mission: ResMut<NextState<PreviousMission>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera_query: Query<&Transform, With<Camera>>,
    score: Res<Score>,
) {
    let camera = camera_query.get_single().unwrap();

    let texture_handle = match player_char.0 {
        PlayerCharacter::Female => asset_server.load("sprites/farmer_female.png"),
        PlayerCharacter::Male => asset_server.load("sprites/farmer_male.png"),
    };
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 128.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 1 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player { level: 1 },
    ));

    if player_char.0 == PlayerCharacter::Male {
        println!("Starting game as a boy.");
        if score.value == 0 && debug_state.0 != DebugState::Develop {
            next_mission.set(CurrentMission::WaterFlowers);
            previous_mission.set(PreviousMission::NewGame);
        }
    }
    if player_char.0 == PlayerCharacter::Female {
        println!("Starting game as a girl.");
        if score.value == 0 && debug_state.0 != DebugState::Develop {
            next_mission.set(CurrentMission::GetWorms);
            previous_mission.set(PreviousMission::NewGame);
        }
    }
}

pub fn player_despawn(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    for camera in camera_query.iter() {
        if let Ok(player_entity) = player_query.get_single() {
            commands.entity(camera).remove_children(&[player_entity]);
            commands.entity(player_entity).despawn();
        }
    }
}

pub fn player_animate(
    player_animation: Res<State<PlayerAnimationState>>,
    player_orientation: Res<State<PlayerOrientationState>>,
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if player_animation.0 == PlayerAnimationState::Running {
        for (indices, mut timer, mut sprite) in &mut query {
            timer.tick(time.delta());
            if timer.just_finished() {
                if player_orientation.0 == PlayerOrientationState::Right {
                    sprite.flip_x = false
                } else {
                    sprite.flip_x = true
                }
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            };
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    player_animation: Res<State<PlayerAnimationState>>,
    mut player_animation_next_state: ResMut<NextState<PlayerAnimationState>>,
    player_orientation: Res<State<PlayerOrientationState>>,
    mut player_orientation_next_state: ResMut<NextState<PlayerOrientationState>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            if player_orientation.0 == PlayerOrientationState::Right {
                player_orientation_next_state.set(PlayerOrientationState::Left);
            }
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            if player_orientation.0 == PlayerOrientationState::Left {
                player_orientation_next_state.set(PlayerOrientationState::Right);
            }
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        if player_animation.0 == PlayerAnimationState::Idle && direction.length() > 0.0 {
            player_animation_next_state.set(PlayerAnimationState::Running);
            // println!("Player is moving!");
        } else if player_animation.0 == PlayerAnimationState::Running && direction.length() == 0.0 {
            player_animation_next_state.set(PlayerAnimationState::Idle);
            // println!("Player is idle!");
        }
    }
}

pub fn link_camera_with_player(
    mut commands: Commands,
    mut player_query: Query<Entity, With<Player>>,
    mut camera_query: Query<Entity, With<Camera>>,
) {
    for cam_entity in camera_query.iter_mut() {
        for player_entity in player_query.iter_mut() {
            commands.entity(cam_entity).push_children(&[player_entity]);
        }
    }
}

pub fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in camera_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        for event in scroll_evr.iter() {
            if event.y < 0.0 {
                ortho.scale += 0.2
            }
            if event.y > 0.0 {
                ortho.scale -= 0.2
            }
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

use crate::DebugState;

use crate::AppState;

pub fn level_up(
    mut player_query: Query<&mut Player, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    debug_state: Res<State<DebugState>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3)
        && app_state.0 == AppState::Game
        && debug_state.0 == DebugState::Develop
    {
        app_state_next_state.set(AppState::LevelUp);
        println!("State changed: LevelUp");
    }
}
