use std::{
    fmt::{self, Debug, Formatter},
    ops::{
        Add,
        Mul,
        Sub,
    },
};
use std::cmp::min;

use bevy::{
    math::{
        Rect,
        URect,
    },
    prelude::{
        IVec2,
        UVec2,
        Vec2,
    },
    render::color::Color,
};
use derive_more::AddAssign;
use ndarray::{Array2, ArrayView2, IndexLonger, Ix2, NdIndex, s};
use num_traits::AsPrimitive;
use rand::Rng;

use crate::{
    components::{
        grid_components::definitions::Occupation,
        movement_components::{Coordinate, SurfaceCoordinate},
        pathfinding_components::PathfindingMap,
    },
    function_libs::grid_calculations::{calculate_1d_index, calculate_2d_index, slice_2d_array},
};

use super::definitions::{CellIndex1d, CellIndex2d, Grid2D, GridCellData, GridRelatedData};

impl From<UVec2> for CellIndex2d {
    fn from(vec: UVec2) -> Self {
        CellIndex2d::new(vec.x, vec.y)
    }
}

impl From<Vec2> for CellIndex2d {
    fn from(vec: Vec2) -> Self {
        CellIndex2d::new(vec.x, vec.y)
    }
}

impl From<CellIndex2d> for Vec2 {
    fn from(index: CellIndex2d) -> Self {
        Vec2 { x: index.x as f32, y: index.y as f32 }
    }
}

impl From<CellIndex2d> for UVec2 {
    fn from(value: CellIndex2d) -> Self {
        UVec2 { x: value.x.into(), y: value.y.into() }
    }
}

impl Add<IVec2> for CellIndex2d {
    type Output = CellIndex2d;

    fn add(self, rhs: IVec2) -> Self::Output {
        CellIndex2d {
            x: (self.x as i32 + rhs.x) as u32,
            y: (self.y as i32 + rhs.y) as u32,
        }
    }
}

impl AddAssign<IVec2> for CellIndex2d {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x = (self.x as i32 + rhs.x) as u32;
        self.y = (self.y as i32 + rhs.y) as u32;
    }
}

impl Mul<Vec2> for CellIndex2d {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(
            (self.x as f32) * rhs.x,
            (self.y as f32) * rhs.y,
        )
    }
}

impl Mul<u32> for CellIndex2d {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f64> for CellIndex2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64 = self.x.as_();
        let y: f64 = self.y.as_();
        Self {
            x: (x * rhs) as u32,
            y: (y * rhs) as u32,
        }
    }
}

impl Mul<i32> for CellIndex2d {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as u32,
            y: self.y * rhs as u32,
        }
    }
}

impl Debug for CellIndex2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GridCellIndex {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl fmt::Display for CellIndex2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl CellIndex2d {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new<T: AsPrimitive<u32>>(x: T, y: T) -> Self {
        CellIndex2d {
            x: x.as_(),
            y: y.as_(),
        }
    }

    pub fn normalize(&self) -> Self {
        let length = ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt() as u32;
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn euclidean_distance(&self, other: &CellIndex2d) -> f32 {
        let x_distance = other.x as f64 - self.x as f64;
        let y_distance = other.y as f64 - self.y as f64;

        (x_distance.powi(2) + y_distance.powi(2)).sqrt() as f32
    }
}

unsafe impl NdIndex<Ix2> for &CellIndex2d {
    fn index_checked(&self, dim: &Ix2, strides: &Ix2) -> Option<isize> {
        if (self.x as usize) < dim[0] && (self.y as usize) < dim[1] {
            Some(self.index_unchecked(strides))
        } else {
            None
        }
    }

    fn index_unchecked(&self, strides: &Ix2) -> isize {
        (self.x as isize * strides[0] as isize) + (self.y as isize * strides[1] as isize)
    }
}

/*unsafe impl NdIndex<Ix2> for &CellIndex2d {
fn index_checked(&self, dim: &Ix2, strides: &Ix2) -> Option<isize> {
    if (self.x as usize) < dim[0] && (self.y as usize) < dim[1] {
        Some(self.index_unchecked(strides))
    } else {
        None
    }
}

fn index_unchecked(&self, strides: &Ix2) -> isize {
    (self.x as isize * strides[0] as isize) + (self.y as isize * strides[1] as isize)
}
}*/

impl GridRelatedData {
    pub fn new(grid_parameters: &Grid2D) -> Self {
        GridRelatedData { data: Array2::default((grid_parameters.column_number as usize, grid_parameters.row_number as usize)) }
    }

    pub fn create_pathfinding_map(&self, inclusive_rect: URect) -> PathfindingMap {
        let min: (usize, usize) = (inclusive_rect.min.x as usize, inclusive_rect.min.y as usize);
        let max: (usize, usize) = (inclusive_rect.max.x as usize, inclusive_rect.max.y as usize);
        assert!(self.data.dim().0 > max.0 && self.data.dim().1 > max.1, "The provided bounds are out of the original grid's range.");

        let slice = self.data.slice(s![min.0..=max.0, min.1..=max.1]);
        PathfindingMap::new(slice, inclusive_rect)
    }

    pub fn has_obstacle_in(&self, area: URect) -> bool {
        let mut has = false;
        for cell_data in self.get_segment_view_of(area) {
            if cell_data.occupation_state != Occupation::Free {
                has = true;
                break;
            }
        }

        return has;
    }

    pub fn get_segment_view_of(&self, area: URect) -> ArrayView2<GridCellData> {
        slice_2d_array(&self.data, area)
    }

    pub fn fill_with_random_obstacle_pattern(&mut self, grid_parameters: &Grid2D) {
        let mut rng = rand::thread_rng();
        let shape = (grid_parameters.row_number as usize, grid_parameters.column_number as usize);
        self.data = Array2::from_shape_fn(shape, |_idx| {
            let occupation = if rng.gen_bool(0.9) {
                Occupation::Free
            } else {
                Occupation::Occupied
            };
            GridCellData {
                color: Color::WHITE, // Adjust color corresponding to Occupation
                occupation_state: occupation,
            }
        });
    }

    pub fn get_data_at_mut(&mut self, cell_index: &CellIndex2d) -> &mut GridCellData { &mut self.data[cell_index] }
    pub fn get_data_at(&self, cell_index: &CellIndex2d) -> &GridCellData { &self.data[cell_index] }
}

pub struct AreaLineIterator {
    bounds: URect,
    current: CellIndex2d,
    direction: IVec2,
}

impl Iterator for AreaLineIterator {
    type Item = CellIndex2d;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.bounds.contains((self.current + self.direction).into())
        {
            return None;
        }

        let result = Some(self.current);
        self.current += self.direction;

        result
    }
}

pub struct AreaFullIterator {
    bounds: URect,
    current: CellIndex2d,
    direction: IVec2,
}

impl Iterator for AreaFullIterator {
    type Item = CellIndex2d;

    fn next(&mut self) -> Option<Self::Item> {
        let offset_x = IVec2::new(self.direction.x, 0);
        let offset_y = IVec2::new(0, self.direction.y);
        // let prev = self.current;

        if offset_y == IVec2::ZERO && offset_x == IVec2::ZERO {
            return None;
        }

        if offset_y != IVec2::ZERO && self.bounds.contains((self.current + offset_y).into()) {
            self.current += offset_y;
            /*   println!("prev: {prev}; offset is: {offset_y}; result is: {}", self.current)*/
        } else if offset_x != IVec2::ZERO && self.bounds.contains((self.current + offset_x).into()) {
            if self.direction.y > 0 {
                self.current.y = self.bounds.min.y;
            } else {
                self.current.y = self.bounds.max.y;
            }
            self.current += offset_x;
            /*  println!("prev: {prev}; offset is: {offset_x}; result is: {}", self.current)*/
        } else {
            return None;// Finished iterating when the offsets are out of bounds
        }

        Some(self.current)
    }
}

pub struct CoordinateIterator {
    inner: std::vec::IntoIter<CellIndex2d>,
}

impl CoordinateIterator {
    pub fn new(start_i: u32, end_i: u32, start_j: u32, end_j: u32) -> Self {
        let data: Vec<CellIndex2d> = (start_j..end_j).flat_map(move |j| (start_i..end_i).map(move |i| CellIndex2d { x: i, y: j })).collect();
        let inner = data.into_iter();
        Self { inner }
    }

    pub fn iter_area_in_line_from(start: CellIndex2d, direction: IVec2, area: URect) -> AreaLineIterator {
        AreaLineIterator {
            bounds: area,
            current: start,
            direction,
        }
    }

    pub fn iter_area_fully_from(start: CellIndex2d, direction: IVec2, area: URect) -> AreaFullIterator {
        AreaFullIterator {
            bounds: area,
            current: start,
            direction,
        }
    }
}

impl Iterator for CoordinateIterator {
    type Item = CellIndex2d;

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
    pub fn iter_coordinates(&self) -> CoordinateIterator {
        CoordinateIterator::new(0, self.column_number, 0, self.row_number)
    }

    pub fn iter_coordinates_range(&self, min: UVec2, max: UVec2) -> CoordinateIterator {
        assert!(max.x <= self.column_number);
        assert!(max.y <= self.row_number);
        CoordinateIterator::new(min.x, max.x, min.y, max.y)
    }

    pub fn iter_coordinates_in_area(&self, area: URect) -> CoordinateIterator {
        assert!(area.max.x <= self.column_number);
        assert!(area.max.y <= self.row_number);
        CoordinateIterator::new(area.min.x, area.max.x, area.min.y, area.max.y)
    }

    pub fn new(column_number: u32, row_number: u32, cell_size: Vec2) -> Self {
        let grid_size = Vec2::new(column_number as f32 * cell_size.x, row_number as f32 * cell_size.y);
        let mut grid = Grid2D {
            column_number,
            row_number,
            cell_size,
            grid_size,
            cells_spacing: 0.0,
            rect: Rect::from_center_size(Vec2::ZERO, grid_size),
            max_column_index: column_number - 1,
            max_row_index: row_number - 1,
            indexes: Default::default(),
        };
        grid.indexes = grid.get_indexes();
        return grid;
    }

    pub fn calculate_area_clamped_from_center(&self, center: CellIndex2d, size: UVec2) -> URect {
        // Calculate half sizes
        let half_width = size.x / 2;
        let half_height = size.y / 2;

        // Calculate top left cell positions ensuring it doesn't go beyond grid boundaries
        let min_x = center.x.saturating_sub(half_width);
        let min_y = center.y.saturating_sub(half_height);

        // Calculate bottom right cell positions ensuring it doesn't go beyond grid boundaries
        let max_x = (center.x + half_width).min(self.max_column_index);
        let max_y = (center.y + half_height).min(self.max_row_index);

        // Creating the URect
        let rect = URect::from_corners(UVec2::new(min_x, min_y), UVec2::new(max_x, max_y));

        rect
    }

    pub fn calculate_area_from(&self, point: CellIndex2d, in_direction: IVec2, num_cells: u32) -> URect {
        // Convert direction vectors to positive u32, clamping negative values to 0
        let direction_x = (in_direction.x.max(0) as u32) * num_cells;
        let direction_y = (in_direction.y.max(0) as u32) * num_cells;

        // Add the direction vector to the point to get bottom right cell positions,
        // ensuring it doesn't go beyond grid boundaries. Subtract 1 to account for zero based indexing.
        let max_x = (point.x as u32 + direction_x).min(self.max_column_index);
        let max_y = (point.y as u32 + direction_y).min(self.max_row_index);

        // Creating the URect
        let rect = URect::from_corners(point.into(), UVec2::new(max_x, max_y));

        rect
    }

    pub fn calculate_line_from(&self, point: CellIndex2d, in_direction: IVec2, num_cells: u32) -> URect {
        // Calculate the new positions in i32 to allow for negative changes
        let new_x = (point.x as i32 + (in_direction.x * num_cells as i32)).clamp(0, self.max_column_index as i32) as u32;
        let new_y = (point.y as i32 + (in_direction.y * num_cells as i32)).clamp(0, self.max_row_index as i32) as u32;

        // Creating the URect
        URect::from_corners(point.into(), UVec2::new(new_x, new_y))
    }

    fn get_indexes(&self) -> Array2<CellIndex2d> {
        let mut arr = Array2::from_elem((self.row_number as usize,
                                         self.column_number as usize), CellIndex2d { x: 0, y: 0 });

        for ((col, row), cell_index_2d) in arr.indexed_iter_mut() {
            *cell_index_2d = CellIndex2d {
                x: col as CellIndex1d,
                y: row as CellIndex1d,
            };
        }

        arr
    }

    pub fn get_indexes_segment(&self, area: URect) -> ArrayView2<CellIndex2d> {
        slice_2d_array(&self.indexes, area)
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
    pub fn calc_cell_index_1d_at(&self, cell_index2d: CellIndex2d) -> CellIndex1d {
        calculate_1d_index(cell_index2d, self.column_number)
    }

    #[inline]
    pub fn calc_cell_index_2d_at(&self, cell_index1d: CellIndex1d) -> CellIndex2d {
        calculate_2d_index(cell_index1d, self.column_number)
    }

    #[inline]
    pub fn form_grid_bound_cell_index(&self, cell_index_x: u32, cell_index_y: u32) -> CellIndex2d {
        CellIndex2d::new(cell_index_x.clamp(0u32, self.max_column_index), cell_index_y.clamp(0u32, self.max_row_index))
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from(&self, cell_index2d: CellIndex2d) -> SurfaceCoordinate
    {
        let latitude: Coordinate = cell_index2d.x as f32 / self.max_column_index as f32;
        let longitude: Coordinate = cell_index2d.y as f32 / self.max_row_index as f32;
        SurfaceCoordinate { latitude, longitude }
    }

    #[inline]
    pub fn is_cell_index_in_grid_bounds(&self, cell_index: CellIndex2d) -> bool {
        u32::from(cell_index.x) < self.column_number && u32::from(cell_index.y) < self.row_number
    }

    #[inline]
    pub fn is_position_in_grid_bounds(&self, position: Vec2) -> bool {
        self.rect.contains(position)
    }
}