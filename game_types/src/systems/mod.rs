use bevy::math::UVec2;

pub mod flow_driven_movement;
pub mod flow_field_manipulations;
pub mod grid_related;
pub mod selection_related;

const PATHFINDING_RECT: UVec2 = UVec2::new(8, 8);
const CELLS_IN_FRONT: u32 = PATHFINDING_RECT.x / 3;