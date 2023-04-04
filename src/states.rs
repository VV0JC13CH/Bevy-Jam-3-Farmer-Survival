use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Intro,
    Menu,
    NewGame,
    Game,
    GameOver,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum DebugState {
    #[default]
    Develop,
    Release,
}