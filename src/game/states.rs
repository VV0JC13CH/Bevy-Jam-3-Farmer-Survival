
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

