use std::fmt::{Display, Formatter};

use bevy::math::{Quat, UVec2, Vec2, Vec3};
use bevy::prelude::Transform;

use crate::components::{
    grid_components::GridParameters,
    movement_components::{
        CoordinateBounds, SurfaceCoordinate,
    },
};

const MAX_BOUND_ANGLE: f32 = 180.0;
const FULL_BOUND_ANGLE: f32 = MAX_BOUND_ANGLE * 2.0;
const COORDINATE_BOUNDS: CoordinateBounds = CoordinateBounds { min: -MAX_BOUND_ANGLE, max: MAX_BOUND_ANGLE };


impl Display for SurfaceCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude, )
    }
}

impl SurfaceCoordinate {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        SurfaceCoordinate { latitude, longitude }
    }
    pub fn set_latitude(&mut self, value: f32) {
        self.latitude = value.clamp(COORDINATE_BOUNDS.min, COORDINATE_BOUNDS.max);
    }

    pub fn set_longitude(&mut self, value: f32) {
        self.longitude = value.clamp(COORDINATE_BOUNDS.min, COORDINATE_BOUNDS.max);
    }

    #[inline]
    pub fn adjust_coordinate(&mut self, offset: Vec2) {
        self.latitude = wrap_value(self.latitude + offset.x, COORDINATE_BOUNDS.min, COORDINATE_BOUNDS.max);
        self.longitude = wrap_value(self.longitude + offset.y, COORDINATE_BOUNDS.min, COORDINATE_BOUNDS.max);
    }

    #[inline]
    pub fn calculate_cell_index_on_flat_surface(&self, grid_parameters: &GridParameters) -> UVec2 {
        // Scale factor for a single step in grid coordinates
        let scale_factor = FULL_BOUND_ANGLE / (grid_parameters.max_column_index as f32);

        let cell_index_x = (((self.latitude + MAX_BOUND_ANGLE) / scale_factor).round() as u32).min(grid_parameters.max_column_index);
        let cell_index_y = (((self.longitude + MAX_BOUND_ANGLE) / scale_factor).round() as u32).min(grid_parameters.max_row_index);

        grid_parameters.form_grid_bound_cell_index(cell_index_x, cell_index_y)
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from(grid_parameters: &GridParameters, cell_index2d: UVec2) -> Self
    {
        // Scale factor for a single step in grid coordinates
        let scale_factor = FULL_BOUND_ANGLE / (grid_parameters.max_column_index as f32);

        let latitude = cell_index2d.x as f32 * scale_factor - MAX_BOUND_ANGLE;
        let longitude = cell_index2d.y as f32 * scale_factor - MAX_BOUND_ANGLE;

        Self { latitude, longitude }
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from_pos(grid_parameters: &GridParameters, world_position: Vec2) -> Self {
        let hovered_cell_index = grid_parameters.calculate_cell_index_from_position(world_position);
        return SurfaceCoordinate::calculate_flat_surface_coordinate_from(grid_parameters, hovered_cell_index);
    }

    #[inline]
    pub fn project_surface_coordinate_on_grid(&self, grid: &GridParameters) -> Transform {
        // MAX_BOUND_ANGLE is 180.0 and FULL_BOUND_ANGLE is 360.0
        // Convert [-180, 180] to [0, 360] and then find the proportional position within the grid
        let grid_width = grid.rect.width() - 1.0;
        let grid_height = grid.rect.height() - 1.0;

        let proportional_latitude = (self.latitude + MAX_BOUND_ANGLE) * grid_width / FULL_BOUND_ANGLE;
        let proportional_longitude = (self.longitude + MAX_BOUND_ANGLE) * grid_height / FULL_BOUND_ANGLE;

        // Adjust the proportional position with the grid offset and add half cell size to move the position to the cell center
        let x = proportional_latitude + grid.rect.min.x + grid.cell_size.x / 2.0;
        let y = proportional_longitude + grid.rect.min.y + grid.cell_size.y / 2.0;

        let position = Vec2::new(x, y);  // z is 0 for a flat surface

        // Since we're working with a flat grid, translation is straightforward and doesn't require rotation or scaling.
        let transform = Transform {
            translation: position.extend(0.0),
            rotation: Quat::IDENTITY,  // No rotation
            scale: Vec3::ONE,  // No scaling
        };

        transform
    }
}

#[inline]
pub fn wrap_value(value: f32, min_value: f32, max_value: f32) -> f32 {
    let range = max_value - min_value;
    let value = repeat(value - min_value, range) + min_value;
    value
}

#[inline]
pub fn repeat(t: f32, length: f32) -> f32 {
    (t - ((t / length).floor() * length)).clamp(0.0, length)
}

