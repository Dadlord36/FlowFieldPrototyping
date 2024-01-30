use bevy::{
    math::Vec2,
    prelude::{Component, Resource}
};
use ndarray::Array2;

use crate::components::grid_components::CellIndex2d;

#[derive(Component)]
pub struct Arrow;

#[derive(Resource)]
pub struct FlowField {
    pub field: Array2<Vec2>,
}

#[derive(Default)]
pub struct ExplosionParameters
{
    pub impact_center_cell_index: CellIndex2d,
    pub impact_radius: f32,
}