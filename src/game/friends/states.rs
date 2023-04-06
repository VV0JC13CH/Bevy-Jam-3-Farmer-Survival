use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedMouse {
    #[default]
    Spawn,
    Aggro,
    Blocked,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedCat {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedDog {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedCow {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedFlower {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBee {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedButterfly {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedSpider {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBeaver {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedFish {
    #[default]
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBear {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedTree {
    #[default]
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedDonkey {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedWorm {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedSheep {
    #[default]
    Blocked,
    Spawn,
    Aggro,
}
