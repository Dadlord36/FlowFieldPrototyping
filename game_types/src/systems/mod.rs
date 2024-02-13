use bevy::math::UVec2;

pub mod flow_driven_movement;
pub mod flow_field_manipulations;
pub mod grid_related;
pub mod selection_related;

const CELLS_IN_FRONT: u32 = 5;
const PATHFINDING_RECT: UVec2 = UVec2::new(10, 10);