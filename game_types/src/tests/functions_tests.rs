use bevy::{
    math::{Rect, UVec2},
    prelude::*,
};

use crate::{
    components::{
        grid_components::{
            definitions::{
                CellIndex2d,
                Grid2D,
                GridSegment,
            },
            grid_related_iterators::CoordinateIterator,
        },
        movement_components::{
            self,
            Maneuver,
        },
    },
    function_libs::grid_calculations,
    tests::common,
};

use test_case::test_case;

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
        println!("Iterating in direction: {direction} from: {starting_point} in rect area: {:?}", rect);
        for cell_index in iterator {
            println!("cell_index: {cell_index}");
        }
    }
}

/*#[test]
fn test_if_grid_transforming_works() {
    let grid: Grid2D = common::construct_default_grid();
    let segment: GridSegment = grid.form_segment_for(URect::from_corners(UVec2::new(5, 5),
                                                                         grid.indexes_rect.max));

}*/

#[test_case(CellIndex2d { x: 8, y: 8 }, CellIndex2d { x: 2, y: 2 }; "corner case")]
#[test_case(CellIndex2d { x: 7, y: 7 }, CellIndex2d { x: 1, y: 1 }; "middle case")]
#[test_case(CellIndex2d { x: 6, y: 6 }, CellIndex2d { x: 0, y: 0 }; "lower limit case")]
fn test_global_to_local_index(global_index: CellIndex2d, expected_local_index: CellIndex2d) {
    let grid: Grid2D = common::construct_default_grid();
    let child = URect::from_corners(UVec2::new(6, 6), UVec2::new(9, 9));
    let segment = GridSegment::new(grid.indexes_rect, child);

    println!("Global index: {:?}", global_index);
    println!("Child grid min corner: {:?}", child.min);

    // Explanation
    println!("\nConverting from global to local index:");
    println!("The global index of (8, 8) corresponds to a location within the child grid.");
    println!("The 'offset' of the GridSegment represents the coordinate of the child grid's top left corner \
     in the parent grid's coordinate system. In this case, it is {:?}", segment.get_offset());
    println!("Therefore, to convert the global index to a local index, we subtract the segment's 'offset' from the global index.");

    let local_index = segment.global_to_local_index(global_index);
    println!("Local index: {local_index} ");
    println!();
    assert_eq!(local_index, expected_local_index);
}

#[test_case(CellIndex2d { x: 2, y: 2 }, CellIndex2d { x: 8, y: 8 }; "corner case")]
#[test_case(CellIndex2d { x: 1, y: 1 }, CellIndex2d { x: 7, y: 7 }; "middle case")]
#[test_case(CellIndex2d { x: 0, y: 0 }, CellIndex2d { x: 6, y: 6 }; "lower limit case")]
fn test_local_to_global_index(local_index: CellIndex2d, expected_global_index: CellIndex2d) {
    let grid: Grid2D = common::construct_default_grid();
    let child = URect::from_corners(UVec2::new(6, 6), UVec2::new(9, 9));
    let segment = GridSegment::new(grid.indexes_rect, child);

    println!("Local index: {:?}", local_index);
    println!("Child grid min corner: {:?}", child.min);

    // Explanation
    println!("\nConverting from local to global index:");
    println!("The local index of (2, 2) corresponds to a location within the child grid.");
    println!("The 'offset' of the GridSegment represents the coordinate of the child grid's top left corner /\
         in the parent grid's coordinate system. In this case, it is {:?}", segment.get_offset());
    println!("Therefore, to convert the local index to a global index, we add the segment's 'offset' to the local index.");

    let global_index = segment.local_to_global_index(local_index);
    println!("Global index: {:?}", global_index);
    assert_eq!(global_index, expected_global_index);
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

    assert_eq!(cell_index_2d, cell_index_2d_restored);
    println!("cell_index_2d: {}, cell_index_2d_restored: {}", cell_index_2d, cell_index_2d_restored);
}

#[test]
fn test_surface_coord_to_occupied_cell_index_conversion() {
    let grid_parameters: Grid2D = common::construct_default_grid();

    for cell_index_2d in grid_parameters.iter_coordinates() {
        let coordinate = grid_parameters.calculate_flat_surface_coordinate_from_2d(cell_index_2d);
        let restored_index = coordinate.calculate_cell_index_on_flat_surface(&grid_parameters);

        assert!(grid_parameters.is_cell_index_in_grid_bounds(cell_index_2d), "cell_index_2d:{cell_index_2d} is not in grid bounds");
        assert!(grid_parameters.is_cell_index_in_grid_bounds(restored_index), "restored_index:{restored_index} is not in grid bounds");

        println!("original_index: {cell_index_2d} : restored_index: {restored_index}");
        assert_eq!(cell_index_2d, restored_index, "original_index: {cell_index_2d}, coordinate: {coordinate}, restored_index: {restored_index}");
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

        let coordinate = grid_parameters.calculate_flat_surface_coordinate_from_2d(cell_index_2d);
        let restored_position = coordinate.project_surface_coordinate_on_grid(&grid_parameters).translation.truncate();

        // assert!(grid_parameters.is_position_in_grid_bounds(restored_position), "restored_position:{restored_position} is not in grid bounds");
        println!("cell_index: {cell_index_2d} :: original_grid_cell_position: {grid_cell_position} :: restored_position: {restored_position} - OK!");
        assert_eq!(grid_cell_position, restored_position, "cell_index: {cell_index_2d} :: original_grid_cell_position: {grid_cell_position} :: restored_position: {restored_position}");
    }
}

#[test]
fn test_bezier_interpolate() {
    let grid_parameters: Grid2D = common::construct_default_grid();
    // Define your points here
    let maneuver_points =
        vec![grid_parameters.calculate_flat_surface_coordinate_from_2d(CellIndex2d::from(UVec2::new(0, 0))),
             grid_parameters.calculate_flat_surface_coordinate_from_2d(CellIndex2d::from(UVec2::new(0, 1))),
             grid_parameters.calculate_flat_surface_coordinate_from_2d(CellIndex2d::from(UVec2::new(1, 1))),
             grid_parameters.calculate_flat_surface_coordinate_from_2d(CellIndex2d::from(UVec2::new(1, 0)))];

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
    let grid_physical_size = grid_parameters.shape_rect.size();
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

