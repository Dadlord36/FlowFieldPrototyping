use bevy::{
    math::{Rect, Vec2},
    prelude::{Color, Component, Resource},
};
use bevy::core::Zeroable;
use derive_more::{AsRef, Constructor, Add, From, Into, Rem, Sub, AddAssign};
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

#[derive(Clone, Copy, Default, Add, AddAssign, Sub, Rem, From, Into, Eq, PartialEq)]
pub struct CellIndex2d {
    pub x: CellIndex1d,
    pub y: CellIndex1d,
}

#[derive(Resource, Clone)]
pub struct Grid2D {
    pub column_number: u32,
    pub row_number: u32,
    pub cell_size: Vec2,
    pub grid_size: Vec2,
    pub cells_spacing: f32,
    pub rect: Rect,
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

#[derive(Component, Clone, Copy, Default, AsRef, Constructor, From, Into)]
pub struct CellIndex {
    pub index: CellIndex2d,
}

#[derive(Component)]
pub struct GridCellTag;
