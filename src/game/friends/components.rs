use bevy::prelude::*;
use crate::game::player::{components::Player, states::PlayerOrientationState};


#[derive(PartialEq)]
pub enum FriendType {
    Mouse,
    Cat,
    Dog,
    Cow,
    Flower,
    Bee,
    Butterfly,
    Spider,
    Beaver,
    Fish,
    Bear,
    Tree,
    Donkey,
    Worm,
    Sheep,
    Player,
    None,
}

pub enum ItemType {
    Honey,
    Axe,
    Apple,
    Water,
    Milk,
    Rod,
    Hoe,
    BugNet,
    Bone,
    CatItem,
    Worm,
    BeeNest,
    DogItem,
    Cheese,
    Wood,
    None,
}

pub enum AnimationType {
    Idle,
    Running,
    OneTime,
}

#[derive(Component)]
pub struct Health {
    pub value: f32
}

#[derive(Component)]
pub struct Damage {
    pub value: f32
}

#[derive(Component)]
pub struct Friend {
    pub kind: FriendType,
    pub targeting_friend: FriendType,
    pub targeting_item: ItemType,
    pub current_animation: AnimationType,
    pub last_position_x: f32,
    pub last_position_y: f32,
    pub speed: f32
}

#[derive(Component)]
pub struct Item {
    pub kind: ItemType,
    pub targeting_friend: FriendType,
    pub current_animation: AnimationType,
    pub direction: PlayerOrientationState,
}

#[derive(Default, Component)]
pub struct SpawnTimeStamp {
    pub value: f64,
}

#[derive(Component)]
pub struct AnimationIndicesIdle {
    pub first: usize,
    pub second: usize,
}

#[derive(Component)]
pub struct AnimationIndicesRunning {
    pub first: usize,
    pub second: usize,
}

#[derive(Component)]
pub struct AnimationIndicesItem {
    pub icon: usize,
    pub first: usize,
    pub second: usize,
    pub third: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
