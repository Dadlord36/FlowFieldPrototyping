use bevy::math::{Rect, UVec2};
use bevy::prelude::*;

use crate::{
    components::{
        grid_components::GridParameters,
        movement_components::{
            SurfaceCoordinate,
            Maneuver,
        },
    },
    function_libs::{
        grid_calculations,
        pathfinding_calculations::{self},
    },
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
fn test_surface_coord_to_occupied_cell_index_conversion() {
    let grid_parameters: GridParameters = construct_default_grid();

    for (col, row) in grid_parameters.coordinates() {
        let cell_index_2d = UVec2::new(col, row);
        let coordinate = grid_parameters.calculate_flat_surface_coordinate_from(cell_index_2d);
        let restored_index = coordinate.calculate_cell_index_on_flat_surface(&grid_parameters);

        assert!(grid_parameters.is_cell_index_in_grid_bounds(cell_index_2d), "cell_index_2d:{cell_index_2d} is not in grid bounds");
        assert!(grid_parameters.is_cell_index_in_grid_bounds(restored_index), "restored_index:{restored_index} is not in grid bounds");

        assert_eq!(cell_index_2d, restored_index, "original_index: {cell_index_2d}, coordinate: {coordinate}, restored_index: {restored_index}");
        println!("original_index: {cell_index_2d} : restored_index: {restored_index}");
    }
}

#[test]
fn test_coordinate_to_position_on_grid_conversion()
{
    let grid_parameters: GridParameters = construct_default_grid();

    for (col, row) in grid_parameters.coordinates() {
        let cell_index_2d = UVec2::new(col, row);
        assert!(grid_parameters.is_cell_index_in_grid_bounds(cell_index_2d), "cell_index: {cell_index_2d} is out of the grid bounds.");
        let grid_cell_position = grid_parameters.calculate_cell_position(cell_index_2d);
        // assert!(grid_parameters.is_position_in_grid_bounds(grid_cell_position), "grid_cell_position {grid_cell_position} is out of the grid bounds.");

        let coordinate = grid_parameters.calculate_flat_surface_coordinate_from(cell_index_2d);
        let restored_position = coordinate.project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();

        // assert!(grid_parameters.is_position_in_grid_bounds(restored_position), "restored_position:{restored_position} is not in grid bounds");
        assert_eq!(grid_cell_position, restored_position, "cell_index: {cell_index_2d} :: original_grid_cell_position: {grid_cell_position} :: restored_position: {restored_position}");

        println!("cell_index: {cell_index_2d} :: original_grid_cell_position: {grid_cell_position} :: restored_position: {restored_position} - OK!");
    }
}

#[test]
fn test_bezier_interpolate() {
    let grid_parameters: GridParameters = construct_default_grid();
    // Define your points here
    let maneuver_points =
        vec![grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(0, 0)),
             grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(0, 1)),
             grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(1, 1)),
             grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(1, 0))];

    let mut maneuver = Maneuver::new(maneuver_points.clone());

    // Construct the bounding rectangle
    let min_corner = maneuver_points.iter().fold(
        Vec2::new(f32::MAX, f32::MAX),
        |min, coordinate| {
            let point = coordinate.project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();
            point.min(min)
        },
    );
    let max_corner = maneuver_points.iter().fold(
        Vec2::new(f32::MIN, f32::MIN),
        |max, coordinate| {
            let point = coordinate.project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();
            point.max(max)
        },
    );

    // execute bezier_interpolate function
    let output = maneuver.catmull_rom_interpolate_along_path(0.5).project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();

    let rect = Rect::from_corners(min_corner - grid_parameters.cell_size / 2.0, max_corner + grid_parameters.cell_size / 2.0);

    println!("Points:");
    for (coordinate) in maneuver_points.iter() {
        let point = coordinate.project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();
        println!("{point}")
    }
    let grid_physical_size = grid_parameters.rect.size();
    println!("Grid Physical Size: {grid_physical_size}");
    println!("{:?}", rect);

    // Check if the output is within the bounds
    assert!(rect.contains(output), "Output location is out of bounds. bezier interpolation output: {output}");
}