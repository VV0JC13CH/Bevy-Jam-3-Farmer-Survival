use bevy::prelude::*;

#[derive(Component)]
pub struct MenuElement {}


#[derive(Component)]
pub struct LevelUpElement {
pub _id : usize
}

#[derive(Component)]
pub struct IntroElement {}

#[derive(Component)]
pub struct AnimationIndices {
    pub index: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
