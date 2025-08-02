use bevy::{ecs::{entity::Entity, system::{Query, Res, ResMut}, world::Mut}, sprite::Sprite, transform::components::Transform};

use crate::{constants::{SPATIAL_GRID_CELL_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH}, resources::{spatial_grid::SpatialGrid, team_counts::TeamCounts}, teams::Team};
use itertools::Itertools;

const COLLISION_DISTANCE: f32 = 25.0;

pub fn update_spatial_grid_system(mut grid: ResMut<SpatialGrid>, query: Query<(Entity, &Transform)>) {
    for cell in grid.cells.iter_mut() {
        cell.clear();
    }

    let grid_width = (WINDOW_WIDTH / SPATIAL_GRID_CELL_SIZE).ceil() as usize;

    for (entity, transform) in &query {
        let cell_x = ((transform.translation.x + WINDOW_WIDTH / 2.0) / SPATIAL_GRID_CELL_SIZE).floor() as usize;
        let cell_y = ((transform.translation.y + WINDOW_HEIGHT / 2.0) / SPATIAL_GRID_CELL_SIZE).floor() as usize;

        let cell_idx = (cell_y * grid_width + cell_x).min(grid.cells.len() - 1);

        grid.cells[cell_idx].push(entity);
    }
}

pub fn collision_system(
    grid: Res<SpatialGrid>,
    mut query: Query<(&mut Team, &Transform, &mut Sprite)>,
    mut team_counts: ResMut<TeamCounts>,
) {
    let grid_width = (WINDOW_WIDTH / SPATIAL_GRID_CELL_SIZE).ceil() as usize;

    for (cell_idx, entities) in grid.cells.iter().enumerate().filter(|(_, c)| !c.is_empty()) {
        handle_intra_cell_collisions(&mut query, &mut team_counts, entities);

        let is_last_col = (cell_idx + 1) % grid_width == 0;
        let is_last_row = cell_idx >= grid.cells.len() - grid_width;


        if !is_last_col {
            handle_inter_cell_collisions(&mut query, &mut team_counts, entities, &grid.cells[cell_idx + 1]);
        }

        if !is_last_row {
            handle_inter_cell_collisions(&mut query, &mut team_counts, entities, &grid.cells[cell_idx + grid_width]);
        }

        if !is_last_col && !is_last_row {
            handle_inter_cell_collisions(&mut query, &mut team_counts, entities, &grid.cells[cell_idx + grid_width + 1]);
        }
    }
}

pub fn handle_intra_cell_collisions(
    query: &mut Query<(&mut Team, &Transform, &mut Sprite)>,
    team_counts: &mut ResMut<TeamCounts>,
    cell_entities: &[Entity],
) {
    let mut combinations = cell_entities.iter().combinations(2);
    while let Some([entity_a, entity_b]) = combinations.next().as_deref() {
        if let Ok([a, b]) = query.get_many_mut([**entity_a, **entity_b]) {
            handle_collision_logic(a, b, team_counts);
        }
    }
}

fn handle_inter_cell_collisions(
    query: &mut Query<(&mut Team, &Transform, &mut Sprite)>,
    team_counts: &mut ResMut<TeamCounts>,
    cell_a_entities: &[Entity],
    cell_b_entities: &[Entity],
) {
    for entity_a in cell_a_entities {
        for entity_b in cell_b_entities {
            if let Ok([a, b]) = query.get_many_mut([*entity_a, *entity_b]) {
                handle_collision_logic(a, b, team_counts);
            }
        }
    }
}

fn handle_collision_logic(
    mut a: (Mut<Team>, &Transform, Mut<Sprite>),
    mut b: (Mut<Team>, &Transform, Mut<Sprite>),
    team_counts: &mut ResMut<TeamCounts>,
) {
    if *a.0 == *b.0 {
        return;
    }
    if a.1.translation.distance_squared(b.1.translation) < COLLISION_DISTANCE.powi(2) {
        if a.0.wins_against(&b.0) {
            team_counts.decrease_team(&b.0);
            team_counts.increase_team(&a.0);
            *b.0 = *a.0;
            b.2.image = a.2.image.clone();
        } else if b.0.wins_against(&a.0) {
            team_counts.decrease_team(&a.0);
            team_counts.increase_team(&b.0);
            *a.0 = *b.0;
            a.2.image = b.2.image.clone();
        }
    }
}
