use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct Arrow;

#[derive(Resource)]
pub struct FlowField {
    pub(crate) field: Vec<Vec2>,
}

#[derive(Default)]
pub struct ExplosionParameters
{
    pub impact_center_cell_index: UVec2,
    pub impact_radius: f32,
}