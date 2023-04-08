
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CurrentMission {
    #[default]
    WaterFlowers,
    GetWorms,
    CatchMouses,
    CatchFishes,
    FeedFishes,
    FeedTheBears,
    ChopTrees,
    CatchButterflies,
    FeedDonkeys,
    GiveBonesToDogs,
    TakeSheepsBack,
    TakeHoney,
    MilkTheCow,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PreviousMission {
    #[default]
    NewGame,
    WaterFlowers,
    GetWorms,
    CatchMouses,
    CatchFishes,
    FeedFishes,
    FeedTheBears,
    ChopTrees,
    CatchButterflies,
    FeedDonkeys,
    GiveBonesToDogs,
    TakeSheepsBack,
    TakeHoney,
    MilkTheCow,
}

