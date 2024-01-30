use std::cmp::min;

use bevy::{
    math::{Rect, Vec2},
    prelude::UVec2,
};
use ndarray::{Array2, s};
use num_traits::{AsPrimitive, Unsigned};
use crate::{
    components::{
        grid_components::{CellIndex1d, CellIndex2d, Grid2D, GridCellData, GridRelatedData},
        movement_components::{
            SurfaceCoordinate,
            Coordinate,
        },
        pathfinding_components::PathfindingMap,
    }
};

impl GridRelatedData {
    pub fn new(grid_parameters: &Grid2D) -> Self {
        GridRelatedData { data: Array2::default((grid_parameters.column_number as usize, grid_parameters.row_number as usize)) }
    }

    pub fn create_pathfinding_map(&self, top_left: (usize, usize), bottom_right: (usize, usize)) -> PathfindingMap {
        let width = bottom_right.0 - top_left.0 + 1;
        let height = bottom_right.1 - top_left.1 + 1;

        assert!(self.data.dim().0 > bottom_right.0 && self.data.dim().1 > bottom_right.1, "The provided bounds are out of the original grid's range.");
        let slice = self.data.slice(s![top_left.0..=bottom_right.0, top_left.1..=bottom_right.1]);
        PathfindingMap {
            grid_segment_data: slice,
            height: height as i32,
            width: width as i32,
        }
    }

    pub fn get_data_at_mut(&mut self, cell_index: CellIndex2d) -> Option<&mut GridCellData> { self.data.get_mut(cell_index) }

    pub fn get_data_at(&self, cell_index: CellIndex2d) -> Option<&GridCellData> { self.data.get(cell_index) }
}

pub struct CoordinateIterator {
    inner: std::vec::IntoIter<(u32, u32)>,
}

impl CoordinateIterator {
    pub fn new(start_i: u32, end_i: u32, start_j: u32, end_j: u32) -> Self {
        let data: Vec<_> = (start_j..end_j).flat_map(move |j| (start_i..end_i).map(move |i| (i, j))).collect();
        let inner = data.into_iter();
        Self { inner }
    }
}


impl Iterator for CoordinateIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl DoubleEndedIterator for CoordinateIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl Grid2D {
    pub fn iterate_coordinates(&self) -> CoordinateIterator {
        CoordinateIterator::new(0, self.column_number, 0, self.row_number)
    }

    pub fn coordinates_range(&self, min: UVec2, max: UVec2) -> CoordinateIterator {
        assert!(max.x <= self.column_number);
        assert!(max.y <= self.row_number);
        CoordinateIterator::new(min.x, max.x, min.y, max.y)
    }

    pub fn new(column_number: u32, row_number: u32, cell_size: Vec2) -> Self {
        let grid_size = Vec2::new(column_number as f32 * cell_size.x, row_number as f32 * cell_size.y);
        Grid2D {
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
    pub fn calculate_cell_position(&self, cell_index: CellIndex2d) -> Vec2 {
        let cell_size = self.cell_size;
        let grid_center = self.rect.center();
        let cell_index_float: Vec2 = cell_index.into();

        Vec2::new(grid_center.x - (self.grid_size.x / 2.0) + (cell_index_float.x * cell_size.x) + (cell_size.x / 2.0),
                  grid_center.y - (self.grid_size.y / 2.0) + (cell_index_float.y * cell_size.y) + (cell_size.y / 2.0))
    }

    #[inline]
    pub fn calculate_indexes_limits_in_rang(&self, center_cell_index: CellIndex2d, radius: u32) -> (CellIndex2d, CellIndex2d) {
        let cell_index_float: UVec2 = center_cell_index.into();

        let min_x = cell_index_float.x.saturating_sub(radius);
        let min_y = cell_index_float.y.saturating_sub(radius);

        let max_x = min(cell_index_float.x + radius, self.max_column_index);
        let max_y = min(cell_index_float.y + radius, self.max_row_index);

        return (CellIndex2d::new(min_x, min_y), CellIndex2d::new(max_x, max_y));
    }

    #[inline]
    pub fn calculate_cell_index_from_position(&self, position: Vec2) -> CellIndex2d {
        let grid_center = self.rect.center();

        let cell_index_x = ((position.x + self.grid_size.x / 2.0 - grid_center.x) / self.cell_size.x).floor() as u32;
        let cell_index_y = ((position.y + self.grid_size.y / 2.0 - grid_center.y) / self.cell_size.y).floor() as u32;

        self.form_grid_bound_cell_index(cell_index_x, cell_index_y)
    }

    #[inline]
    pub fn form_grid_bound_cell_index(&self, cell_index_x: u32, cell_index_y: u32) -> CellIndex2d {
        CellIndex2d::new(cell_index_x.clamp(0u32, self.max_column_index), cell_index_y.clamp(0u32, self.max_row_index))
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from(&self, cell_index2d: CellIndex2d) -> SurfaceCoordinate
    {
        let latitude: Coordinate = f32::from(cell_index2d.x) / self.max_column_index as f32;
        let longitude: Coordinate = f32::from(cell_index2d.y) / self.max_row_index as f32;
        SurfaceCoordinate { latitude, longitude }
    }

    pub fn is_cell_index_in_grid_bounds(&self, cell_index: CellIndex2d) -> bool {
        u32::from(cell_index.x) < self.column_number && u32::from(cell_index.y) < self.row_number
    }

    pub fn is_position_in_grid_bounds(&self, position: Vec2) -> bool {
        self.rect.contains(position)
    }
}

#[inline]
pub fn calculate_2d_index<T>(index: T, column_number: u32) -> CellIndex2d
    where
        T: Unsigned + Copy + From<u32>, u32: From<T>
{
    let index_u32: u32 = index.into();
    CellIndex2d::new(index_u32 % column_number, index_u32 / column_number)
}

#[inline]
pub fn calculate_1d_index(index: CellIndex2d, column_number: u32) -> CellIndex1d {
    index.x + index.y * column_number
}

#[inline]
pub fn calculate_2d_from_1d_index(grid_parameters: &Grid2D, index: CellIndex1d) -> CellIndex2d
{
    calculate_2d_index::<u32>(index.into(), grid_parameters.column_number)
}

#[inline]
pub fn calculate_1d_from_2d_index(grid_parameters: &Grid2D, index: CellIndex2d) -> CellIndex1d {
    calculate_1d_index(index, grid_parameters.column_number)
}

#[inline]
pub fn calculate_indexes_in_circle_from_index(grid_parameters: &Grid2D, center_cell_index: CellIndex2d, radius: u32)
                                              -> Vec<CellIndex2d> {
    let indexes = calculate_indexes_in_range(grid_parameters, center_cell_index, radius);
    let result_indexes: Vec<CellIndex2d> = indexes.into_iter()
        .filter(|&index| center_cell_index.euclidean_distance(&index) <= radius as f32)
        .collect();
    result_indexes
}

#[inline]
pub fn calculate_indexes_in_range(grid_parameters: &Grid2D, center_cell_index: CellIndex2d, radius: u32) -> Vec<CellIndex2d> {
    let (min_limit, max_limit) = grid_parameters.calculate_indexes_limits_in_rang(center_cell_index, radius);

    (min_limit.y.as_()..=max_limit.y.as_()).flat_map(|y: u32| {
        (min_limit.x.as_()..=max_limit.x.as_()).map(move |x| CellIndex2d::new(x, y))
    }).collect()
}

pub fn euclidean_distance(a: Vec2, b: Vec2) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    ((dx * dx) + (dy * dy)).sqrt()
}

pub fn is_position_within_bounds(grid_position_center: Vec2, grid_parameter: &Grid2D, position: Vec2) -> bool {
    let half_grid_size = (grid_parameter.grid_size * grid_parameter.cell_size) / 2.0;
    let bottom_left_corner = grid_position_center - half_grid_size;
    let top_right_corner = grid_position_center + half_grid_size;

    // Check if position is within bounds
    position.x >= bottom_left_corner.x
        && position.y >= bottom_left_corner.y
        && position.x <= top_right_corner.x
        && position.y <= top_right_corner.y
}
