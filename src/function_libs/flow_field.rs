use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Resource};
use rand::Rng;

use crate::function_libs::grid_calculations;
use crate::function_libs::grid_calculations::GridParameters;

#[derive(Resource)]
pub struct FlowField {
    field: Vec<Vec2>,
}

impl FlowField {
    fn from_array(array: &Vec<Vec2>) -> FlowField
    {
        FlowField {
            field: array.clone()
        }
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

        FlowField::from_array(&final_array)
    }

    pub fn get_field_at(&self, grid_parameters: &GridParameters, cell_index: UVec2) -> Vec2 {
        self.field[grid_calculations::calculate_1d_from_2d_index(grid_parameters, cell_index)]
    }

    //get a rotation angle in radians from flow direction at index
    pub fn get_rotation_angle_at(&self, grid_parameters: &GridParameters, cell_index: UVec2) -> f32 {
        let field_at_index: Vec2 = self.get_field_at(grid_parameters, cell_index);
        field_at_index.y.atan2(field_at_index.x)
    }
}

