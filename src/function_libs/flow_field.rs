use std::cmp::{max, min};

use bevy::math::{UVec2, Vec2};
use rand::Rng;

use crate::{
    components::{
        flow_field_components::{ExplosionParameters, FlowField},
        grid_components::GridParameters
    },
    function_libs::grid_calculations
};

impl ExplosionParameters {
    pub fn new(impact_center_cell_index: UVec2, impact_radius: f32) -> Self {
        Self { impact_center_cell_index, impact_radius }
    }
}


impl FlowField {
    fn from_array(array: Vec<Vec2>) -> FlowField
    {
        FlowField { field: array }
    }

    pub fn form_field(grid_cols: u32, grid_rows: u32) -> Self {
        let x_bias = -1.0;
        let mut rng = rand::thread_rng();

        let field_init: Vec<Vec<Vec2>> = (0..grid_cols).map(|_c| {
            (0..grid_rows).map(
                |_| Vec2::new(x_bias, rng.gen_range(-0.2..0.2))
            ).collect()
        }).collect();

        let mut field_values = field_init;

        let boundary_thickness = 4;

        for i in 1..boundary_thickness {
            let damping = i as f32 / boundary_thickness as f32;
            for j in 0..grid_rows {
                let first_element = field_values[i as usize][j as usize];
                let second_element = field_values[(grid_cols - (i + 1)) as usize][j as usize];

                let new_value = first_element.y * damping;
                field_values[i as usize][j as usize].y = new_value;
                field_values[(grid_cols - (i + 1)) as usize][j as usize].y = second_element.y * -damping;
            }
        }

        for i in 0..grid_cols {
            for j in 0..grid_rows {
                field_values[i as usize][j as usize] = field_values[i as usize][j as usize].normalize();
            }
        }

        for i in 0..grid_cols {
            field_values[i as usize][0] = field_values[i as usize][(grid_rows - 1) as usize];
        }
        let final_array: Vec<Vec2> = field_values.concat();

        FlowField::from_array(final_array)
    }

    pub fn get_field_at(&self, grid_parameters: &GridParameters, cell_index: UVec2) -> Vec2 {
        self.field[grid_calculations::calculate_1d_from_2d_index(grid_parameters, cell_index)]
    }

    //get a rotation angle in radians from flow direction at index
    pub fn get_rotation_angle_at(&self, grid_parameters: &GridParameters, cell_index: UVec2) -> f32 {
        let field_at_index: Vec2 = self.get_field_at(grid_parameters, cell_index);
        field_at_index.x.atan2(field_at_index.y)
    }

    pub fn apply_smooth_explosion(&mut self, grid_parameters: &GridParameters, explosion_parameters: ExplosionParameters) {
        let impact_full_radius = explosion_parameters.impact_radius;
        let (min_limit, max_limit) = grid_parameters.calculate_indexes_limits_in_rang(explosion_parameters.impact_center_cell_index,
                                                                                      impact_full_radius as u32);

        let mut cell_index_2d = UVec2::ZERO;
        for x in min_limit.x..=max_limit.x {
            cell_index_2d.x = x;
            for y in min_limit.y..=max_limit.y {
                cell_index_2d.y = y;

/*                let distance_to_center = explosion_parameters.impact_center_cell_index.as_vec2().distance(cell_index_2d.as_vec2());
                if distance_to_center < impact_full_radius {*/
                    let cel_index_1d = grid_calculations::calculate_1d_from_2d_index(grid_parameters, cell_index_2d);
                    self.field[cel_index_1d] = apply_explosion_to_flow_vector(self.field[cel_index_1d],
                                                                              cell_index_2d, explosion_parameters.impact_center_cell_index,
                                                                              grid_parameters.cell_size, 0.9);
                // }
            }
        }
    }
}

pub fn apply_explosion_to_flow_vector(current_flow_vector: Vec2, cell_index: UVec2, impact_center_cell_index: UVec2,
                                      cell_size: Vec2, effect_magnitude: f32) -> Vec2 {
    if impact_center_cell_index == cell_index {
        return current_flow_vector;
    }

    let world_center = impact_center_cell_index.as_vec2() * cell_size;
    let world_position = cell_index.as_vec2() * cell_size;

    let direction = (world_position - world_center).normalize();
    let explosion_vector = direction * effect_magnitude;

    return current_flow_vector + explosion_vector;
}

pub fn calculate_max_index(grid_parameters: &GridParameters, impact_center_cell_index: UVec2, impact_radius: f32)
                           -> UVec2 {
    let x = min(grid_parameters.max_column_index, (impact_center_cell_index.x as f32 + impact_radius / grid_parameters.cell_size.x) as u32);
    let y = min(grid_parameters.max_row_index, (impact_center_cell_index.y as f32 + impact_radius / grid_parameters.cell_size.y) as u32);
    return UVec2::new(x, y);
}

pub fn calculate_min_index(grid_parameters: &GridParameters, impact_center_cell_index: UVec2, impact_radius: f32)
                           -> UVec2 {
    let x = max(0, (impact_center_cell_index.x as f32 - impact_radius / grid_parameters.cell_size.x) as u32);
    let y = max(0, (impact_center_cell_index.y as f32 - impact_radius / grid_parameters.cell_size.y) as u32);
    return UVec2::new(x, y);
}
