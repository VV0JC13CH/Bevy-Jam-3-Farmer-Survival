use super::components::*;
use super::states::*;
use bevy::{input::Input, prelude::*, render::camera::Camera};

use crate::game::player::PlayerCharacter;
use crate::game::AppState;
use crate::game::SimulationState;

use crate::game::score::resources::Lives;
use crate::game::score::resources::*;
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
                                next_mission.set(CurrentMission::WaterFlowers);
                                println!("Next mission: Water Flowers!")
                            }
                        }
                    }
                }
                ItemChoice::ItemOnRight => {
                    if level_up_element._id == 2 {
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

use bevy::window::PrimaryWindow;
pub fn setup_lives_icons(
    mut commands: Commands,
    player_char: Res<State<PlayerCharacter>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera_query: Query<&Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let camera = camera_query.get_single().unwrap();
    let window = window_query.get_single().unwrap();

    let texture_handle_1 = match player_char.0 {
        PlayerCharacter::Female => asset_server.load("sprites/lives_female.png"),
        PlayerCharacter::Male => asset_server.load("sprites/lives_male.png"),
    };
    let texture_handle_2 = match player_char.0 {
        PlayerCharacter::Female => asset_server.load("sprites/lives_female.png"),
        PlayerCharacter::Male => asset_server.load("sprites/lives_male.png"),
    };
    let texture_handle_3 = match player_char.0 {
        PlayerCharacter::Female => asset_server.load("sprites/lives_female.png"),
        PlayerCharacter::Male => asset_server.load("sprites/lives_male.png"),
    };
    let texture_atlas_1 =
        TextureAtlas::from_grid(texture_handle_1, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_2 =
        TextureAtlas::from_grid(texture_handle_2, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_3 =
        TextureAtlas::from_grid(texture_handle_3, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle_1 = texture_atlases.add(texture_atlas_1);
    let texture_atlas_handle_2 = texture_atlases.add(texture_atlas_2);
    let texture_atlas_handle_3 = texture_atlases.add(texture_atlas_3);
    // Use only the subset of sprites in the sheet that make up the run animation
    const HEIGHT: f32 = 64.0;
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_1,
            sprite: TextureAtlasSprite::new(0),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x - 256.0,
                camera.translation.y + window.height() / 2.0 - HEIGHT,
                -5.0,
            ),
            ..default()
        },
        GameElement { _id: 1 },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_2,
            sprite: TextureAtlasSprite::new(0),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x,
                camera.translation.y + window.height() / 2.0 - HEIGHT,
                -5.0,
            ),
            ..default()
        },
        GameElement { _id: 2 },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_3,
            sprite: TextureAtlasSprite::new(0),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x + 256.0,
                camera.translation.y + window.height() / 2.0 - HEIGHT,
                -5.0,
            ),
            ..default()
        },
        GameElement { _id: 3 },
    ));
    ///////////////////////////////////////////////////////////////////////////////////////////
    let texture_item_1_handle = asset_server.load("sprites/items_tilemap.png");
    let texture_item_2_handle = asset_server.load("sprites/entities_tilemap.png");
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
        15,
        4,
        None,
        None,
    );
    let texture_item_1_atlas_handle = texture_atlases.add(texture_item_1_atlas);
    let texture_item_2_atlas_handle = texture_atlases.add(texture_item_2_atlas);
    /////////////////////////////////////////////////////
    let texture_item_3_handle = asset_server.load("sprites/game_ui_tool.png");
    let texture_item_4_handle = asset_server.load("sprites/game_ui_target.png");
    let texture_item_3_atlas = TextureAtlas::from_grid(
        texture_item_3_handle,
        Vec2::new(128.0, 128.0),
        1,
        1,
        None,
        None,
    );

    let texture_item_4_atlas = TextureAtlas::from_grid(
        texture_item_4_handle,
        Vec2::new(128.0, 128.0),
        1,
        1,
        None,
        None,
    );
    let texture_item_3_atlas_handle = texture_atlases.add(texture_item_3_atlas);
    let texture_item_4_atlas_handle = texture_atlases.add(texture_item_4_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_item_1_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x - 512.0,
                camera.translation.y + window.height() / 2.0 - HEIGHT,
                -5.0,
            ),
            ..default()
        },
        GameElement { _id: 4 },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_item_2_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x + 512.0,
                camera.translation.y + window.height() / 2.0 - HEIGHT,
                -5.0,
            ),
            ..default()
        },
        GameElement { _id: 5 },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_item_3_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x - 512.0,
                camera.translation.y + window.height() / 2.0 - HEIGHT,
                -6.0,
            ),
            ..default()
        },
        GameElement { _id: 6 },
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_item_4_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            // transform: Transform::from_scale(Vec3::splat(1.0)),
            transform: Transform::from_xyz(
                camera.translation.x + 512.0,
                camera.translation.y + window.height() / 2.0 - HEIGHT,
                -6.0,
            ),
            ..default()
        },
        GameElement { _id: 7 },
    ));
}

pub fn show_game_icons(
    lives: Res<Lives>,
    mut icons_query: Query<
        (
            Entity,
            &mut GameElement,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        (With<GameElement>, Without<Camera>),
    >,
    camera_query: Query<&mut Transform, (With<Camera>, Without<IntroElement>)>,
    current_mission: Res<State<CurrentMission>>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let camera = camera_query.get_single().unwrap();
    let window = window_query.get_single().unwrap();

    // Lives, item and target
    for (entity, game_element, mut transform, mut sprite) in icons_query.iter_mut() {
        if keyboard_input.any_pressed([
            KeyCode::Left,
            KeyCode::A,
            KeyCode::Right,
            KeyCode::D,
            KeyCode::Up,
            KeyCode::W,
            KeyCode::Down,
            KeyCode::S,
        ]) {
            let mut direction = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            if game_element._id == 3 {
                transform.translation += direction * 300.0 * time.delta_seconds();
                if lives.value == 2 {
                    commands.entity(entity).despawn()
                }
            } else if game_element._id == 2 {
                transform.translation += direction * 300.0 * time.delta_seconds();

                if lives.value == 1 {
                    commands.entity(entity).despawn();
                }
            } else if game_element._id == 1 {
                transform.translation += direction * 300.0 * time.delta_seconds();
            } else if game_element._id == 4 {
                transform.translation += direction * 300.0 * time.delta_seconds();

                if game_element._id == 4 {
                    sprite.index = match current_mission.0 {
                        CurrentMission::WaterFlowers => 0,
                        CurrentMission::GetWorms => 1,
                        CurrentMission::ChopTrees => 2,
                        CurrentMission::CatchMouses => 3,
                        CurrentMission::FeedTheBears => 4,
                        CurrentMission::MilkTheCow => 5,
                        CurrentMission::CatchButterflies => 6,
                        CurrentMission::CatchFishes => 7,
                        CurrentMission::FeedDonkeys => 8,
                        CurrentMission::GiveBonesToDogs => 9,
                        CurrentMission::TakeSheepsBack => 10,
                        CurrentMission::FeedFishes => 11,
                        CurrentMission::TakeHoney => 12,
                    };
                }
            } else if game_element._id == 5 {
                transform.translation += direction * 300.0 * time.delta_seconds();

                sprite.index = match current_mission.0 {
                    CurrentMission::WaterFlowers => 4,
                    CurrentMission::GetWorms => 13,
                    CurrentMission::ChopTrees => 11,
                    CurrentMission::CatchMouses => 0,
                    CurrentMission::CatchFishes => 9,
                    CurrentMission::FeedFishes => 9,
                    CurrentMission::FeedTheBears => 10,
                    CurrentMission::CatchButterflies => 6,
                    CurrentMission::FeedDonkeys => 12,
                    CurrentMission::GiveBonesToDogs => 2,
                    CurrentMission::TakeSheepsBack => 14,
                    CurrentMission::TakeHoney => 11,
                    CurrentMission::MilkTheCow => 3,
                };
            } else if game_element._id == 6 {
                transform.translation += direction * 300.0 * time.delta_seconds();
            } else if game_element._id == 7 {
                transform.translation += direction * 300.0 * time.delta_seconds();
            } else {
            }
            if lives.value == 0 {
                commands.entity(entity).despawn();
            }
        } else {
            const HEIGHT: f32 = 64.0;
            if game_element._id == 3 {
                let update_transform = Transform::from_xyz(
                    camera.translation.x + 256.0,
                    camera.translation.y + window.height() / 2.0 - HEIGHT,
                    -5.0,
                );
                transform.translation = update_transform.translation;
                if lives.value == 2 {
                    commands.entity(entity).despawn()
                }
            } else if game_element._id == 2 {
                let update_transform = Transform::from_xyz(
                    camera.translation.x,
                    camera.translation.y + window.height() / 2.0 - HEIGHT,
                    -5.0,
                );
                transform.translation = update_transform.translation;

                if lives.value == 1 {
                    commands.entity(entity).despawn();
                }
            } else if game_element._id == 1 {
                let update_transform = Transform::from_xyz(
                    camera.translation.x - 256.0,
                    camera.translation.y + window.height() / 2.0 - HEIGHT,
                    -5.0,
                );
                transform.translation = update_transform.translation;
            } else if game_element._id == 4 {
                let update_transform = Transform::from_xyz(
                    camera.translation.x - 512.0,
                    camera.translation.y + window.height() / 2.0 - HEIGHT,
                    -5.0,
                );
                transform.translation = update_transform.translation;

                if game_element._id == 4 {
                    sprite.index = match current_mission.0 {
                        CurrentMission::WaterFlowers => 0,
                        CurrentMission::GetWorms => 1,
                        CurrentMission::ChopTrees => 2,
                        CurrentMission::CatchMouses => 3,
                        CurrentMission::FeedTheBears => 4,
                        CurrentMission::MilkTheCow => 5,
                        CurrentMission::CatchButterflies => 6,
                        CurrentMission::CatchFishes => 7,
                        CurrentMission::FeedDonkeys => 8,
                        CurrentMission::GiveBonesToDogs => 9,
                        CurrentMission::TakeSheepsBack => 10,
                        CurrentMission::FeedFishes => 11,
                        CurrentMission::TakeHoney => 12,
                    };
                }
            } else if game_element._id == 5 {
                let update_transform = Transform::from_xyz(
                    camera.translation.x + 512.0,
                    camera.translation.y + window.height() / 2.0 - HEIGHT,
                    -5.0,
                );
                transform.translation = update_transform.translation;

                sprite.index = match current_mission.0 {
                    CurrentMission::WaterFlowers => 4,
                    CurrentMission::GetWorms => 13,
                    CurrentMission::ChopTrees => 11,
                    CurrentMission::CatchMouses => 0,
                    CurrentMission::CatchFishes => 9,
                    CurrentMission::FeedFishes => 9,
                    CurrentMission::FeedTheBears => 10,
                    CurrentMission::CatchButterflies => 6,
                    CurrentMission::FeedDonkeys => 12,
                    CurrentMission::GiveBonesToDogs => 2,
                    CurrentMission::TakeSheepsBack => 14,
                    CurrentMission::TakeHoney => 11,
                    CurrentMission::MilkTheCow => 3,
                };
            } else if game_element._id == 6 {
                let update_transform = Transform::from_xyz(
                    camera.translation.x - 512.0,
                    camera.translation.y + window.height() / 2.0 - HEIGHT,
                    -6.0,
                );
                transform.translation = update_transform.translation;
            } else if game_element._id == 7 {
                let update_transform = Transform::from_xyz(
                    camera.translation.x + 512.0,
                    camera.translation.y + window.height() / 2.0 - HEIGHT,
                    -6.0,
                );
                transform.translation = update_transform.translation;
            } else {
            }
            if lives.value == 0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn score_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/EduNSWACTFoundation-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Stage:",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::ORANGE,
                },
            ),
            TextSection::new(
                "\n",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "Score:",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::ORANGE,
                },
            ),
            TextSection::new(
                "\n",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "Highscore:",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::ORANGE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ScoreText,
    ));
}

pub fn show_score(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
    stage: Res<Stage>,
    highscore: Res<HighScores>,
) {
    if (highscore.scores).len() > 0 {
        for mut text in &mut query {
            text.sections[1].value = stage.value.to_string();
            text.sections[4].value = score.value.to_string();
            text.sections[7].value = highscore.scores.iter().max().unwrap().to_string();
        }
    } else {
        for mut text in &mut query {
            text.sections[1].value = stage.value.to_string();
            text.sections[4].value = score.value.to_string();
            text.sections[7].value = 0.to_string();
        }
    }
}
