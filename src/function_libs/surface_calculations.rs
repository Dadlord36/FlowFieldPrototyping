use std::fmt::{Display, Formatter};

use bevy::math::{Quat, UVec2, Vec2, Vec3};
use bevy::prelude::{Component, Transform};

use crate::{
    components::{
        movement_components::{
            self, SurfaceCoordinate, CoordinateBounds,
        },
        grid_components::{
            GridParameters, GridRelatedData, GridCellData
        },
    },
    function_libs::grid_calculations,
};

const COORDINATE_BOUNDS: CoordinateBounds = CoordinateBounds { min: -180.0, max: 180.0 };


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
        // convert SurfaceCoordinate [-180,180] to cell index, factoring in the origin of the rect.
        let scale_x = (self.latitude - COORDINATE_BOUNDS.min) / (COORDINATE_BOUNDS.max - COORDINATE_BOUNDS.min);
        let scale_y = (self.longitude - COORDINATE_BOUNDS.min) / (COORDINATE_BOUNDS.max - COORDINATE_BOUNDS.min);

        // Using the calculated scale, find the corresponding cell index
        let cell_index_x = (scale_x * grid_parameters.column_number as f32).floor() as u32;
        let cell_index_y = (scale_y * grid_parameters.row_number as f32).floor() as u32;

        UVec2::new(cell_index_x, cell_index_y)
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from(grid_parameters: &GridParameters, cell_index2d: UVec2) -> Self
    {
        // convert cell index to SurfaceCoordinate that has range ofr longitude and latitude as [-180,180],
        let latitude = 180.0 * (2.0 * (cell_index2d.x as f32 / grid_parameters.max_column_index as f32) - 1.0);
        let longitude = 180.0 * (2.0 * (cell_index2d.y as f32 / grid_parameters.max_row_index as f32) - 1.0);

        Self {
            latitude,
            longitude,
        }
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from_pos(grid_parameters: &GridParameters, world_position: Vec2) -> Self{
        let hovered_cell_index = grid_calculations::calculate_cell_index_from_position(grid_parameters, world_position);
        return SurfaceCoordinate::calculate_flat_surface_coordinate_from(grid_parameters, hovered_cell_index);
    }

    #[inline]
    pub fn project_surface_coordinate_on_grid(&self, grid: &GridParameters) -> Transform {
        // Normalize the coordinates, converting them from [-180, 180] to [0, 1]
        // Then multiply by grid size to convert them into world coordinates
        let normalized_latitude = (self.latitude + 180.0) / 360.0;
        let normalized_longitude = (self.longitude + 180.0) / 360.0;

        let x = normalized_latitude * grid.rect.width() + grid.rect.min.x;
        let y = normalized_longitude * grid.rect.height() + grid.rect.min.y;

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

