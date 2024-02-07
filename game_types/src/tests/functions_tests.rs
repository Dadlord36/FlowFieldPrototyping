use bevy::{
    math::{Rect, UVec2},
    prelude::*,
};

use crate::{
    components::{
        grid_components::definitions::{CellIndex2d, Grid2D},
        movement_components::Maneuver,
        grid_components::grid_related_traits::CoordinateIterator,
        movement_components,
    },
    function_libs::grid_calculations,
    tests::common,
};

#[test]
pub fn test_grid_iteration() {
    let grid_parameters = common::construct_default_grid();


    println!("Grid coordinates iteration test.");
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
    println!();

    // Create the vector of expected outputs
    let mut expected_output: Vec<CellIndex2d> = Vec::new();
    for row in 0..grid_parameters.row_number {
        for col in 0..grid_parameters.column_number {
            expected_output.push(CellIndex2d::new(col, row));
        }
    }

    // Get the actual coordinates from the grid
    let actual_output: Vec<_> = grid_parameters.iter_coordinates().collect();

    assert_eq!(expected_output, actual_output, "The order of coordinate iteration is incorrect");
}

#[test]
fn test_grid_area_iteration() {
    let grid = common::construct_default_grid();

    // Define the start point for each direction
    let starting_point = CellIndex2d::new(5, 5);

    let rect = grid.calculate_area_from(CellIndex2d::new(0, 0), IVec2::new(1, 1), 15);

    for direction in movement_components::DIRECTIONS {
        // Run the URectIterator with 'num_steps' steps

        let iterator = CoordinateIterator::iter_area_fully_from(starting_point, direction, rect);
        println!("Iterating fully in direction from: {direction} in rect area: {:?}", rect);
        for cell_index in iterator {
            println!("cell_index: {cell_index}");
        }
    }
}


#[test]
fn test_grid_line_iteration() {
    let grid = common::construct_default_grid();

    // Define the start point for each direction
    let starting_point = CellIndex2d::new(5, 5);

    let rect = grid.calculate_area_from(CellIndex2d::new(0, 0), IVec2::new(1, 1), 10);

    for direction in movement_components::DIRECTIONS {
        // Run the URectIterator with 'num_steps' steps

        let iterator = CoordinateIterator::iter_area_in_line_from(starting_point, direction, rect);
        println!("Iterating in direction from: {direction} in rect area: {:?}", rect);
        for cell_index in iterator {
            println!("cell_index: {cell_index}");
        }
    }
}

// Helper function to compute the expected outcome
fn compute_expected_outcome(start_point: CellIndex2d, direction: Vec2, num_steps: u32) -> Vec<CellIndex2d> {
    (1..=num_steps).map(|i| start_point + CellIndex2d::from(direction * Vec2::new(i as f32, i as f32))).collect()
}

#[test]
fn test_grid_indexing() {
    let grid_parameters = common::construct_default_grid();

    let cell_index_2d = CellIndex2d::new(grid_parameters.max_column_index, grid_parameters.max_row_index);
    let cell_index_1d = grid_calculations::calculate_1d_from_2d_index(&grid_parameters, cell_index_2d);
    let cell_index_2d_restored = grid_calculations::calculate_2d_from_1d_index(&grid_parameters, cell_index_1d);

    println!("cell_index_2d: {}, cell_index_2d_restored: {}", cell_index_2d, cell_index_2d_restored);
    assert_eq!(cell_index_2d, cell_index_2d_restored);
}

#[test]
fn test_surface_coord_to_occupied_cell_index_conversion() {
    let grid_parameters: Grid2D = common::construct_default_grid();

    for cell_index_2d in grid_parameters.iter_coordinates() {
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
    let grid_parameters: Grid2D = common::construct_default_grid();

    for cell_index_2d in grid_parameters.iter_coordinates() {
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
    let grid_parameters: Grid2D = common::construct_default_grid();
    // Define your points here
    let maneuver_points =
        vec![grid_parameters.calculate_flat_surface_coordinate_from(CellIndex2d::from(UVec2::new(0, 0))),
             grid_parameters.calculate_flat_surface_coordinate_from(CellIndex2d::from(UVec2::new(0, 1))),
             grid_parameters.calculate_flat_surface_coordinate_from(CellIndex2d::from(UVec2::new(1, 1))),
             grid_parameters.calculate_flat_surface_coordinate_from(CellIndex2d::from(UVec2::new(1, 0)))];

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

    let cell_half_size = grid_parameters.cell_size / 2.0;
    let rect = Rect::from_corners(min_corner - cell_half_size, max_corner + cell_half_size);

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

//if it is not panicking - it is probably working fine
#[test]
fn test_calculate_indexes_in_circle_from_index() {
    let grid_parameters: Grid2D = common::construct_default_grid();
    // let mut output = String::new();
    for cell_index in grid_parameters.iter_coordinates() {
        let cells_in_range = grid_calculations::calculate_indexes_in_circle_from_index(&grid_parameters,
                                                                                       cell_index, 5u32);
    }
}