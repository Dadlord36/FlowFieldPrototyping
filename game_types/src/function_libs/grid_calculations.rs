use std::{
    collections::HashMap,
    ops::Sub,
};

use bevy::math::{
    FloatExt,
    URect,
    UVec2,
    Vec2,
};
use ndarray::{Array2, ArrayView2, ArrayViewMut2, s};
use num_traits::AsPrimitive;

use crate::components::{
    directions::Direction,
    grid_components::definitions::{CellIndex1d, CellIndex2d, Grid2D},
};

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

/// Splits a grid into eight compass directions: north, northeast, east, southeast, south, southwest, west, and northwest.
///
/// # Arguments
///
/// * `grid` - A reference to a `URect` representing the grid to split.
///
/// # Returns
///
/// An array of `URect` representing the eight compass directions.
/*pub fn split_grid_in_compass_directions(grid: &URect) -> HashMap<Direction, URect> {
    let min = grid.min.as_vec2();
    let max = grid.max.as_vec2();

    let first_seg_max_x = (min.x + (max.x - min.x) / 3.0).round() as u32;
    let second_seg_max_x = (min.x + 2.0 * (max.x - min.x) / 3.0).round() as u32;
    let first_seg_max_y = (min.y + (max.y - min.y) / 3.0).round() as u32;
    let second_seg_max_y = (min.y + 2.0 * (max.y - min.y) / 3.0).round() as u32;

    let min = grid.min;
    let max = grid.max;
    let mut directions_map: HashMap<Direction, URect> = HashMap::with_capacity(8);

    directions_map.insert(Direction::NorthWest, URect::new(min.x, second_seg_max_y + 1, first_seg_max_x, max.y));
    directions_map.insert(Direction::North, URect::new(first_seg_max_x + 1, second_seg_max_y + 1, second_seg_max_x, max.y));
    directions_map.insert(Direction::NorthEast, URect::new(second_seg_max_x + 1, second_seg_max_y + 1, max.x, max.y));
    directions_map.insert(Direction::West, URect::new(min.x, first_seg_max_y + 1, first_seg_max_x, second_seg_max_y));
    directions_map.insert(Direction::East, URect::new(second_seg_max_x + 1, first_seg_max_y + 1, max.x, second_seg_max_y));
    directions_map.insert(Direction::SouthWest, URect::new(min.x, min.y, first_seg_max_x, first_seg_max_y));
    directions_map.insert(Direction::South, URect::new(first_seg_max_x + 1, min.y, second_seg_max_x, first_seg_max_y));
    directions_map.insert(Direction::SouthEast, URect::new(second_seg_max_x + 1, min.y, max.x, first_seg_max_y));

    directions_map
}*/

pub fn split_grid_in_compass_directions(grid: &URect) -> HashMap<Direction, URect> {
    let min = grid.min.as_vec2();
    let max = grid.max.as_vec2();

    let first_seg_max_x: u32 = (min.x + (max.x - min.x) / 3.0) as u32;
    let second_seg_max_x: u32 = (min.x + 2.0 * (max.x - min.x) / 3.0) as u32;
    let first_seg_max_y: u32 = (min.y + (max.y - min.y) / 3.0) as u32;
    let second_seg_max_y: u32 = (min.y + 2.0 * (max.y - min.y) / 3.0) as u32;

    // Compute the center cell index.
    let center_x: u32 = ((max.x + min.x) / 2.0) as u32;
    let center_y: u32 = ((max.y + min.y) / 2.0) as u32;


    let min = grid.min;
    let max = grid.max;

    let mut directions_map: HashMap<Direction, URect> = HashMap::with_capacity(8);
    directions_map.insert(Direction::NorthWest, URect::new(min.x, second_seg_max_y + 1, first_seg_max_x, max.y));
    directions_map.insert(Direction::North, URect::new(first_seg_max_x + 1, second_seg_max_y + 1, second_seg_max_x, max.y));
    directions_map.insert(Direction::NorthEast, URect::new(second_seg_max_x + 1, second_seg_max_y + 1, max.x, max.y));
    directions_map.insert(Direction::West, URect::new(min.x, first_seg_max_y + 1, center_x, second_seg_max_y));
    directions_map.insert(Direction::East, URect::new(center_x + 1, first_seg_max_y + 1, max.x, second_seg_max_y));
    directions_map.insert(Direction::SouthWest, URect::new(min.x, min.y, first_seg_max_x, first_seg_max_y));
    directions_map.insert(Direction::South, URect::new(first_seg_max_x + 1, min.y, second_seg_max_x, first_seg_max_y));
    directions_map.insert(Direction::SouthEast, URect::new(second_seg_max_x + 1, min.y, max.x, first_seg_max_y));

    directions_map
}


/// Checks if two rectangles intersect.
///
/// # Arguments
///
/// * `rect1` - The first rectangle.
/// * `rect2` - The second rectangle.
///
/// # Returns
///
/// Returns `true` if the rectangles intersect, `false` otherwise.
///
/// # Examples
///
/// ```
/// use bevy::prelude::{URect, UVec2};
/// use game_types::function_libs::grid_calculations::{self};
/// //Non intersection case
/// {
///    let rect = URect {
///    min: UVec2 { x: 0, y: 0 },
///    max: UVec2 { x: 12, y: 12 },
///    };
///
///    let rect2 = URect {
///    min: UVec2 { x: 13, y: 13 },
///    max: UVec2 { x: 22, y: 22 },
///    };
///
///    let result = grid_calculations::are_intersecting_inclusive(rect, rect2);
///    assert!(!result, "result was {result}")
/// }
/// //Intersection case
/// {
///    let rect = URect {
///    min: UVec2 { x: 0, y: 0 },
///    max: UVec2 { x: 12, y: 12 },
/// };
///
///    let rect2 = URect {
///    min: UVec2 { x: 5, y: 5 },
///    max: UVec2 { x: 22, y: 22 },
///    };
///
///    let result = grid_calculations::are_intersecting_inclusive(rect, rect2);
///    assert!(result, "result was {result}")
///}
/// println!("Intersection is working fine!")
/// ```
/*#[inline]
pub fn are_intersecting_inclusive(rect1: URect, rect2: URect) -> bool {
    rect1.contains(rect2.min) || rect1.contains(rect2.max)
}*/

/*#[inline]
pub fn are_intersecting_exclusive(rect1: URect, rect2: URect) -> bool {
    rect_contains_exclusive(rect1, rect2.min) || rect_contains_exclusive(rect1, rect2.max)
}*/
#[inline]
fn rect_contains_exclusive(rect: URect, point: UVec2) -> bool {
    (point.cmpgt(rect.min) & point.cmplt(rect.max)).all()
}

/*#[inline]
pub fn are_intersecting_exclusive(rect: URect, other: URect) -> bool {
    (rect.min.cmpge(other.max) | rect.max.cmple(other.min)).any()
}*/

pub fn are_intersecting_exclusive(rect: URect, other: URect) -> bool {
    !(rect.max.x < other.min.x ||
        rect.min.x > other.max.x ||
        rect.max.y < other.min.y ||
        rect.min.y > other.max.y)
}

pub fn are_intersecting_inclusive(rect: URect, other: URect) -> bool {
    !(rect.max.x <= other.min.x ||
        rect.min.x >= other.max.x ||
        rect.max.y <= other.min.y ||
        rect.min.y >= other.max.y)
}