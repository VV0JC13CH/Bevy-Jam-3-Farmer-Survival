use bevy::{
    input::{mouse::MouseWheel, Input},
    math::Vec3,
    prelude::*,
    render::camera::Camera,
};

use super::states::PlayerAnimationState;
use super::states::PlayerCharacter;
use super::states::PlayerOrientationState;

use super::components::*;

pub fn intro_spawn(){}
pub fn menu_spawn(){}
pub fn item_spawn(){}
pub fn play_intro(){}
pub fn choose_character(){}
pub fn choose_item(){}
pub fn intro_despawn(){}
pub fn menu_despawn(){}
pub fn item_despawn(){}
