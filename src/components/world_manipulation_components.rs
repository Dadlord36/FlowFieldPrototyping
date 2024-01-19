use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Component, Resource};

#[derive(Resource, Default)]
pub struct HoverCell {
    pub hovered_cell: UVec2,
}

#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Vec2,
}

#[derive(Component)]
pub struct SelectedCell;