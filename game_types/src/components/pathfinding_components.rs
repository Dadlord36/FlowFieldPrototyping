use std::{
    borrow::Borrow,
    ops::Index,
};

use bevy::prelude::{Component, URect};
use derive_more::Constructor;
use ndarray::ArrayView2;

use crate::components::grid_components::definitions::{CellIndex2d, GridCellData, GridSegment};

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
}


#[derive(Component, Constructor)]
pub struct PathfindingMap<'a> {
    pub(super) grid_segment: GridSegment,
    grid_segment_data: ArrayView2<'a, GridCellData>,
    pub(super) area: URect,
}

impl Index<CellIndex2d> for PathfindingMap<'_> {
    type Output = GridCellData;

    fn index(&self, index: CellIndex2d) -> &Self::Output {
        self.grid_segment_data[&index].borrow()
    }
}

#[derive(Component, Constructor, Clone, Default)]
pub struct MovementSpeed {
    pub value: f32,
}

