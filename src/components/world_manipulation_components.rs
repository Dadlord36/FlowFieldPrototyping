use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Component, Resource};

#[derive(Resource, Default)]
pub struct HoverCell {
    pub hovered_cell: UVec2,
}

#[derive(Resource)]
pub struct CursorWorldPosition {
    pub position: Vec2,
}

impl Default for CursorWorldPosition {
    fn default() -> Self {
        CursorWorldPosition {
            position: Vec2::ZERO
        }
    }
}

#[derive(Component)]
pub struct SelectedCell;