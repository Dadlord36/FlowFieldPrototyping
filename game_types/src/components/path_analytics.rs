use bracket_pathfinding::prelude::{a_star_search, BaseMap, NavigationPath, SmallVec};
use colored::{ColoredString, Colorize};
use pathfinding::prelude::astar;

use crate::{
    components::{
        grid_components::{
            definitions::{
                Grid2D,
                CellIndex1d,
                CellIndex2d,
                Occupation,
                GridCellData,
                GridRelatedData,
            },
            grid_related_iterators::CoordinateIterator,
        },
        movement_components::{
            Direction,
            SurfaceCoordinate,
        },
        pathfinding_components::{Pathfinder, PathfindingMap},
    },
    function_libs::grid_calculations::{
        self
        ,
    },
};
use crate::components::grid_components::grid_related_iterators::AreaLineIterator;

impl<'a> PathfindingMap<'a> {
    #[deprecated]
    pub fn calculate_path_coordinates_global_old(&self, grid2d: &Grid2D, pathfinder: Pathfinder)
                                                 -> (NavigationPath, Vec<SurfaceCoordinate>) {
        let path = self.calculate_path_local(pathfinder);
        let global_points = &self.convert_normalized_1d_to_global_points(&path.steps);
        (path, grid2d.calculate_surface_coordinates_for_2d(&global_points))
    }

    #[inline]
    pub fn calculate_path_coordinates_global(&self, pathfinder: Pathfinder)
                                             -> Option<Vec<CellIndex2d>> {
        let result = self.find_path_points(pathfinder);
        if result.is_none() {
            return None;
        }
        let path_points = result.unwrap().0;
        return Some(self.convert_normalized_2d_to_global_points(&path_points));
    }

    #[inline]
    fn calculate_path_local(&self, pathfinder: Pathfinder) -> NavigationPath {
        //log what is going to happen, printing out the area as well
        println!("Calculating path from {} to {} in area {:?}", pathfinder.start, pathfinder.end,
                 self.area);
        let width = self.area.width();
        a_star_search(grid_calculations::calculate_1d_index(pathfinder.start, width),
                      grid_calculations::calculate_1d_index(pathfinder.end, width), self)
    }

    #[inline]
    fn find_path_points(&self, pathfinder: Pathfinder) -> Option<(Vec<CellIndex2d>, u32)> {
        astar(&pathfinder.start,
              |p| self.calculate_successors(p),
              |p| p.distance(&pathfinder.end),
              |p| *p == pathfinder.end)
    }

    fn convert_normalized_1d_to_global_points(&self, normalized_points: &Vec<usize>) -> Vec<CellIndex2d> {
        //preallocate the array of the same size
        let mut global_points = Vec::with_capacity(normalized_points.len());

        for normalized_point in normalized_points.iter() {
            let local_cell_index2d = self.grid_segment.convert_1d_to_2d(normalized_point.clone() as CellIndex1d);
            let global_cell_index2d = self.grid_segment.local_to_global_index(local_cell_index2d);
            global_points.push(global_cell_index2d)
        }

        global_points
    }
    //
    fn convert_normalized_2d_to_global_points(&self, normalized_points: &Vec<CellIndex2d>) -> Vec<CellIndex2d> {
        let mut global_points = Vec::with_capacity(normalized_points.len());

        for normalized_point in normalized_points.iter() {
            let global_cell_index2d = self.grid_segment.local_to_global_index(normalized_point.clone());
            global_points.push(global_cell_index2d)
        }

        global_points
    }

    pub fn find_destination_in_direction(&self, from: CellIndex2d, direction: Direction) -> Option<Pathfinder> {
        let mut result: Option<Pathfinder> = None;
        let mut pathfinder = Pathfinder::ZERO;

        pathfinder.start = self.grid_segment.global_to_local_index(from);
        let closest_obstacle = self.find_closest_cell_in_direction_local(pathfinder.start,
                                                                         direction, Occupation::Occupied);
        if closest_obstacle.is_none() {
            return None;
        }
        let closest_empty_cell = self.find_closest_cell_in_direction_local(closest_obstacle.unwrap(),
                                                                           direction, Occupation::Free);
        if closest_empty_cell.is_some() {
            pathfinder.end = closest_empty_cell.unwrap();

            result = Some(pathfinder);
        }
        return result;
    }

    pub fn find_closest_cell_in_direction_local(&self, from_local_index: CellIndex2d, direction: Direction,
                                                occupation_state: Occupation) -> Option<CellIndex2d> {
        let mut was_found = false;
        let mut local_index = from_local_index;
        let mut global_index: Option<CellIndex2d> = None;

        for cell_index in AreaLineIterator::iter_area_in_line_from(local_index, direction.as_vector(),
                                                                   self.area_normalized)
        {
            if self[cell_index].occupation_state != occupation_state {
                continue;
            }

            let global_index_temp = self.grid_segment.local_to_global_index(cell_index);
            // info!("{}", "We are iterating! {global_index_temp}".green());

            was_found = true;
            global_index = Some(global_index_temp);
            local_index = cell_index;
            break;
        };
        return Some(local_index);
    }

    pub fn calculate_successors(&self, cell_index2d: &CellIndex2d) -> SmallVec<[(CellIndex2d, u32); 10]> {
        let mut successors = SmallVec::<[(CellIndex2d, u32); 10]>::new();

        for direction in [Direction::North.as_vector(), Direction::East.as_vector(),
            Direction::South.as_vector(), Direction::West.as_vector()]
        {
            let outer_cell = *cell_index2d + direction;
            // Boundary check
            if !self.is_valid_index(&outer_cell) {
                continue;
            }
            if self[outer_cell].occupation_state != Occupation::Free {
                continue;
            }
            successors.push((outer_cell, 1));
        }

        successors
    }

    pub fn convert_to_global(&self, pathfinder: Pathfinder) -> Pathfinder {
        Pathfinder {
            start: self.grid_segment.local_to_global_index(pathfinder.start),
            end: self.grid_segment.local_to_global_index(pathfinder.end),
        }
    }

    pub fn visualize_key_points_on_grid(&self, grid: &Grid2D, pathfinder: &Pathfinder,
                                        grid_related_data: &GridRelatedData) {
        println!("{}", "Visualizing grid...".yellow());

        let mut output: Vec<ColoredString> = Vec::new();
        for row in (0..grid.row_number).rev() {
            for col in 0..grid.column_number {
                let cell_index2d = CellIndex2d::new(col, row);
                let cell_related_data = grid_related_data.get_data_at(&cell_index2d);
                let cell_repr: ColoredString = self.determine_cell_type(cell_index2d, pathfinder, cell_related_data,
                                                                        false);
                output.push(format!("|{}| ", cell_repr).normal());
            }
            output.push("\n".normal());
        }

        for colored_string in output {
            print!("{}", colored_string);
        }
    }

    pub fn visualize_path_on_grid(&self, grid: &Grid2D, pathfinder: &Pathfinder,
                                  grid_related_data: &GridRelatedData,
                                  path: &Vec<CellIndex2d>) {
        println!("{}", "Visualizing grid...".yellow());

        let mut output: Vec<ColoredString> = Vec::new();
        for row in (0..grid.row_number).rev() {
            for col in 0..grid.column_number {
                let cell_index2d = CellIndex2d::new(col, row);
                let cell_related_data = grid_related_data.get_data_at(&cell_index2d);
                let is_in_path = path.contains(&cell_index2d);
                let cell_repr: ColoredString = self.determine_cell_type(cell_index2d, pathfinder,
                                                                        cell_related_data, is_in_path);
                output.push(format!("|{}| ", cell_repr).normal());
            }
            output.push("\n".normal());
        }

        for colored_string in output {
            print!("{}", colored_string);
        }
    }

    fn determine_cell_type(&self, cell_index2d: CellIndex2d, pathfinder: &Pathfinder, cell_related_data: &GridCellData,
                           is_in_path: bool) -> ColoredString {
        let cell_repr: ColoredString =
            if cell_index2d == pathfinder.start {
                "S".green()
            } else if cell_index2d == pathfinder.end {
                "T".red()
            } else if is_in_path {
                "P".blue()  // Path
            } else if cell_related_data.occupation_state == Occupation::Occupied {
                "O".black()  // Obstacle
            } else if self.area.contains(cell_index2d.into()) {
                "E".bright_yellow()
            } else {
                "E".bright_black()  // Empty cell
            };
        cell_repr
    }
}


impl<'a> BaseMap for PathfindingMap<'a> {
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let width = self.area.width() + 1;

        let central_cell = grid_calculations::calculate_2d_index(_idx as CellIndex1d, width);

        let mut exits = SmallVec::<[(usize, f32); 10]>::new();

        for direction in [Direction::North.as_vector(), Direction::East.as_vector(),
            Direction::South.as_vector(), Direction::West.as_vector()]
        {
            let outer_cell = central_cell + direction;

            // Boundary check
            if !self.is_valid_index(&outer_cell) {
                continue;
            }

            let cell_index_1d = grid_calculations::calculate_1d_index(outer_cell, width);
            if self[outer_cell].occupation_state == Occupation::Free {
                exits.push((cell_index_1d as usize, self[outer_cell].detraction_factor));
            }
        }
        exits
    }
}

