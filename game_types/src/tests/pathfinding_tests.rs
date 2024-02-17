use bevy::math::UVec2;
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

const PATHFINDING_RECT: UVec2 = UVec2::new(10, 10);

#[test]
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
    //extract vector into a separate variable
    let path_coordinates = result.0;

    assert!(path_coordinates.len() > 0, "No steps were made");
    pathfinder.visualize_path_on_grid(&grid, &grid_related_data, &path_coordinates, &area);
}

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

