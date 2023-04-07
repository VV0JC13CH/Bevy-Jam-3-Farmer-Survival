use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedWaterCan {
    #[default]
    Enabled,

    Blocked,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedHoe {
    #[default]
    Enabled,

    Blocked,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedAxe {
    #[default]
    Enabled,
    Blocked,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedCatItem {
    #[default]
    Enabled,

    Blocked,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedHoney {
    #[default]
    Enabled,

    Blocked,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedMilk {
    #[default]
    Enabled,

    Blocked,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBugNet {
    #[default]
    Enabled,

    Blocked,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedRod {
    #[default]
    Enabled,

    Blocked,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedApple {
    #[default]
    Enabled,

    Blocked,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBone {
    #[default]
    Enabled,

    Blocked,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedDogItem {
    #[default]
    Enabled,

    Blocked,
}
