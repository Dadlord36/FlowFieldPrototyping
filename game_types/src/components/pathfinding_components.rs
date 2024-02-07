use bevy::prelude::{Component, Resource, URect};
use derive_more::Constructor;
use ndarray::ArrayView2;

use crate::components::grid_components::definitions::{CellIndex2d, GridCellData};

#[derive(Component, Constructor)]
pub struct Pathfinder {
    pub start: CellIndex2d,
    pub end: CellIndex2d,
}

impl Pathfinder {
    pub const ZERO: Self = Pathfinder { start: CellIndex2d::ZERO, end: CellIndex2d::ZERO };
}


#[derive(Resource, Constructor)]
pub struct PathfindingMap<'a> {
    pub(crate) grid_segment_data: ArrayView2<'a, GridCellData>,
    pub(crate) area: URect,
}

