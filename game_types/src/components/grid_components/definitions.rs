use bevy::{
    math::{Rect, Vec2},
    prelude::{
        Color,
        Component,
        Resource,
        URect,
    },
};
use derive_more::{Add, AddAssign, AsRef, Constructor, Display, From, Into, Rem, Sub};
use ndarray::Array2;

pub type CellIndex1d = u32;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Occupation {
    Free,
    Occupied,
}

impl Default for Occupation {
    fn default() -> Self {
        Occupation::Free
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Add, AddAssign, Sub, Rem, From, Into)]
pub struct CellIndex2d {
    pub x: CellIndex1d,
    pub y: CellIndex1d,
}

#[derive(Copy, Clone, Default, Display)]
#[display("parent_grid: {:?};  \
segment_grid: {:?}", parent_grid, segment_grid)]
pub struct GridSegment {
    pub(super) parent_grid: URect,
    pub(super) segment_grid: URect,
    pub(super) bounds: URect,
}

#[derive(Resource, Clone)]
pub struct Grid2D {
    pub column_number: u32,
    pub row_number: u32,
    pub cell_size: Vec2,
    pub grid_size: Vec2,
    pub cells_spacing: f32,
    pub shape_rect: Rect,
    pub indexes_rect: URect,
    pub max_row_index: u32,
    pub max_column_index: u32,
    pub(crate) indexes: Array2<CellIndex2d>,
}


#[derive(Clone, Default)]
pub struct GridCellData {
    pub color: Color,
    pub occupation_state: Occupation,
}

#[derive(Resource)]
pub struct GridRelatedData {
    pub(super) data: Array2<GridCellData>,
}

#[derive(Clone, Copy, Default, Component, AsRef, Constructor, From, Into)]
pub struct CellIndex {
    pub index: CellIndex2d,
}

#[derive(Component)]
pub struct GridCellTag;
