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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Mouse,
                targeting_friend: FriendType::Player,
                targeting_item: ItemType::Cheese,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 75.0,
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Cat,
                targeting_friend: FriendType::Mouse,
                targeting_item: ItemType::Milk,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 55.0,
            },
        ));
    }
}

pub fn dog_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 2;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Dog,
                targeting_friend: FriendType::Cat,
                targeting_item: ItemType::Bone,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 65.0,
            },
        ));
    }
}

pub fn cow_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 3;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Cow,
                targeting_friend: FriendType::Flower,
                targeting_item: ItemType::Milk,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 45.0,
            },
        ));
    }
}

pub fn flower_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 4;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Flower,
                targeting_friend: FriendType::None,
                targeting_item: ItemType::None,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 0.0,
            },
        ));
    }
}

pub fn bee_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 5;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Bee,
                targeting_friend: FriendType::Flower,
                targeting_item: ItemType::None,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 55.0,
            },
        ));
    }
}

pub fn butterfly_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 6;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Butterfly,
                targeting_friend: FriendType::Flower,
                targeting_item: ItemType::None,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 35.0,
            },
        ));
    }
}
pub fn spider_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 7;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Spider,
                targeting_friend: FriendType::Butterfly,
                targeting_item: ItemType::None,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 45.0,
            },
        ));
    }
}
pub fn beaver_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 8;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Beaver,
                targeting_friend: FriendType::Player,
                targeting_item: ItemType::Wood,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 65.0,
            },
        ));
    }
}

pub fn fish_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 9;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Fish,
                targeting_friend: FriendType::Player,
                targeting_item: ItemType::Water,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 85.0,
            },
        ));
    }
}
pub fn bear_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 10;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Bear,
                targeting_friend: FriendType::Player,
                targeting_item: ItemType::Honey,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 45.0,
            },
        ));
    }
}
pub fn tree_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 11;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Tree,
                targeting_friend: FriendType::None,
                targeting_item: ItemType::None,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 0.0,
            },
        ));
    }
}
pub fn donkey_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 12;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Donkey,
                targeting_friend: FriendType::None,
                targeting_item: ItemType::Apple,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 35.0,
            },
        ));
    }
}
pub fn worm_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 13;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Worm,
                targeting_friend: FriendType::None,
                targeting_item: ItemType::Apple,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 100.0,
            },
        ));
    }
}
pub fn sheep_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Friend>>,
) {
    let index_of_friend = 14;
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
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Health { value: 1.0 },
            Friend {
                kind: FriendType::Sheep,
                targeting_friend: FriendType::None,
                targeting_item: ItemType::DogItem,
                current_animation: AnimationType::Idle,
                last_position_x: camera.translation.x + rand_x,
                last_position_y: camera.translation.y + rand_y,
                speed: 45.0,
            },
        ));
    }
}
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
                _ => {
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

pub fn friends_target_target(
    time: Res<Time>,

    camera_query: Query<&Transform, (With<Camera>, Without<Friend>)>,
    mut friend_query: Query<
        (&mut Transform, &Friend),
        (With<Friend>, Without<Camera>, Without<Item>),
    >,
) {
    let camera = camera_query.get_single().unwrap();

    for (mut transform, friend) in &mut friend_query {
        let mut direction = Vec3::ZERO;
        if transform.translation.x > camera.translation.x {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        } else if transform.translation.x < camera.translation.x {
            direction += Vec3::new(1.0, 0.0, 0.0)
        } else {
            direction += Vec3::new(0.0, 0.0, 0.0)
        }
        if transform.translation.y < camera.translation.y {
            direction += Vec3::new(0.0, 1.0, 0.0);
        } else if transform.translation.y > camera.translation.y {
            direction += Vec3::new(0.0, -1.0, 0.0);
        } else {
            direction += Vec3::new(0.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * friend.speed * time.delta_seconds();
    }
}
