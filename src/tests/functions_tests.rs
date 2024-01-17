use bevy::math::UVec2;
use bevy::prelude::Vec2;

use crate::{
    function_libs::flow_field::FlowField,
    function_libs::grid_calculations::{GridParameters, GridRelatedData},
    function_libs::surface_calculations::SurfaceCoordinate,
};
use crate::function_libs::grid_calculations;

fn construct_default_grid() -> GridParameters {
    let grid_parameters = GridParameters::new(25, 25, Vec2::new(50f32, 50f32));
    grid_parameters
}

#[test]
fn test_grid_iteration() {
    let grid_parameters = construct_default_grid();
    let mut output = String::new();
    let mut previous_row = grid_parameters.max_row_index;

    for row in (0..grid_parameters.row_number).rev() {
        if row != previous_row {
            output.push_str("\n");
            previous_row = row;
        }
        for col in 0..grid_parameters.column_number {
            output.push_str(&format!("|{:2}:{:2}|\t", col, row));
        }
    }
    print!("{}", output);

    // Create the vector of expected outputs
    let mut expected_output: Vec<(u32, u32)> = Vec::new();
    for col in 0..grid_parameters.column_number {
        for row in 0..grid_parameters.row_number {
            expected_output.push((col, row));
        }
    }

    // Get the actual coordinates from the grid
    let actual_output: Vec<(u32, u32)> = grid_parameters.coordinates().collect();

    assert_eq!(expected_output, actual_output, "The order of coordinate iteration is incorrect");
}

#[test]
fn test_grid_indexing() {
    let grid_parameters = construct_default_grid();

    let cell_index_2d = UVec2::new(grid_parameters.max_column_index, grid_parameters.max_row_index);
    let cell_index_1d = grid_calculations::calculate_1d_from_2d_index(&grid_parameters, cell_index_2d);
    let cell_index_2d_restored = grid_calculations::calculate_2d_from_1d_index(&grid_parameters, cell_index_1d as u32);

    println!("cell_index_2d: {}, cell_index_2d_restored: {}", cell_index_2d, cell_index_2d_restored);
    assert_eq!(cell_index_2d, cell_index_2d_restored);
}

#[test]
fn test_surface_coord_conversion() {
    let grid_parameters = construct_default_grid();

    let cell_index_2d = UVec2::new(grid_parameters.max_column_index, grid_parameters.max_row_index);
    let coordinate = SurfaceCoordinate::calculate_flat_surface_coordinate_from(&grid_parameters, cell_index_2d);
    let restored_index = coordinate.calculate_cell_index_on_flat_surface(&grid_parameters);

    println!("original_index: {}, coordinate: {}, restored_index: {}", cell_index_2d, coordinate, restored_index);
    assert_eq!(cell_index_2d, restored_index, "original_index: {}, restored_index: {}", cell_index_2d, restored_index);
}