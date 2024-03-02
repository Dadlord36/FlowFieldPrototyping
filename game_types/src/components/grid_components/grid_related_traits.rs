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
use bevy::log::info;
use colored::{ColoredString, Colorize};
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
use crate::function_libs::grid_calculations::normalize_rect;

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

impl Sub<IVec2> for CellIndex2d {
    type Output = CellIndex2d;

    fn sub(self, rhs: IVec2) -> Self::Output {
        CellIndex2d {
            x: (self.x as i32 - rhs.x) as u32,
            y: (self.y as i32 - rhs.y) as u32,
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

    pub fn distance(&self, other: &CellIndex2d) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn euclidean_distance(&self, other: &CellIndex2d) -> f32 {
        let x_distance = other.x as f64 - self.x as f64;
        let y_distance = other.y as f64 - self.y as f64;

        (x_distance.powi(2) + y_distance.powi(2)).sqrt() as f32
    }

    pub fn inverse_chebyshev_distance(&self, other: &CellIndex2d) -> f32 {
        let x_distance = (other.x as f64 - self.x as f64).abs();
        let y_distance = (other.y as f64 - self.y as f64).abs();

        let distance = x_distance.max(y_distance) as f32;

        // Check if distance is very small to prevent division by zero errors
        return if distance.abs() < f32::EPSILON {
            0.0
        } else {
            1.0 / distance
        };
    }

    pub fn normalize(&self) -> Self {
        let length = ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt() as u32;
        Self {
            x: self.x / length,
            y: self.y / length,
        }
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
        GridRelatedData {
            data: Array2::default((grid_parameters.column_number as usize,
                                   grid_parameters.row_number as usize))
        }
    }

    pub fn create_pathfinding_map_on(&self, target_grid: &Grid2D, inclusive_rect: URect) -> PathfindingMap {
        let slice = self.get_segment_view_of(inclusive_rect);
        PathfindingMap::new(target_grid.form_segment_for(inclusive_rect), slice, inclusive_rect,
                            normalize_rect(inclusive_rect))
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

    pub fn set_color_for_index(&mut self, cell_index2d: &CellIndex2d, color: Color) {
        self.data[cell_index2d].color = color
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

    pub fn set_color_at(&mut self, cell_index2d: CellIndex2d, color: Color) {
        self[cell_index2d].color = color;
    }

    pub fn fill_with_random_obstacle_pattern(&mut self, grid_parameters: &Grid2D) {
        const BORDER_RANGE: usize = 5;
        let mut rng = rand::thread_rng();
        let shape = (grid_parameters.row_number as usize,
                     grid_parameters.column_number as usize);
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
                detraction_factor: 0.0,
            }
        });
    }

    pub fn visualize_on_grid(&self, grid: &Grid2D) {
        println!("{}", "Visualizing grid...".yellow());

        let mut output: Vec<ColoredString> = Vec::new();
        for row in (0..grid.row_number).rev() {
            for col in 0..grid.column_number {
                let cell_index2d = CellIndex2d::new(col, row);
                let cell_related_data = self.get_data_at(&cell_index2d);
                let cell_repr: ColoredString = self.determine_cell_type(cell_related_data);
                output.push(format!("|{}| ", cell_repr).normal());
            }
            output.push("\n".normal());
        }

        for colored_string in output {
            print!("{}", colored_string);
        }
    }

    fn determine_cell_type(&self, cell_related_data: &GridCellData) -> ColoredString {
        let cell_repr: ColoredString =
            if cell_related_data.occupation_state == Occupation::Occupied {
                " O ".black()  // Obstacle
            } else if cell_related_data.detraction_factor > 0.0 {
                let number = format!("{:.1}", cell_related_data.detraction_factor);
                number.black()
            } else if cell_related_data.occupation_state == Occupation::Temp {
                " T ".black()
            } else {
                " E ".bright_black()  // Empty cell
            };
        cell_repr
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
    pub fn new(parent: URect, child: URect) -> Self {
        let offset = IVec2 {
            x: (child.min.x - parent.min.x) as i32,
            y: (child.min.y - parent.min.y) as i32,
        };

        let bounds = URect::from_corners(UVec2::ZERO, child.size());

        Self {
            parent_grid: parent,
            offset,
            bounds,
        }
    }

    // Transform from global index to local index
    pub fn global_to_local_index(&self, global_index: CellIndex2d) -> CellIndex2d {
        CellIndex2d {
            x: (global_index.x as i32 - self.offset.x) as CellIndex1d,
            y: (global_index.y as i32 - self.offset.y) as CellIndex1d,
        }
    }

    // Transform from local index to global index
    pub fn local_to_global_index(&self, local_index: CellIndex2d) -> CellIndex2d {
        CellIndex2d {
            x: (local_index.x as i32 + self.offset.x) as CellIndex1d,
            y: (local_index.y as i32 + self.offset.y) as CellIndex1d,
        }
    }

    pub fn convert_1d_to_2d(&self, index: CellIndex1d) -> CellIndex2d {
        grid_calculations::calculate_2d_index(index, self.bounds.width())
    }

    // calculate_line_of_cells_in_direction
    pub fn calculate_line_of_cells_in_direction(
        &self,
        start_cell: CellIndex2d,
        direction: IVec2,
        length: u32,
    ) -> URect {
        let starting_cell_index = self.global_to_local_index(start_cell);
        let ending_cell_index = starting_cell_index + (direction * (length - 1) as i32);
        URect::from_corners(starting_cell_index.into(), ending_cell_index.into())
    }

    // contains_global
    #[inline]
    pub fn contains_global(&self, cell_index2d: CellIndex2d) -> bool {
        let local_index = self.global_to_local_index(cell_index2d);
        self.contains_local(local_index)
    }

    // contains_local
    #[inline]
    pub fn contains_local(&self, index: CellIndex2d) -> bool {
        self.bounds.contains(index.into())
    }
}