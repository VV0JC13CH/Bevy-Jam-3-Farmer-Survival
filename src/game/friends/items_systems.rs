use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::components::*;
use crate::game::player::components::Player;

const MAX_TYPES_OF_ITEMS: usize = 13;

pub fn action_water_can() {}
pub fn action_hoe() {}
pub fn action_axe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: ResMut<Time>,
    camera_query: Query<&Transform, With<Camera>>,
    spawn_timers: Query<&SpawnTimeStamp, With<Item>>,
) {
    let index_of_item = 2;
    let mut spawn_item: bool = true;
    let current_time = time.elapsed_seconds_f64();
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
                sprite: TextureAtlasSprite::new(animation_indices_item.icon),
                // transform: Transform::from_scale(Vec3::splat(1.0)),
                transform: Transform::from_xyz(camera.translation.x, camera.translation.y, -50.0),
                ..default()
            },
            animation_indices_item,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            SpawnTimeStamp {
                value: current_time,
            },
            Item {
                kind: ItemType::Axe,
                targeting_friend: FriendType::Tree,
                current_animation: AnimationType::Idle,
                size_x: camera.translation.x,
                sizy_y: camera.translation.y,
            },
        ));
    }
}
pub fn action_honey() {}
pub fn action_milk() {}
pub fn action_bug_net() {}
pub fn action_rod() {}
pub fn action_apple() {}
pub fn action_bone() {}
pub fn action_dog_item() {}
pub fn action_cat_item() {}

pub fn items_animate(
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera>, Without<Item>)>,
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
        // Friend behind or in front of character
        if camera.translation.y - 45.0 < transform.translation.y {
            transform.translation.z = -1.0;
        } else {
            transform.translation.z = 1.0;
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
