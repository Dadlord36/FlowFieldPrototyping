use std::cmp::{max, min};

use bevy::math::{UVec2, Vec2};
use ndarray::prelude::*;
use rand::Rng;

use crate::components::{
    flow_field_components::{ExplosionParameters, FlowField},
    grid_components::definitions::{CellIndex2d, Grid2D},
};

impl ExplosionParameters {
    pub fn new(impact_center_cell_index: CellIndex2d, impact_radius: f32) -> Self {
        Self { impact_center_cell_index, impact_radius }
    }
}


impl FlowField {
    fn from_array(array: Array2<Vec2>) -> FlowField
    {
        FlowField { field: array }
    }

    pub fn form_field(grid_cols: usize, grid_rows: usize) -> Self {
        let x_bias = -1.0;
        let mut rng = rand::thread_rng();

        let mut field_init: Array2<Vec2> = Array2::from_shape_fn((grid_cols, grid_rows), |(_c, _r)| {
            Vec2::new(x_bias, rng.gen_range(-0.2..0.2))
        });

        let boundary_thickness = 4;

        for i in 1..boundary_thickness {
            let damping = i as f32 / boundary_thickness as f32;
            let (mut first_half, mut second_half) =
                field_init.view_mut().split_at(Axis(0), boundary_thickness);

            let mut first_element = first_half.column_mut(i);
            let mut second_element = second_half.column_mut(i);

            first_element.iter_mut().for_each(|v| v.y *= damping);
            second_element.iter_mut().for_each(|v| v.y *= -damping);
        }

        for element in field_init.iter_mut() {
            *element = element.normalize();
        }

        for i in 0..grid_cols {
            field_init[[i, 0]] = field_init[[i, grid_rows - 1]];
        }

        FlowField::from_array(field_init)
    }

    pub fn get_field_at(&self, cell_index: &CellIndex2d) -> Vec2 {
        self.field[cell_index]
    }

    //get a rotation angle in radians from flow direction at index
    pub fn get_rotation_angle_at(&self, cell_index: &CellIndex2d) -> f32 {
        let field_at_index: Vec2 = self.get_field_at(cell_index);
        field_at_index.x.atan2(field_at_index.y)
    }

    pub fn apply_smooth_explosion(&mut self, grid_parameters: &Grid2D, explosion_parameters: ExplosionParameters) {
        let impact_full_radius = explosion_parameters.impact_radius;
        let (min_limit, max_limit) = grid_parameters.calculate_indexes_limits_in_rang(explosion_parameters.impact_center_cell_index,
                                                                                      impact_full_radius as u32);

        let mut cell_index_2d = CellIndex2d::default();
        for x in u32::from(min_limit.x)..=u32::from(max_limit.x) {
            cell_index_2d.x = x.into();
            for y in u32::from(min_limit.y)..=u32::from(max_limit.y) {
                cell_index_2d.y = y.into();

                self.field[&cell_index_2d] = apply_explosion_to_flow_vector(self.field[&cell_index_2d],
                                                                            cell_index_2d.into(), explosion_parameters.impact_center_cell_index,
                                                                            grid_parameters.cell_size, 0.9);
            }
        }
    }
}

pub fn apply_explosion_to_flow_vector(current_flow_vector: Vec2, cell_index: CellIndex2d, impact_center_cell_index: CellIndex2d,
                                      cell_size: Vec2, effect_magnitude: f32) -> Vec2 {
    if impact_center_cell_index == cell_index {
        return current_flow_vector;
    }

    let world_center = impact_center_cell_index * cell_size;
    let world_position = cell_index * cell_size;

    let direction = (world_position - world_center).normalize();
    let explosion_vector = direction * effect_magnitude;

    return current_flow_vector + explosion_vector;
}

pub fn calculate_max_index(grid_parameters: &Grid2D, impact_center_cell_index: UVec2, impact_radius: f32)
                           -> UVec2 {
    let x = min(grid_parameters.max_column_index, (impact_center_cell_index.x as f32 + impact_radius / grid_parameters.cell_size.x) as u32);
    let y = min(grid_parameters.max_row_index, (impact_center_cell_index.y as f32 + impact_radius / grid_parameters.cell_size.y) as u32);
    return UVec2::new(x, y);
}

pub fn calculate_min_index(grid_parameters: &Grid2D, impact_center_cell_index: UVec2, impact_radius: f32)
                           -> UVec2 {
    let x = max(0, (impact_center_cell_index.x as f32 - impact_radius / grid_parameters.cell_size.x) as u32);
    let y = max(0, (impact_center_cell_index.y as f32 - impact_radius / grid_parameters.cell_size.y) as u32);
    return UVec2::new(x, y);
}
