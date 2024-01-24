use std::cmp::min;

use bevy::math::{Rect, UVec2, Vec2};

use crate::{
    components::{
        grid_components::{GridCellData, GridParameters, GridRelatedData},
        movement_components::{Coordinate, SurfaceCoordinate}
    }
};

impl GridRelatedData {
    pub fn new(grid_parameters: &GridParameters) -> Self {
        let items_num = (grid_parameters.row_number * grid_parameters.column_number) as usize;
        GridRelatedData { data: vec![GridCellData::default(); items_num] }
    }

    pub fn get_data_at_mut(&mut self, grid_parameters: &GridParameters, cell_index: UVec2)
                           -> Option<&mut GridCellData> {
        self.data.get_mut(calculate_1d_from_2d_index(grid_parameters, cell_index))
    }

    pub fn get_data_at(&self, grid_parameters: &GridParameters, cell_index: UVec2) -> Option<&GridCellData> {
        self.data.get(calculate_1d_from_2d_index(grid_parameters, cell_index))
    }
}

pub struct CoordinateIterator {
    inner: Box<dyn Iterator<Item=(u32, u32)> + 'static>,
}

impl CoordinateIterator {
    pub fn new(max_i: u32, max_j: u32) -> Self {
        let inner = (0..max_j).flat_map(move |j| (0..max_i).map(move |i| (i, j)));
        Self { inner: Box::new(inner) }
    }
}

impl Iterator for CoordinateIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl GridParameters {
    pub fn coordinates(&self) -> CoordinateIterator {
        CoordinateIterator::new(self.column_number, self.row_number)
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
            max_column_index: column_number - 1,
            max_row_index: row_number - 1,
        }
    }

    #[inline]
    pub fn calculate_cell_position(&self, cell_index: UVec2) -> Vec2 {
        let cell_size = self.cell_size;
        let grid_center = self.rect.center();

        Vec2::new(grid_center.x - (self.grid_size.x / 2.0) + (cell_index.x as f32 * cell_size.x) + (cell_size.x / 2.0),
                  grid_center.y - (self.grid_size.y / 2.0) + (cell_index.y as f32 * cell_size.y) + (cell_size.y / 2.0))
    }

    #[inline]
    pub fn calculate_indexes_limits_in_rang(&self, center_cell_index: UVec2, radius: u32) -> (UVec2, UVec2) {
        let min_x = center_cell_index.x.saturating_sub(radius);
        let min_y = center_cell_index.y.saturating_sub(radius);
        let min_limit = UVec2::new(min_x, min_y);

        let max_x = min(center_cell_index.x + radius, self.max_column_index);
        let max_y = min(center_cell_index.y + radius, self.max_row_index);

        let max_limit = UVec2::new(max_x, max_y);
        return (min_limit, max_limit);
    }

    #[inline]
    pub fn calculate_cell_index_from_position(&self, position: Vec2) -> UVec2 {
        let grid_center = self.rect.center();

        let cell_index_x = ((position.x + self.grid_size.x / 2.0 - grid_center.x) / self.cell_size.x).floor() as u32;
        let cell_index_y = ((position.y + self.grid_size.y / 2.0 - grid_center.y) / self.cell_size.y).floor() as u32;

        self.form_grid_bound_cell_index(cell_index_x, cell_index_y)
    }

    #[inline]
    pub fn form_grid_bound_cell_index(&self, cell_index_x: u32, cell_index_y: u32) -> UVec2 {
        UVec2::new(cell_index_x.clamp(0u32, self.max_column_index), cell_index_y.clamp(0u32, self.max_row_index))
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from(&self, cell_index2d: UVec2) -> SurfaceCoordinate
    {
        let latitude = cell_index2d.x as Coordinate / self.max_column_index as Coordinate;
        let longitude = cell_index2d.y as Coordinate / self.max_row_index as Coordinate;
        SurfaceCoordinate { latitude, longitude }
    }

    pub fn is_cell_index_in_grid_bounds(&self, cell_index: UVec2) -> bool {
        cell_index.x < self.column_number && cell_index.y < self.row_number
    }

    pub fn is_position_in_grid_bounds(&self, position: Vec2) -> bool {
        self.rect.contains(position)
    }
}

#[inline]
pub fn calculate_2d_from_1d_index(grid_parameters: &GridParameters, index: u32) -> UVec2 {
    UVec2::new(index % grid_parameters.column_number, index / grid_parameters.column_number)
}

#[inline]
pub fn calculate_1d_from_2d_index(grid_parameters: &GridParameters, index: UVec2) -> usize {
    ((index.x) + (index.y) * grid_parameters.column_number) as usize
}

#[inline]
pub fn calculate_indexes_in_circle_from_index(grid_parameters: &GridParameters, center_cell_index: UVec2, radius: u32)
                                              -> Vec<UVec2> {
    let indexes = calculate_indexes_in_range(grid_parameters, center_cell_index, radius);
    let result_indexes: Vec<UVec2> = indexes.into_iter()
        .filter(|&index| euclidean_distance_unsigned(center_cell_index, index) <= radius as f32)
        .collect();
    result_indexes
}

#[inline]
pub fn calculate_indexes_in_range(grid_parameters: &GridParameters, center_cell_index: UVec2, radius: u32)
                                  -> Vec<UVec2> {
    let (min_limit, max_limit) = grid_parameters.calculate_indexes_limits_in_rang(center_cell_index, radius);

    (min_limit.y..=max_limit.y).flat_map(|y| {
        (min_limit.x..=max_limit.x).map(move |x| UVec2::new(x, y))
    }).collect()
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