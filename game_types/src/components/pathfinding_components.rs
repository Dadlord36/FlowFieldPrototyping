use std::{
    borrow::Borrow,
    ops::Index,
};

use bevy::prelude::{Component, URect};
use colored::{ColoredString, Colorize};
use derive_more::Constructor;
use ndarray::{
    ArrayView2,
    Ix2,
    iter::IndexedIter,
};

use crate::components::grid_components::definitions::{CellIndex2d, Grid2D, GridCellData,
                                                      GridRelatedData, GridSegment, Occupation};

pub enum CoordinateType {
    Normalized,
    Local,
    Global,
}

/// A struct representing a pathfinder. Coordinates should always be normalized.
#[derive(Component, Constructor, Default, Copy, Clone, Eq, PartialEq)]
pub struct Pathfinder {
    pub start: CellIndex2d,
    pub end: CellIndex2d,
}

impl Pathfinder {
    pub const ZERO: Self = Pathfinder { start: CellIndex2d::ZERO, end: CellIndex2d::ZERO };
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

