use bevy::ecs::{entity::Entity, resource::Resource};

use crate::constants::{SPATIAL_GRID_CELL_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Resource, Debug)]
pub struct SpatialGrid {
    pub cells: [Vec<Entity>; ((WINDOW_WIDTH * WINDOW_HEIGHT) / (SPATIAL_GRID_CELL_SIZE * SPATIAL_GRID_CELL_SIZE)) as usize]
}
