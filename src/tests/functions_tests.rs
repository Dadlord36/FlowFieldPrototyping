use approx::assert_relative_eq;
use bevy::log::info;
use bevy::math::UVec2;
use bevy::prelude::Vec2;

use crate::{
    components::{
        grid_components::GridParameters,
        movement_components::SurfaceCoordinate,
    },
    function_libs::grid_calculations,
};

fn construct_default_grid() -> GridParameters {
    let grid_parameters = GridParameters::new(25, 25, Vec2::new(50f32, 50f32));
    grid_parameters
}

#[test]
pub fn test_grid_iteration() {
    let grid_parameters = construct_default_grid();
    let mut output = String::new();
    let mut previous_row = grid_parameters.max_row_index;
    println!("Grid coordinates iteration test.");
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

    println!();

    // Create the vector of expected outputs
    let mut expected_output: Vec<(u32, u32)> = Vec::new();
    for row in 0..grid_parameters.row_number {
        for col in 0..grid_parameters.column_number {
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
    let grid_parameters: GridParameters = construct_default_grid();

    for (col, row) in grid_parameters.coordinates() {
        let cell_index_2d = UVec2::new(col, row);
        let coordinate = SurfaceCoordinate::calculate_flat_surface_coordinate_from(&grid_parameters, cell_index_2d);
        let restored_index = coordinate.calculate_cell_index_on_flat_surface(&grid_parameters);
        let restored_location = coordinate.project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();

        assert!(grid_parameters.is_cell_index_in_grid_bounds(restored_index), "restored_index:{restored_index} is not in grid bounds");
        assert!(grid_parameters.is_position_in_grid_bounds(restored_location), "restored_index{restored_index} restored_location:{restored_location} is not in grid bounds");

        assert_eq!(cell_index_2d, restored_index, "original_index: {cell_index_2d}, coordinate: {coordinate}, restored_index: {restored_index}");
        println!("original_index: {cell_index_2d} : restored_index: {restored_index}");
    }

   /* for (col, row) in grid_parameters.coordinates() {
        let cell_index_2d = UVec2::new(col, row);
        let original_position = grid_parameters.calculate_cell_position(cell_index_2d);
        let coordinate = SurfaceCoordinate::calculate_flat_surface_coordinate_from(&grid_parameters, cell_index_2d);
        let restored_position = coordinate.project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();


        assert_relative_eq!(original_position.x, restored_position.x);
        assert_relative_eq!(original_position.y, restored_position.y);
        println!("original_position: {original_position}, restored_position: {restored_position}");
    }*/
}