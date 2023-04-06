use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::components::*;
use crate::game::player::components::Player;

const MAX_SPECIES_OF_FRIENDS: usize = 15;

pub fn adjust_states_to_character(
    friend_query: Query<Entity, With<Friend>>,
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera>>,
) {
}
pub fn reset_states(
    friend_query: Query<Entity, With<Friend>>,
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera>>,
) {
}

pub fn mouse_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 0;
    let mut spawn_friend: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 5.0 {
            spawn_friend = true
        } else {
            spawn_friend = false
        }
    }
    if spawn_friend {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/entities_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_SPECIES_OF_FRIENDS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_idle = AnimationIndicesIdle {
            first: index_of_friend,
            second: MAX_SPECIES_OF_FRIENDS + index_of_friend,
        };
        let animation_indices_running = AnimationIndicesRunning {
            first: MAX_SPECIES_OF_FRIENDS * 2 + index_of_friend,
            second: MAX_SPECIES_OF_FRIENDS * 3 + index_of_friend,
        };
        let mut random = thread_rng();
        let rand_x: f32;
        let rand_y: f32;
        const DISTANCE_CLOSE: f32 = 400.0;
        const DISTANCE_LONG: f32 = 1500.0;
        let rand_placement = random.gen_range(0..4);
        if rand_placement == 0 {
            rand_x = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
            rand_y = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
        } else if rand_placement == 1 {
            rand_x = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
            rand_y = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
        } else if rand_placement == 2 {
            rand_x = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
            rand_y = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
        } else {
            rand_x = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
            rand_y = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_idle.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(
                    camera.translation.x + rand_x,
                    camera.translation.y + rand_y,
                    0.0,
                ),
                ..default()
            },
            animation_indices_idle,
            animation_indices_running,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Friend {
                kind: FriendType::Mouse,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
            },
        ));
    }
}

pub fn cat_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 1;
    let mut spawn_friend: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 5.0 {
            spawn_friend = true
        } else {
            spawn_friend = false
        }
    }
    if spawn_friend {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/entities_tilemap.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 15, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_idle = AnimationIndicesIdle {
            first: index_of_friend,
            second: MAX_SPECIES_OF_FRIENDS + index_of_friend,
        };
        let animation_indices_running = AnimationIndicesRunning {
            first: MAX_SPECIES_OF_FRIENDS * 2 + index_of_friend,
            second: MAX_SPECIES_OF_FRIENDS * 3 + index_of_friend,
        };
        let mut random = thread_rng();
        let rand_x: f32;
        let rand_y: f32;
        const DISTANCE_CLOSE: f32 = 400.0;
        const DISTANCE_LONG: f32 = 1500.0;
        let rand_placement = random.gen_range(0..4);
        if rand_placement == 0 {
            rand_x = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
            rand_y = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
        } else if rand_placement == 1 {
            rand_x = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
            rand_y = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
        } else if rand_placement == 2 {
            rand_x = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
            rand_y = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
        } else {
            rand_x = random.gen_range(-DISTANCE_LONG..-DISTANCE_CLOSE);
            rand_y = random.gen_range(DISTANCE_CLOSE..DISTANCE_LONG);
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_idle.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(
                    camera.translation.x + rand_x,
                    camera.translation.y + rand_y,
                    0.0,
                ),
                ..default()
            },
            animation_indices_idle,
            animation_indices_running,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Friend {
                kind: FriendType::Cat,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
            },
        ));
    }
}

pub fn dog_spawn() {}
pub fn cow_spawn() {}
pub fn flower_spawn() {}
pub fn butterfly_spawn() {}
pub fn spider_spawn() {}
pub fn bee_spawn() {}
pub fn beaver_spawn() {}
pub fn fish_spawn() {}
pub fn bear_spawn() {}
pub fn tree_spawn() {}
pub fn donkey_spawn() {}
pub fn worm_spawn() {}
pub fn sheep_spawn() {}
pub fn friends_despawn(
    friend_query: Query<Entity, With<Friend>>,
    camera_query: Query<Entity, With<Camera>>,
) {
}

pub fn friends_movement(mut friend_query: Query<(&mut Friend, &Transform), With<Friend>>) {
    for (mut friend, transform) in &mut friend_query {
        if friend.last_position_x == transform.translation.x
            && friend.last_position_y == transform.translation.y
        {
            friend.current_animation = AnimationType::Idle
        } else {
            friend.current_animation = AnimationType::Running
        }
        friend.last_position_x = transform.translation.x;
        friend.last_position_y = transform.translation.y;
    }
}
pub fn friends_animate(
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera>, Without<Friend>)>,
    mut query: Query<
        (
            &mut Transform,
            &AnimationIndicesIdle,
            &AnimationIndicesRunning,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Friend,
        ),
        (With<Friend>, Without<Camera>),
    >,
) {
    let camera = camera_query.get_single().unwrap();

    for (mut transform, indices_idle, indices_running, mut timer, mut sprite, friend) in &mut query
    {
        timer.tick(time.delta());
        if camera.translation.x > transform.translation.x {
            sprite.flip_x = true
        } else {
            sprite.flip_x = false
        }
        // Friend behind or in front of character
        if camera.translation.y - 45.0 < transform.translation.y {
            transform.translation.z = -1.0;
        } else {
            transform.translation.z = 1.0;
        }

        if timer.just_finished() {
            match friend.current_animation {
                AnimationType::Idle => {
                    sprite.index = if sprite.index == indices_idle.second {
                        indices_idle.first
                    } else {
                        indices_idle.second
                    };
                }
                AnimationType::Running => {
                    sprite.index = if sprite.index == indices_running.second {
                        indices_running.first
                    } else {
                        indices_running.second
                    };
                }
            }
        };
    }
}
