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
    Spawn,
    Aggro,
    Blocked,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedDog {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedCow {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedFlower {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBee {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedButterfly {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedSpider {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedBeaver {
    #[default]
    Spawn,
    Blocked,
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
    Spawn,
    Blocked,
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
    Spawn,
    Blocked,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedWorm {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum UnlockedSheep {
    #[default]
    Spawn,
    Blocked,
    Aggro,
}
