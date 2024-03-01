use std::ops::Sub;
use bevy::{
    math::{
        URect,
        Vec2,
    }
};
use bevy::math::UVec2;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, s};
use num_traits::AsPrimitive;

use crate::components::grid_components::definitions::{CellIndex1d, CellIndex2d, Grid2D};

#[inline]
pub fn calculate_2d_index(index: CellIndex1d, column_number: u32) -> CellIndex2d
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
    calculate_2d_index(index, grid_parameters.column_number)
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

#[inline]
pub fn slice_2d_array<T>(data: &Array2<T>, inclusive_rect: URect) -> ArrayView2<T> {
    let min: (usize, usize) = (inclusive_rect.min.x as usize, inclusive_rect.min.y as usize);
    let max: (usize, usize) = (inclusive_rect.max.x as usize, inclusive_rect.max.y as usize);

    assert!(data.dim().0 > max.0 && data.dim().1 > max.1, "The provided bounds are out of the original grid's range.");

    let slice = data.slice(s![min.0..=max.0, min.1..=max.1]);
    slice
}

#[inline]
pub fn slice_2d_array_mut<T>(data: &mut Array2<T>, inclusive_rect: URect) -> ArrayViewMut2<T> {
    let min: (usize, usize) = (inclusive_rect.min.x as usize, inclusive_rect.min.y as usize);
    let max: (usize, usize) = (inclusive_rect.max.x as usize, inclusive_rect.max.y as usize);

    assert!(data.dim().0 > max.0 && data.dim().1 > max.1, "The provided bounds are out of the original grid's range.");

    let slice = data.slice_mut(s![min.0..=max.0, min.1..=max.1]);
    slice
}

#[inline]
pub fn global_to_local(global_index: CellIndex2d, segment: URect) -> CellIndex2d {
    let local_x = global_index.x.sub(segment.min.x);
    let local_y = global_index.y.sub(segment.min.y);
    return CellIndex2d { x: local_x, y: local_y };
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

pub fn normalize_rect(rect: URect) -> URect {
    URect {
        min: UVec2::ZERO,
        max: UVec2 { x: rect.width(), y: rect.height() },
    }
}