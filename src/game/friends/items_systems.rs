use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::components::*;
use crate::game::player::components::Player;

use crate::game::player::states::PlayerOrientationState;
const MAX_TYPES_OF_ITEMS: usize = 13;

pub fn action_water_can(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 0;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Axe,
                targeting_friend: FriendType::Flower,
                current_animation: AnimationType::OneTime,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_hoe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 1;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Hoe,
                targeting_friend: FriendType::Worm,
                current_animation: AnimationType::OneTime,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_axe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 2;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::CatItem,
                targeting_friend: FriendType::Mouse,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_cat_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 3;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::CatItem,
                targeting_friend: FriendType::Mouse,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}


pub fn action_honey(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 4;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Honey,
                targeting_friend: FriendType::Bear,
                current_animation: AnimationType::OneTime,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_milk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 5;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Milk,
                targeting_friend: FriendType::Cow,
                current_animation: AnimationType::OneTime,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_bug_net(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 6;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::BugNet,
                targeting_friend: FriendType::Butterfly,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_rod(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 7;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Rod,
                targeting_friend: FriendType::Fish,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_apple(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 8;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Apple,
                targeting_friend: FriendType::Donkey,
                current_animation: AnimationType::OneTime,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_bone(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 9;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Bone,
                targeting_friend: FriendType::Dog,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_dog_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 10;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::DogItem,
                targeting_friend: FriendType::Cat,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}

pub fn action_worms_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 11;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Worm,
                targeting_friend: FriendType::Bear,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}
pub fn action_bee_nest_item(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
    player_orientation: Res<State<PlayerOrientationState>>,
) {
    let index_of_item = 12;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
    for timer in spawn_timers.iter() {
        if (current_time - timer.value) > 1.0 {
            spawn_item = true
        } else {
            spawn_item = false
        }
    }
    if spawn_item {
        let camera = camera_query.get_single().unwrap();
        let texture_handle = asset_server.load("sprites/items_tilemap.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(128.0, 128.0),
            MAX_TYPES_OF_ITEMS,
            4,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices_item = AnimationIndicesItem {
            icon: index_of_item,
            first: MAX_TYPES_OF_ITEMS + index_of_item,
            second: MAX_TYPES_OF_ITEMS * 2 + index_of_item,
            third: MAX_TYPES_OF_ITEMS * 3 + index_of_item,
        };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, 0.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::BeeNest,
                targeting_friend: FriendType::Bear,
                current_animation: AnimationType::OneTime,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
            },
        ));
    }
}



pub fn items_animate(
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera>, Without<Item>)>,
    player_orientation: Res<State<PlayerOrientationState>>,

    mut query: Query<
        (
            &mut Transform,
            &AnimationIndicesItem,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Item,
        ),
        (With<Item>, Without<Camera>),
    >,
) {
    let camera = camera_query.get_single().unwrap();

    for (mut transform, indices_item, mut timer, mut sprite, item) in &mut query {
        timer.tick(time.delta());
        if camera.translation.x > transform.translation.x {
            sprite.flip_x = false
        } else {
            sprite.flip_x = true
        }
        let dir = match item.direction {
            PlayerOrientationState::Right => 1.0,
            PlayerOrientationState::Left => -1.0,
        };
        match item.kind {
            ItemType::Axe => transform.translation.x += 10.0 * dir,
            _ => transform.translation.x += 10.0 * dir,
        }

        if timer.just_finished() {
            match item.current_animation {
                AnimationType::Idle => sprite.index = indices_item.first,
                AnimationType::Running => {
                    //CatItem and DogItem
                    sprite.index = if sprite.index == indices_item.second {
                        indices_item.first
                    } else {
                        indices_item.second
                    };
                }
                AnimationType::OneTime => {
                    sprite.index = if sprite.index == indices_item.first {
                        indices_item.second
                    } else if sprite.index == indices_item.second {
                        indices_item.third
                    } else {
                        indices_item.third
                    }
                }
            }
        };
    }
}
