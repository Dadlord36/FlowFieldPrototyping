use std::{
    borrow::Borrow,
    ops::Index,
};

use bevy::prelude::{Component, URect};
use colored::{ColoredString, Colorize};
use derive_more::Constructor;
use ndarray::{ArrayView2, Ix2};
use ndarray::iter::IndexedIter;

use crate::components::grid_components::definitions::{CellIndex2d, Grid2D, GridCellData, GridRelatedData, GridSegment, Occupation};

pub enum CoordinateType {
    Normalized,
    Local,
    Global,
}

/// A struct representing a pathfinder. Coordinates should always be normalized.
#[derive(Component, Constructor, Copy, Clone)]
pub struct Pathfinder {
    pub start: CellIndex2d,
    pub end: CellIndex2d,
}

impl Pathfinder {
    pub const ZERO: Self = Pathfinder { start: CellIndex2d::ZERO, end: CellIndex2d::ZERO };

    pub fn visualize_path_on_grid(&self, grid: &Grid2D, grid_related_data: &GridRelatedData,
                                  path: &Vec<CellIndex2d>) {
        println!("{}", "Visualizing grid...".yellow());

        let mut output: Vec<ColoredString> = Vec::new();
        for row in (0..grid.row_number).rev() {
            for col in 0..grid.column_number {
                let cell_index2d = CellIndex2d::new(col, row);
                let cell_related_data = grid_related_data.get_data_at(&cell_index2d);
                let is_in_path = path.contains(&cell_index2d);
                let cell_repr: ColoredString = self.determine_cell_type(cell_index2d, cell_related_data,
                                                                        is_in_path);
                output.push(format!("|{}| ", cell_repr).normal());
            }
            output.push("\n".normal());
        }

        for colored_string in output {
            print!("{}", colored_string);
        }
    }

    fn determine_cell_type(&self, cell_index2d: CellIndex2d,
                           cell_related_data: &GridCellData, is_in_path: bool) -> ColoredString {
        let cell_repr: ColoredString =
            if cell_index2d == self.start {
                "S".green()
            } else if cell_index2d == self.end {
                "T".red()
            } else if is_in_path {
                "P".blue()  // Path
            } else if cell_related_data.occupation_state == Occupation::Occupied {
                "O".black()  // Obstacle
            } else {
                "E".bright_black()  // Empty cell
            };
        cell_repr
    }
}


#[derive(Component, Constructor)]
pub struct PathfindingMap<'a> {
    pub(super) grid_segment: GridSegment,
    grid_segment_data: ArrayView2<'a, GridCellData>,
    pub(super) area: URect,
    pub(super) area_normalized: URect,
}

impl Index<CellIndex2d> for PathfindingMap<'_> {
    type Output = GridCellData;

    fn index(&self, index: CellIndex2d) -> &Self::Output {
        self.grid_segment_data[&index].borrow()
    }
}

impl<'a> PathfindingMap<'a> {
    pub fn iter_segment_data_indexed(&self) -> IndexedIter<'_, GridCellData, Ix2> {
        self.grid_segment_data.indexed_iter()
    }
    pub fn is_valid_index(&self, cell_index2d: &CellIndex2d) -> bool {
        self.grid_segment_data.get(cell_index2d).is_some()
    }
}

#[derive(Component, Constructor, Clone, Default)]
pub struct MovementSpeed {
    pub value: f32,
}

