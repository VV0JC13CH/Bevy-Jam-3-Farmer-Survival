mod resources;
mod systems;

use bevy_ecs_tilemap::prelude::*;
use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct TerrainPlugin;

// For this example, don't choose too large a chunk size.

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            // `TilemapRenderSettings` be added before the `TilemapPlugin`.
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            })
            .add_plugin(TilemapPlugin)
            .insert_resource(ChunkManager::default())
            // Systems
            .add_systems(
                (
                    terrain_spawn_around_camera,
                    //terrain_despawn_around_camera,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
            // On Exit State
 //           .add_system(terrain_despawn_all.in_schedule(OnExit(AppState::Game)));
    }
}
