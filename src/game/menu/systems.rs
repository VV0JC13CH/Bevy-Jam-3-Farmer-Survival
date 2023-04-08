use super::components::*;
use super::states::*;
use bevy::{input::Input, prelude::*, render::camera::Camera};

use crate::game::player::PlayerCharacter;
use crate::game::AppState;
use crate::game::SimulationState;

use crate::game::CurrentMission;
use crate::game::PreviousMission;
pub fn intro_spawn(
    audio: Res<Audio>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let camera = camera_query.get_single().unwrap();

    let texture_handle = asset_server.load("sprites/menu_tilemap.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(400.0, 400.0), 17, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { index: 1 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.index),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(camera.translation.x, camera.translation.y, -0.5),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(1.3, TimerMode::Repeating)),
        IntroElement {},
    ));
    let sound_intro = asset_server.load("audio/intro.ogg");
    audio.play(sound_intro);
    simulation_state_next_state.set(SimulationState::Paused);
}
pub fn menu_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    println!("Im menu");
    let camera = camera_query.get_single().unwrap();

    let texture_handle = asset_server.load("sprites/menu_tilemap.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(400.0, 400.0), 17, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { index: 0 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.index),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(camera.translation.x, camera.translation.y, -0.5),
            ..default()
        },
        MenuElement {},
    ));
}

use rand::{thread_rng, Rng};

pub fn item_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    println!("Im levelup");
    let camera = camera_query.get_single().unwrap();

    let texture_handle = asset_server.load("sprites/choose_item.png");
    let texture_item_1_handle = asset_server.load("sprites/items_tilemap.png");
    let texture_item_2_handle = asset_server.load("sprites/items_tilemap.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(400.0, 400.0), 2, 1, None, None);
    let texture_item_1_atlas = TextureAtlas::from_grid(
        texture_item_1_handle,
        Vec2::new(128.0, 128.0),
        13,
        4,
        None,
        None,
    );

    let texture_item_2_atlas = TextureAtlas::from_grid(
        texture_item_2_handle,
        Vec2::new(128.0, 128.0),
        13,
        4,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let texture_item_1_atlas_handle = texture_atlases.add(texture_item_1_atlas);
    let texture_item_2_atlas_handle = texture_atlases.add(texture_item_2_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { index: 0 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.index),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(camera.translation.x, camera.translation.y, -0.5),
            ..default()
        },
        LevelUpElement { _id: 0 },
    ));
    let mut random = thread_rng();
    let item_1: usize = random.gen_range(0..7);
    let item_2: usize = random.gen_range(7..13);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_item_1_atlas_handle,
            sprite: TextureAtlasSprite::new(item_1),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x - 95.0,
                camera.translation.y - 80.0,
                -0.4,
            ),
            ..default()
        },
        LevelUpElement { _id: 1 },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_item_2_atlas_handle,
            sprite: TextureAtlasSprite::new(item_2),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x + 95.0,
                camera.translation.y - 80.0,
                -0.4,
            ),
            ..default()
        },
        LevelUpElement { _id: 2 },
    ));
}
pub fn choose_item(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,
    chosen_item: Res<State<ItemChoice>>,
    current_mission: Res<State<CurrentMission>>,
    mut next_previous_mission: ResMut<NextState<PreviousMission>>,
    mut chosen_item_next_state: ResMut<NextState<ItemChoice>>,
    mut next_mission: ResMut<NextState<CurrentMission>>,
    mut query: Query<(&mut TextureAtlasSprite, &LevelUpElement), With<LevelUpElement>>,
) {
    for (mut sprite, level_up_element) in &mut query {
        if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::D, KeyCode::A])
        {
            if level_up_element._id == 0 && sprite.index == 1 {
                sprite.index = 0;
                println!("Chosen_item on the left");
                chosen_item_next_state.set(ItemChoice::ItemOnLeft)
            } else if level_up_element._id == 0 && sprite.index == 0 {
                sprite.index = 1;
                println!("Chosen_item on the right");
                chosen_item_next_state.set(ItemChoice::ItemOnRight)
            }
        };
        if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::D, KeyCode::A])
        {
            if level_up_element._id == 1 {
                println!("Left item: {}", sprite.index)
            } else if level_up_element._id == 2 {
                println!("Right item: {}", sprite.index)
            }
        };

        if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Return])
            && app_state.0 == AppState::LevelUp
        {
            next_previous_mission.set(match current_mission.0 {
                CurrentMission::WaterFlowers => PreviousMission::WaterFlowers,
                CurrentMission::GetWorms => PreviousMission::GetWorms,
                CurrentMission::ChopTrees => PreviousMission::ChopTrees,
                CurrentMission::CatchMouses => PreviousMission::CatchMouses,
                CurrentMission::CatchFishes => PreviousMission::CatchFishes,
                CurrentMission::FeedFishes => PreviousMission::FeedFishes,
                CurrentMission::FeedTheBears => PreviousMission::FeedTheBears,
                CurrentMission::ChopTrees => PreviousMission::ChopTrees,
                CurrentMission::CatchButterflies => PreviousMission::CatchButterflies,
                CurrentMission::FeedDonkeys => PreviousMission::FeedDonkeys,
                CurrentMission::GiveBonesToDogs => PreviousMission::GiveBonesToDogs,
                CurrentMission::TakeSheepsBack => PreviousMission::TakeSheepsBack,
                CurrentMission::TakeHoney => PreviousMission::TakeHoney,
                CurrentMission::MilkTheCow => PreviousMission::MilkTheCow,
            });

            match chosen_item.0 {
                ItemChoice::ItemOnLeft => {
                    if level_up_element._id == 1 {
                        match sprite.index {
                            0 => {
                                next_mission.set(CurrentMission::WaterFlowers);
                                println!("Next mission: Water Flowers!")
                            }
                            1 => {
                                next_mission.set(CurrentMission::GetWorms);
                                println!("Next mission: Get Worms!")
                            }
                            2 => {
                                next_mission.set(CurrentMission::ChopTrees);
                                println!("Next mission: Chop Trees!")
                            }
                            3 => {
                                next_mission.set(CurrentMission::CatchMouses);
                                println!("Next mission: Catch Mouses!")
                            }
                            4 => {
                                next_mission.set(CurrentMission::FeedTheBears);
                                println!("Next mission: Feed The Bears!")
                            }
                            5 => {
                                next_mission.set(CurrentMission::MilkTheCow);
                                println!("Next mission: Milk the Cow!")
                            }
                            6 => {
                                next_mission.set(CurrentMission::CatchButterflies);
                                println!("Next mission: CatchButterflies!")
                            }
                            _ => {
                                next_mission.set(CurrentMission::WaterFlowers);
                                println!("Next mission: Water Flowers!")
                            }
                        }
                    }
                }
                ItemChoice::ItemOnRight => {
                    if level_up_element._id == 2 {
                        match sprite.index {
                            7 => {
                                next_mission.set(CurrentMission::CatchFishes);
                                println!("Next mission: Catch Fishes!")
                            }
                            8 => {
                                next_mission.set(CurrentMission::FeedDonkeys);
                                println!("Next mission: Feed Donkeys!")
                            }
                            9 => {
                                next_mission.set(CurrentMission::GiveBonesToDogs);
                                println!("Next mission: Give bones to dogs!")
                            }
                            10 => {
                                next_mission.set(CurrentMission::TakeSheepsBack);
                                println!("Next mission: Take Sheeps Back!")
                            }
                            11 => {
                                next_mission.set(CurrentMission::FeedFishes);
                                println!("Next mission: FeedFishes!")
                            }
                            12 => {
                                next_mission.set(CurrentMission::TakeHoney);
                                println!("Next mission: Gather honey!")
                            }
                            _ => {
                                next_mission.set(CurrentMission::CatchFishes);
                                println!("Next mission: Catch Fishes!")
                            }
                        }
                    }
                }
            }

            simulation_next_state.set(SimulationState::Running);
            app_state_next_state.set(AppState::Game);
            println!("Going to game from levelup.");
        }
    }
}

pub fn play_intro(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,

    mut query: Query<
        (&mut Transform, &mut AnimationTimer, &mut TextureAtlasSprite),
        (With<IntroElement>, Without<Camera>),
    >,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<IntroElement>)>,
) {
    for (mut transform, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if sprite.index == 16 {
                sprite.index = 15
            } else {
                sprite.index += 1;
            }
        };
        for mut transform_camera in camera_query.iter_mut() {
            transform_camera.translation.x += 1.0;
            transform_camera.translation.y += 1.0;
            transform.translation = transform_camera.translation;
        }
    }
    if keyboard_input.any_just_pressed([KeyCode::Escape, KeyCode::Space, KeyCode::Return])
        && app_state.0 == AppState::Intro
    {
        app_state_next_state.set(AppState::Menu);
        println!("Going to menu from intro.");
    }
}
pub fn choose_character(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut choosen_character: ResMut<NextState<PlayerCharacter>>,
    mut simulation_next_state: ResMut<NextState<SimulationState>>,

    mut query: Query<&mut TextureAtlasSprite, With<MenuElement>>,
) {
    for mut sprite in &mut query {
        if keyboard_input.any_just_pressed([KeyCode::Left, KeyCode::Right, KeyCode::D, KeyCode::A])
        {
            if sprite.index == 1 {
                sprite.index = 0;
                choosen_character.set(PlayerCharacter::Female);
                println!("Female");
            } else {
                sprite.index = 1;
                choosen_character.set(PlayerCharacter::Male);
                println!("Male");
            }
        };
        if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Return])
            && app_state.0 == AppState::Menu
        {
            simulation_next_state.set(SimulationState::Running);
            app_state_next_state.set(AppState::Game);
            println!("Going to game from menu.");
        }
    }
}
pub fn intro_despawn(mut commands: Commands, intro_query: Query<Entity, With<IntroElement>>) {
    for intro_element in intro_query.iter() {
        commands.entity(intro_element).despawn();
    }
}
pub fn menu_despawn(mut commands: Commands, intro_query: Query<Entity, With<MenuElement>>) {
    for menu_element in intro_query.iter() {
        commands.entity(menu_element).despawn();
    }
}
pub fn item_despawn(mut commands: Commands, intro_query: Query<Entity, With<LevelUpElement>>) {
    for menu_element in intro_query.iter() {
        commands.entity(menu_element).despawn();
    }
}
