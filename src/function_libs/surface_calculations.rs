use std::ops::{Range, RangeInclusive};
use bevy::math::{Mat4, Quat, UVec2, Vec2, Vec3};
use bevy::prelude::{Component, Resource, Transform};
use crate::{
    function_libs::grid_calculations,
    function_libs::grid_calculations::GridParameters,
};

#[derive(Resource, Clone, Default)]
pub struct CylinderParameters {
    radius: f32,
    height: f32,
}

const COORDINATE_BOUNDS: CoordinateBounds = CoordinateBounds { min: -180.0, max: 180.0 };

struct CoordinateBounds {
    min: f32,
    max: f32,
}

#[derive(Component, Clone, Default)]
pub struct SurfaceCoordinate {
    latitude: f32,
    longitude: f32,

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
    pub fn calculate_flat_surface_coordinate_from(
        grid_parameters: &GridParameters,
        cell_index2d: UVec2,
    ) -> Self {
        // Convert cell index to a relative position (0..1) in the rect along each axis
        let rel_pos_x = (cell_index2d.x as f32) / (grid_parameters.column_number as f32 - 1.0);
        let rel_pos_y = (cell_index2d.y as f32) / (grid_parameters.row_number as f32 - 1.0);

        // Calculate world coordinates by mapping relative position to rect dimensions
        let world_x = grid_parameters.rect.min.x + rel_pos_x * (grid_parameters.rect.width());
        let world_y = grid_parameters.rect.min.y + rel_pos_y * (grid_parameters.rect.height());

        // Map world coordinates to SurfaceCoordinate, scaled from [rect.min, rect.max] to [-180, 180]
        let scale_x = 360.0 / grid_parameters.rect.width();
        let scale_y = 360.0 / grid_parameters.rect.height();

        let latitude = (world_y - grid_parameters.rect.min.y) * scale_y - 180.0;
        let longitude = (world_x - grid_parameters.rect.min.x) * scale_x - 180.0;

        SurfaceCoordinate {
            latitude,
            longitude
        }
    }

    #[inline]
    pub fn calculate_cylinder_surface_coordinate_from(
        grid_parameters: &GridParameters,
        cell_index2d: UVec2,
    ) -> SurfaceCoordinate {
        let normalized_row_index: f32 = (cell_index2d.x / grid_parameters.max_row_index) as f32; // normalize to [0, 1]
        let normalized_column_index: f32 = (cell_index2d.y / grid_parameters.max_column_index) as f32; // normalize to [0, 1]

        let half_pi = std::f32::consts::FRAC_PI_2;
        let pi = std::f32::consts::PI;

        let latitude = (normalized_row_index * 2.0 - 1.0) * half_pi; // scale to [-HalfPi, HalfPi]
        let longitude = (normalized_column_index * 2.0 - 1.0) * pi; // scale to [-Pi, Pi]

        SurfaceCoordinate::new(latitude, longitude)
    }

    #[inline]
    pub fn project_surface_coordinate_on_grid(&self, grid: &GridParameters) -> Transform {
        // Normalize the coordinates, converting them from [-180, 180] to [0, 1]
        // Then multiply by grid size to convert them into world coordinates
        let normalized_longitude = (self.longitude + 180.0) / 360.0;
        let normalized_latitude = (self.latitude + 180.0) / 360.0;

        let x = normalized_longitude * grid.rect.width() + grid.rect.min.x;
        let y = normalized_latitude * grid.rect.height() + grid.rect.min.y;

        let position = Vec3::new(x, y, 0.0);  // z is 0 for a flat surface

        // Since we're working with a flat grid, translation is straightforward and doesn't require rotation or scaling.
        let transform = Transform {
            translation: position,
            rotation: Quat::IDENTITY,  // No rotation
            scale: Vec3::ONE,  // No scaling
        };

        transform
    }

    #[inline]
    pub fn project_surface_coordinate_on_cylinder(&self, cylinder: &CylinderParameters) -> Transform {
        let x = cylinder.radius * self.longitude.sin();
        let y = (self.latitude + 1.0) / 2.0 * cylinder.height;
        let z = cylinder.radius * self.longitude.cos();

        let position = Vec3::new(x, y, z);
        let _up = Vec3::new(0.0, 1.0, 0.0); // assumes top of the cylinder is 'up'

        // Create a rotation matrix based on longitude
        let rotation = Quat::from_rotation_y(self.longitude);  // In Rust-Bevy, Quat represents a quaternion, and we are using it for rotation.

        // Combine translation and rotation to get the final transformation
        // In Rust-Bevy, Transform represents both rotation and translation of an object.
        let transform = Transform {
            translation: position,
            rotation,
            scale: Vec3::ONE,
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