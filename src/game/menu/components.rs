use bevy::prelude::*;

#[derive(Component)]
pub struct MenuElement {}


#[derive(Component)]
pub struct LevelUpElement {}

#[derive(Component)]
pub struct IntroElement {}

#[derive(Component)]
pub struct AnimationIndices {
    pub index: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
