use bevy::math::Vec2;
use crate::components::grid_components::definitions::Grid2D;

pub fn construct_default_grid() -> Grid2D {
    let grid_parameters = Grid2D::new(25, 25, Vec2::new(50f32, 50f32));
    grid_parameters
}