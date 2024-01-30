use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};

use crate::components::grid_components::CellIndex2d;

#[derive(Resource, Default)]
pub struct HoverCell {
    pub hovered_cell: CellIndex2d,
}

#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Vec2,
}

#[derive(Component)]
pub struct SelectedCell;