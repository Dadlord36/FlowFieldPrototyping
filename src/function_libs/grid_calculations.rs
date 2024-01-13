use std::cmp::min;

use bevy::math::{Rect, UVec2, Vec2};
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GridParameters {
    pub column_number: u32,
    pub row_number: u32,
    pub cell_size: Vec2,
    pub grid_size: Vec2,
    pub cells_spacing: f32,
    pub rect: Rect,
}

pub struct CoordinateIterator {
    max_i: u32,
    max_j: u32,
    current_i: u32,
    current_j: u32,
}

impl Iterator for CoordinateIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_i == self.max_i {
            self.current_i = 0;
            self.current_j += 1;

            if self.current_j == self.max_j {
                return None;
            }
        }

        let result = Some((self.current_i, self.current_j));
        self.current_i += 1;
        result
    }
}

impl GridParameters {
    pub fn coordinates(&self) -> CoordinateIterator {
        CoordinateIterator {
            max_i: self.column_number,
            max_j: self.row_number,
            current_i: 0,
            current_j: 0,
        }
    }

    pub fn new(column_number: u32, row_number: u32, cell_size: Vec2) -> Self {
        let grid_size = Vec2::new(column_number as f32 * cell_size.x, row_number as f32 * cell_size.y);
        GridParameters {
            column_number,
            row_number,
            cell_size,
            grid_size,
            cells_spacing: 0.0,
            rect: Rect::from_center_size(Vec2::ZERO, grid_size),
        }
    }
}

pub fn calculate_2d_from_1d_index(grid_parameters: &GridParameters, index: u32) -> UVec2 {
    UVec2::new(index % grid_parameters.column_number, index / grid_parameters.column_number)
}

pub fn calculate_1d_from_2d_index(grid_parameters: &GridParameters, index: UVec2) -> usize {
    ((index.x) + (index.y) * grid_parameters.column_number) as usize
}

pub fn calculate_cell_position(grid_parameters: &GridParameters, grid_center: Vec2, cell_index: UVec2) -> Vec2 {
    let cell_size = grid_parameters.cell_size;
    Vec2::new(grid_center.x - (grid_parameters.grid_size.x / 2.0) + (cell_index.x as f32 * cell_size.x) + (cell_size.x / 2.0),
              grid_center.y - (grid_parameters.grid_size.y / 2.0) + (cell_index.y as f32 * cell_size.y) + (cell_size.y / 2.0))
}

pub fn calculate_cell_index_from_position(grid_parameters: &GridParameters, grid_center: Vec2, position: Vec2) -> UVec2 {
    UVec2::new(
        ((position.x + grid_parameters.grid_size.x / 2.0 - grid_center.x) / grid_parameters.cell_size.x).floor() as u32,
        ((position.y + grid_parameters.grid_size.y / 2.0 - grid_center.y) / grid_parameters.cell_size.y).floor() as u32,
    )
}


pub fn calculate_indexes_in_circle_from_index(grid_parameters: &GridParameters, grid_position_center: Vec2, center_cell_index: UVec2, radius: f32) -> Vec<UVec2> {
    let central_cell_position = calculate_cell_position(&grid_parameters, grid_position_center, center_cell_index);

    let radius_in_cells_x = (radius / grid_parameters.cell_size.x).ceil() as u32;
    let radius_in_cells_y = (radius / grid_parameters.cell_size.y).ceil() as u32;

    let min_index = UVec2::new(
        center_cell_index.x.saturating_sub(radius_in_cells_x),
        center_cell_index.y.saturating_sub(radius_in_cells_y),
    );

    let max_index = UVec2::new(
        min(center_cell_index.x + radius_in_cells_x, grid_parameters.column_number),
        min(center_cell_index.y + radius_in_cells_y, grid_parameters.row_number),
    );

    let mut indexes: Vec<UVec2> = Vec::new();

    for y in min_index.y..=max_index.y {
        for x in min_index.x..=max_index.x {
            let index = UVec2::new(x, y);
            let cell_position = calculate_cell_position(&grid_parameters, grid_position_center, index);

            if euclidean_distance(Vec2::new(cell_position.x, cell_position.y), Vec2::new(central_cell_position.x, central_cell_position.y)) <= radius {
                indexes.push(index);
            }
        }
    }
    indexes
}

pub fn euclidean_distance_unsigned(a: UVec2, b: UVec2) -> f32 {
    let dx = a.x as f32 - b.x as f32;
    let dy = a.y as f32 - b.y as f32;
    ((dx * dx) + (dy * dy)).sqrt()
}

pub fn euclidean_distance(a: Vec2, b: Vec2) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    ((dx * dx) + (dy * dy)).sqrt()
}

pub fn is_position_within_bounds(grid_position_center: Vec2, grid_parameter: &GridParameters, position: Vec2) -> bool {
    let half_grid_size = (grid_parameter.grid_size * grid_parameter.cell_size) / 2.0;
    let bottom_left_corner = grid_position_center - half_grid_size;
    let top_right_corner = grid_position_center + half_grid_size;

    // Check if position is within bounds
    position.x >= bottom_left_corner.x
        && position.y >= bottom_left_corner.y
        && position.x <= top_right_corner.x
        && position.y <= top_right_corner.y
}