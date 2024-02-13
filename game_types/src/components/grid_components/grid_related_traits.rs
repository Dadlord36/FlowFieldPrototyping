use std::{
    fmt::{self, Debug, Formatter},
    ops::{
        Add,
        Index,
        IndexMut,
        Mul,
        Sub,
    },
};

use bevy::{
    math::URect,
    prelude::{
        IVec2,
        UVec2,
        Vec2,
    },
    render::color::Color,
};
use derive_more::AddAssign;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, IndexLonger, Ix2, NdIndex};
use num_traits::AsPrimitive;
use rand::Rng;

use crate::{
    components::{
        grid_components::definitions::Occupation,
        pathfinding_components::PathfindingMap,
    },
    function_libs::grid_calculations::{
        slice_2d_array,
        slice_2d_array_mut,
    },
};
use crate::function_libs::grid_calculations;

use super::definitions::{CellIndex1d, CellIndex2d, Grid2D, GridCellData, GridRelatedData, GridSegment};

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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

impl GridRelatedData {
    pub fn new(grid_parameters: &Grid2D) -> Self {
        GridRelatedData { data: Array2::default((grid_parameters.column_number as usize, grid_parameters.row_number as usize)) }
    }

    pub fn create_pathfinding_map_on(&self, target_grid: &Grid2D, inclusive_rect: URect) -> PathfindingMap {
        let slice = self.get_segment_view_of(inclusive_rect);
        PathfindingMap::new(target_grid.form_segment_for(inclusive_rect), slice, inclusive_rect)
    }

    pub fn has_obstacle_in(&self, area: URect) -> bool {
        let mut has = false;
        for cell_data in self.get_segment_view_of(area) {
            if cell_data.occupation_state == Occupation::Occupied {
                has = true;
                break;
            }
        }

        return has;
    }

    pub fn get_segment_view_of(&self, area: URect) -> ArrayView2<GridCellData> {
        slice_2d_array(&self.data, area)
    }

    pub fn get_segment_mut_view_of(&mut self, area: URect) -> ArrayViewMut2<GridCellData> {
        slice_2d_array_mut(&mut self.data, area)
    }

    pub fn set_color_for_area(&mut self, area: URect, color: Color) {
        let mut segment_view = self.get_segment_mut_view_of(area);
        for cell_data in segment_view.iter_mut() {
            cell_data.color = color;
        }
    }

    pub fn fill_with_random_obstacle_pattern(&mut self, grid_parameters: &Grid2D) {
        const BORDER_RANGE: usize = 5;
        let mut rng = rand::thread_rng();
        let shape = (grid_parameters.row_number as usize, grid_parameters.column_number as usize);
        self.data = Array2::from_shape_fn(shape, |idx| {
            let (row, column) = idx;
            let occupation = if row < BORDER_RANGE
                || column < BORDER_RANGE
                || row >= shape.0 - BORDER_RANGE
                || column >= shape.1 - BORDER_RANGE
            {
                Occupation::Free
            } else if rng.gen_bool(0.75) {
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

impl Index<CellIndex2d> for GridRelatedData {
    type Output = GridCellData;

    fn index(&self, index: CellIndex2d) -> &Self::Output {
        self.get_data_at(&index)
    }
}

impl IndexMut<CellIndex2d> for GridRelatedData {
    fn index_mut(&mut self, index: CellIndex2d) -> &mut Self::Output {
        self.get_data_at_mut(&index)
    }
}

impl GridSegment {
    pub fn to_local_index(&self, global_cell_index2d: CellIndex2d) -> CellIndex2d {
        let local_cell_index2d = CellIndex2d {
            x: global_cell_index2d.x - self.segment_grid.min.x,
            y: global_cell_index2d.y - self.segment_grid.min.y,
        };
        local_cell_index2d
    }

    pub fn from_local_to_global_index(&self, local_cell_index2d: CellIndex2d) -> CellIndex2d {
        let global_cell_index = CellIndex2d {
            x: local_cell_index2d.x + self.segment_grid.min.x,
            y: local_cell_index2d.y + self.segment_grid.min.y,
        };
        global_cell_index
    }

    pub fn from_normalized_to_local_index(&self, normalized_index: CellIndex2d) -> CellIndex2d {
        let origin: CellIndex2d = self.segment_grid.min.into();
        let local_cell_index2d = normalized_index + origin;
        local_cell_index2d
    }

    pub fn from_normalized_to_global_index(&self, normalized_index: CellIndex2d) -> CellIndex2d {
        let local_cell_index2d = self.from_normalized_to_local_index(normalized_index);
        self.from_local_to_global_index(local_cell_index2d)
    }

    pub fn from_local_to_normalized_index(&self, local_cell_index2d: CellIndex2d) -> CellIndex2d {
        let origin: CellIndex2d = self.segment_grid.min.into();
        let normalized_index = local_cell_index2d - origin;
        normalized_index
    }

    pub fn from_global_to_normalized_index(&self, global_cell_index2d: CellIndex2d) -> CellIndex2d {
        let local_cell_index2d = self.to_local_index(global_cell_index2d);
        self.from_local_to_normalized_index(local_cell_index2d)
    }

    //Calculate the line of cell in direction starting from cell:
    pub fn calculate_line_of_cells_in_direction(
        &self,
        starting_cell: CellIndex2d,
        direction: IVec2,
        length: u32,
    ) -> URect {
        let starting_cell_index = self.to_local_index(starting_cell);
        let ending_cell_index = starting_cell_index + (direction * (length - 1) as i32);
        URect::from_corners(starting_cell_index.into(), ending_cell_index.into())
    }

    #[inline]
    pub fn normalized_to_global_index(&self, normalized_cell_index1d: CellIndex1d) -> CellIndex2d {
        let normalized_cell_index_2d = grid_calculations::calculate_2d_index(normalized_cell_index1d,
                                                                             self.bounds.width());
        self.from_normalized_to_global_index(normalized_cell_index_2d)
    }

    #[inline]
    pub fn contains_global(&self, cell_index2d: CellIndex2d) -> bool {
        self.parent_grid.contains(cell_index2d.into())
    }

    #[inline]
    pub fn contains_local(&self, cell_index2d: CellIndex2d) -> bool {
        self.segment_grid.contains(cell_index2d.into())
    }

    #[inline]
    pub fn contains_normalized(&self, cell_index2d: CellIndex2d) -> bool {
        self.bounds.contains(cell_index2d.into())
    }
}