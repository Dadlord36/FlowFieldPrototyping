use bevy::math::{URect, UVec2};
use bevy::utils::HashMap;

use crate::{
    components::grid_components::definitions::{
        Grid2D,
        GridRelatedData,
        Occupation,
    },
    tests::common,
};
use crate::components::directions::Direction;
use crate::function_libs::grid_calculations;
use crate::tests::common::construct_default_grid;

const PATHFINDING_RECT: UVec2 = UVec2::new(10, 10);

/*#[test]
fn test_generate_map() {
    let grid: Grid2D = common::construct_default_grid();
    let mut grid_related_data = GridRelatedData::new(&grid);
    grid_related_data.fill_with_random_obstacle_pattern(&grid);

    let map: PathfindingMap = grid_related_data.create_pathfinding_map_on(&grid, grid.indexes_rect);
    let mut start_point: CellIndex2d = grid.indexes_rect.max.into();
    start_point.y -= start_point.x / 2;
    let area = grid.calculate_square_area_wrapped_from(start_point, PATHFINDING_RECT);

    let pathfinder = map.find_destination_in_direction(start_point, Direction::West);
    //assert that pathfinder is some
    assert!(pathfinder.is_some(), "No pathfinder found");
    let pathfinder = pathfinder.unwrap();

    let result = map.calculate_path_coordinates_global(pathfinder);
    //assert result is some
    assert!(result.is_some(), "No path found");
    //unwrap result into the same variable
    let result = result.unwrap();

    assert!(result.len() > 0, "No steps were made");
    pathfinder.visualize_path_on_grid(&grid, &grid_related_data, &result, &area);
}*/

//Visualize grid with obstacles and a path on it
/*fn visualize(grid: &Grid2D, grid_related_data: &GridRelatedData,
             pathfinder: &Pathfinder, path: &NavigationPath) {
    println!("Visualizing grid...");

    let mut output = String::new();
    for row in (0..grid.row_number).rev() {
        for col in 0..grid.column_number {
            let cell_index2d = CellIndex2d::new(col, row);
            let cell_index1d = grid.calc_cell_index_1d_at(cell_index2d) as usize;

            let cell_related_data = grid_related_data.get_data_at(&cell_index2d);
            let is_in_path = path.steps.contains(&cell_index1d);
            let cell_repr = determine_cell_type(pathfinder, cell_index2d, cell_related_data,
                                                is_in_path);
            output.push_str(&format!("|{}|\t", cell_repr));
        }
        output.push('\n');
    }
    print!("{}", output);
}*/

#[test]
fn test_obstacles_identification() {
    let grid: Grid2D = common::construct_default_grid();
    let mut grid_related_data = GridRelatedData::new(&grid);
    grid_related_data.fill_with_random_obstacle_pattern(&grid);

    for coordinate in grid.iter_coordinates() {
        let central_index = &coordinate;

        if grid_related_data.get_data_at(central_index).occupation_state != Occupation::Occupied {
            continue;
        }

        let segment_rect = grid.calculate_area_clamped_from_center(central_index,
                                                                   UVec2::new(8, 8));

        for cell_in_segment in grid.iter_coordinates_in_area(segment_rect) {
            let detraction_factor = central_index.inverse_chebyshev_distance(&cell_in_segment);
            grid_related_data.set_increased_detraction_factor(&cell_in_segment, detraction_factor);
        }
    }

    // grid_related_data[central_index].occupation_state = Occupation::Temp;

    grid_related_data.visualize_on_grid(&grid)
}

#[test]
fn test_split_grid() {
    let grid = construct_default_grid();
    let rect = grid.indexes_rect;

    let segments_map = grid_calculations::split_grid_in_compass_directions(&rect);
    //Asset that no rects are intersecting other
    for (direction1, rect1) in segments_map.iter() {
        for (direction2, rect2) in segments_map.iter() {
            if direction1 != direction2 && grid_calculations::are_intersecting_exclusive(*rect1, *rect2) {
                panic!("Rects {:?} and {:?} - are intersecting", rect1, rect2);
            }
        }
    }
    println!("All segments are non intersecting each other exclusively")
}

fn printout_segment(result3: &HashMap<Direction, URect>) {
    for segment in result3.iter() {
        println!("Segment: Direction {} : [{:?}]", segment.0, segment.1);
    }
}