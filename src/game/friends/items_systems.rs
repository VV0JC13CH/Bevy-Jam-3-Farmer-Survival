use bevy::prelude::*;
use bevy_rapier2d::parry::transformation::utils::transform;
use rand::{thread_rng, Rng};

use super::components::*;
use crate::game::player;
use crate::game::player::components::Player;

use crate::game::player::states::PlayerOrientationState;
use crate::game::score::resources::Score;
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
                transform: Transform::from_xyz(
                    camera.translation.x,
                    camera.translation.y - 32.0,
                    0.0,
                ),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Damage { value: 1.0 },
            Item {
                kind: ItemType::Water,
                targeting_friend: FriendType::Flower,
                current_animation: AnimationType::OneTime,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
        let mut random = thread_rng();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(
                    camera.translation.x + random.gen_range(-400.0..400.0),
                    camera.translation.y + 400.0,
                    0.0,
                ),
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                kind: ItemType::Axe,
                targeting_friend: FriendType::Tree,
                current_animation: AnimationType::Running,
                direction: match player_orientation.0 {
                    PlayerOrientationState::Right => PlayerOrientationState::Right,
                    PlayerOrientationState::Left => PlayerOrientationState::Left,
                },
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
        let mut random = thread_rng();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(
                    camera.translation.x + random.gen_range(-1000.0..-800.0),
                    camera.translation.y + random.gen_range(-600.0..600.0),
                    0.0,
                ),
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
        let dir = match player_orientation.0 {
            PlayerOrientationState::Right => 1.0,
            PlayerOrientationState::Left => -1.0,
        };

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices_item.first),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(
                    camera.translation.x + 10.0 * dir,
                    camera.translation.y,
                    0.0,
                ),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
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
                spawn_position_x: camera.translation.x,
                spawn_position_y: camera.translation.y,
            },
        ));
    }
}

pub fn items_animate(
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera>, Without<Item>)>,
    mut query: Query<
        (
            &mut Transform,
            &AnimationIndicesItem,
            &mut AnimationTimer,
            &SpawnTimeStamp,
            &mut TextureAtlasSprite,
            &Item,
        ),
        (With<Item>, Without<Camera>),
    >,
) {
    let camera = camera_query.get_single().unwrap();

    for (mut transform, indices_item, mut timer, spawn_timer, mut sprite, item) in &mut query {
        timer.tick(time.delta());
        match item.kind {
            ItemType::CatItem => sprite.flip_x = true,
            _ => {
                if camera.translation.x > transform.translation.x {
                    sprite.flip_x = false
                } else {
                    sprite.flip_x = true
                }
            }
        }
        let dir = match item.direction {
            PlayerOrientationState::Right => 1.0,
            PlayerOrientationState::Left => -1.0,
        };
        let mut random = thread_rng();

        match item.kind {
            ItemType::Water => {
                transform.translation.z = -1.0;
            }
            ItemType::Hoe => {
                if transform.translation.y > (camera.translation.y + random.gen_range(-10.0..10.0))
                {
                    // has to be quicker than player
                    let current_time = time.elapsed_seconds_f64();
                    if (current_time - spawn_timer.value) < 1.0 {
                        transform.translation.y -= 10.0
                    } else {
                    }
                } else {
                    transform.translation.y += 0.0
                }
            }
             ItemType::Axe => {
                 transform.translation.x += 5.0 * dir;
                 transform.rotate(Quat::from_rotation_z((-dir * 10.0_f32).to_radians()))
             }
            ItemType::CatItem => {
                transform.translation.x += 15.0;
            }
            ItemType::Honey => match item.direction {
                PlayerOrientationState::Left => {
                    if transform.translation.x - item.spawn_position_x > -60.0 {
                        transform.translation.x -= 5.0;
                        transform.translation.y += 5.0;
                    } else if transform.translation.x - item.spawn_position_x > -120.0 {
                        transform.translation.y -= 5.0;
                    } else {
                    }
                }

                PlayerOrientationState::Right => {
                    transform.translation.x += 5.0;
                    transform.translation.y += 5.0;
                }
            },
            _ => transform.translation.x += 2.0 * dir,
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
pub fn item_hit_friend(
    mut commands: Commands,
    //   mut game_over_event_writer: EventWriter<GameOver>,
    mut friend_query: Query<(Entity, &mut Transform, &Friend), (With<Friend>, Without<Item>)>,
    mut item_query: Query<
        (Entity, &mut Transform, &Item, &SpawnTimeStamp),
        (With<Item>, Without<Friend>),
    >,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds_f64();

    for (friend_entity, mut friend_transform, friend) in friend_query.iter_mut() {
        for (item_entity, item_transform, item, item_spawn_time) in item_query.iter_mut() {
            let distance = friend_transform
                .translation
                .distance(item_transform.translation);
            if distance < 64.0 {
                if friend.kind == FriendType::Tree && item.kind == ItemType::Axe {
                    println!("Tree targeted!");
                    commands.entity(friend_entity).despawn();
                    let sound_effect = asset_server.load("audio/sound_4.ogg");
                    audio.play(sound_effect);
                    score.value += 1;
                }

                if friend.kind == FriendType::Flower && item.kind == ItemType::Water {
                    println!("Flowers like water!");
                    commands.entity(friend_entity).despawn();
                    let sound_effect = asset_server.load("audio/sound_2.ogg");
                    audio.play(sound_effect);
                    score.value += 1;
                }
                if distance < 32.0 && friend.kind == FriendType::Worm && item.kind == ItemType::Hoe
                {
                    if (current_time - item_spawn_time.value) < 0.8 {
                        println!("Worm collected!");
                        commands.entity(friend_entity).despawn();
                        let sound_effect = asset_server.load("audio/sound_3.ogg");
                        audio.play(sound_effect);
                        score.value += 1;
                    }
                } else {
                    println!("Collision without score")
                }
            }
        }
    }
}

pub fn item_outside_of_range(
    mut commands: Commands,
    mut item_query: Query<(Entity, &Transform), With<Item>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Item>)>,
) {
    let camera = camera_query.get_single().unwrap();

    for (item_entity, item_transform) in item_query.iter_mut() {
        let distance = item_transform.translation.distance(camera.translation);
        if distance > 1000.0 {
            println!("Despawn of item!");
            commands.entity(item_entity).despawn();
        }
    }
}
