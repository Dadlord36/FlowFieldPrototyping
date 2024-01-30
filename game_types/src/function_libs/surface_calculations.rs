use std::fmt::{Display, Formatter};

use bevy::{
    math::{DVec2, Quat, Vec2, Vec3},
    prelude::Transform,
};
use crate::{
    components::{
        grid_components::{CellIndex2d, Grid2D},
        movement_components::{Coordinate, SurfaceCoordinate}
    }
};


/*const MAX_BOUND_ANGLE: f32 = 180.0;
const FULL_BOUND_ANGLE: f32 = MAX_BOUND_ANGLE * 2.0;
const COORDINATE_BOUNDS: CoordinateBounds = CoordinateBounds { min: -MAX_BOUND_ANGLE, max: MAX_BOUND_ANGLE };*/


impl Display for SurfaceCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude, )
    }
}

impl SurfaceCoordinate {
    pub fn new(latitude: Coordinate, longitude: Coordinate) -> Self {
        SurfaceCoordinate { latitude, longitude }
    }
    pub fn set_latitude(&mut self, value: Coordinate) {
        self.latitude = value.clamp(0.0, 1.0);
    }

    pub fn set_longitude(&mut self, value: Coordinate) {
        self.longitude = value.clamp(0.0, 1.0);
    }

    #[inline]
    pub fn adjust_coordinate(&mut self, offset: DVec2) {
        self.latitude = wrap_value_normalized(self.latitude + offset.x as Coordinate);
        self.longitude = wrap_value_normalized(self.longitude + offset.y as Coordinate);
    }

    #[inline]
    pub fn calculate_cell_index_on_flat_surface(&self, grid_parameters: &Grid2D) -> CellIndex2d {
        let cell_index_x: u32 = (self.latitude * grid_parameters.max_column_index as Coordinate).round() as u32;
        let cell_index_y: u32 = (self.longitude * grid_parameters.max_row_index as Coordinate).round() as u32;
        grid_parameters.form_grid_bound_cell_index(cell_index_x, cell_index_y)
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from_pos(grid_parameters: &Grid2D, world_position: Vec2) -> Self {
        let hovered_cell_index = grid_parameters.calculate_cell_index_from_position(world_position);
        return grid_parameters.calculate_flat_surface_coordinate_from(hovered_cell_index);
    }

    #[inline]
    pub fn project_surface_coordinate_on_grid(&self, grid: &Grid2D) -> Transform {
        let proportional_latitude = self.latitude * (grid.rect.width() as Coordinate - grid.cell_size.x as Coordinate)
            + (grid.rect.min.x + grid.cell_size.x / 2.0) as Coordinate;
        let proportional_longitude = self.longitude * (grid.rect.height() as Coordinate - grid.cell_size.y as Coordinate)
            + (grid.rect.min.y + grid.cell_size.y / 2.0) as Coordinate;

        let position = Vec2::new(proportional_latitude as f32, proportional_longitude as f32);

        let transform = Transform {
            translation: position.extend(10.0),
            rotation: Quat::IDENTITY,  // No rotation
            scale: Vec3::ONE,  // No scaling
        };

        transform
    }
}

#[inline]
pub fn wrap_value_normalized(value: Coordinate) -> Coordinate {
    wrap_value(value, 0.0, 1.0)
}

#[inline]
pub fn wrap_value(value: Coordinate, min_value: Coordinate, max_value: Coordinate) -> Coordinate {
    let range = max_value - min_value;
    let value = repeat(value - min_value, range) + min_value;
    value
}

#[inline]
pub fn repeat(t: Coordinate, length: Coordinate) -> Coordinate {
    (t - ((t / length).floor() * length)).clamp(0.0, length)
}

