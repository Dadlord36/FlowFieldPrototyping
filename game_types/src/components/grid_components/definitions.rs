use bevy::{
    math::{
        IVec2,
        Rect,
        Vec2,
    },
    prelude::{
        Color,
        Component,
        Resource,
        URect,
    },
};
use bevy::prelude::UVec2;
use bevy::utils::HashMap;
use derive_more::{Add, AddAssign, AsRef, Constructor, Display, From, Into, Rem, Sub};
use ndarray::Array2;
use crate::components::directions::Direction;

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

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
Add, AddAssign, Sub, Rem, From, Into)]
pub struct CellIndex2d {
    pub x: CellIndex1d,
    pub y: CellIndex1d,
}

#[derive(Copy, Clone, Default, Display)]
#[display("parent_grid: {:?};  \
normalized_grid: {:?}", parent_grid, bounds)]
pub struct GridSegment {
    pub(super) parent_grid: URect,
    // Offset from parent grid to segment grid
    pub(super) offset: IVec2,
    pub(super) bounds: URect,
}

impl GridSegment {
    pub fn get_offset(&self) -> IVec2 {
        self.offset
    }
}

#[derive(Resource, Copy, Clone, Default)]
pub struct ElapsedTimeTracker {
    pub time_stamp: f32,
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
    pub(crate) segments: HashMap<Direction, URect>,
}

#[derive(Clone, Default)]
pub struct GridCellData {
    pub color: Color,
    pub occupation_state: Occupation,
    pub detraction_factor: f32,
}

#[derive(Resource, Clone)]
pub struct ObstaclesParameters {
    pub influence_area: UVec2,
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
