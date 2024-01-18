use bevy::math::{Rect, UVec2, Vec2};
use bevy::prelude::{Color, Component, Resource};

#[derive(Resource)]
pub struct GridParameters {
    pub column_number: u32,
    pub row_number: u32,
    pub cell_size: Vec2,
    pub grid_size: Vec2,
    pub cells_spacing: f32,
    pub rect: Rect,
    pub max_row_index: u32,
    pub max_column_index: u32,
}

#[derive(Clone, Default)]
pub struct GridCellData {
    pub color: Color,
}

#[derive(Resource)]
pub struct GridRelatedData {
    pub(crate) data: Vec<GridCellData>,
}

#[derive(Component, Clone, Copy, Default)]
pub struct CellIndex {
    pub index: UVec2,
}

impl CellIndex {
    pub fn new(index: UVec2) -> Self {
        Self { index }
    }
}

impl From<UVec2> for CellIndex {
    fn from(item: UVec2) -> Self {
        CellIndex {
            index: item,
        }
    }
}

#[derive(Component)]
pub struct GridCellTag;