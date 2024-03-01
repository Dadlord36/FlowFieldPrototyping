use std::cmp;
use bevy::math::{Rect, URect, UVec2};
use crate::{
    components::{
        grid_components::definitions::{
            CellIndex2d,
            Grid2D,
            GridRelatedData
            ,
        },
        movement_components::Direction,
        pathfinding_components::PathfindingMap,
    },
    tests::common,
};
use crate::components::grid_components::definitions::Occupation;

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

    let central_index = grid.get_central_cell();
    let segment_rect = URect::from_center_size(central_index.into(),
                                               UVec2::new(5, 5));
    visualize_rect(&segment_rect);

    for cell_in_segment in grid.iter_coordinates_in_area(segment_rect) {
        grid_related_data[cell_in_segment].occupation_state = Occupation::Occupied;
    }
    grid_related_data[central_index].occupation_state = Occupation::Temp;
    grid_related_data.visualize_on_grid(&grid)
}

fn visualize_rect(rect: &URect) {
    let UVec2 { x: min_x, y: min_y } = rect.min;
    let UVec2 { x: max_x, y: max_y } = rect.max;

    let width = (max_x - min_x) as usize;
    let height = (max_y - min_y) as usize;

    let center_x = width / 2;
    let center_y = height / 2;

    // Add center only if width and height are at minimum 3 which will fit the corners and center
    let top_bottom = "+".to_string() + &"-".repeat(cmp::max(0, width - 2)) + "+";

    let mut rows = vec!["|".to_string() + &" ".repeat(cmp::max(0, width - 2)) + "|"; cmp::max(0, height - 2)];

    if !rows.is_empty() && rows.get(center_y - 1).is_some() {
        let row = &mut rows[center_y - 1];
        let new_row = format!("|{}*{}|",
                              &" ".repeat(cmp::max(0, center_x - 1)),
                              &" ".repeat(cmp::max(0, (width - 2) - center_x)));
        *row = new_row;
    }

    let sides = rows.join("\n");

    let output = vec![top_bottom.clone(), sides, top_bottom].join("\n");

    println!("{}", output);
}
