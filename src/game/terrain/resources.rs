use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}
