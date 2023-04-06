use bevy::prelude::*;

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
    Trees,
    Donkey,
    Worm,
    Sheep,
}

pub enum AnimationType {
Idle,
    Running
}

#[derive(Component)]
pub struct Friend {
    pub kind : FriendType,
    pub current_animation : AnimationType,
    pub last_position_x : f32,
    pub last_position_y : f32,
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

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

