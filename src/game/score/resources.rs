use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<u32>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}

#[derive(Resource)]
pub struct Lives {
    pub value: u32,
}

impl Default for Lives {
    fn default() -> Lives {
        Lives { value: 3 }
    }
}

#[derive(Resource)]
pub struct Stage {
    pub value: u32,
}

impl Default for Stage {
    fn default() -> Stage {
        Stage { value: 0 }
    }
}
