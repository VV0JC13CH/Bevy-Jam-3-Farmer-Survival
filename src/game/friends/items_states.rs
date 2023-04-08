use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedWaterCan {
    #[default]
    Blocked,
    Enabled,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedHoe {
    #[default]
    Blocked,
    Enabled,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedAxe {
    #[default]
    Blocked,
    Enabled,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedCatItem {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedHoney {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedMilk {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBugNet {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedRod {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedApple {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBone {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedDogItem {
    #[default]
    Blocked,
    Enabled,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedWormItem {
    #[default]
    Blocked,
    Enabled,
}
