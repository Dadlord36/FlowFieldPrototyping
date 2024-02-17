use bevy::log::info;
use bracket_pathfinding::prelude::{a_star_search, BaseMap, NavigationPath, SmallVec};
use colored::Colorize;
use pathfinding::prelude::astar;

use crate::{
    components::{
        grid_components::{
            definitions::{
                CellIndex1d,
                CellIndex2d,
                Occupation,
            },
            grid_related_iterators::CoordinateIterator,
            definitions::Grid2D,
        },
        movement_components::{
            Direction,
            SurfaceCoordinate,
        },
        pathfinding_components::{Pathfinder, PathfindingMap},
    },
    function_libs::grid_calculations::{
        self,
        normalize_rect,
    },
};

impl<'a> PathfindingMap<'a> {
    #[deprecated]
    pub fn calculate_path_coordinates_global_old(&self, grid2d: &Grid2D, pathfinder: Pathfinder)
                                                 -> (NavigationPath, Vec<SurfaceCoordinate>) {
        let path = self.calculate_path_normalized(pathfinder);
        let global_points = &self.convert_normalized_1d_to_global_points(&path.steps);
        (path, grid2d.calculate_surface_coordinates_for_2d(&global_points))
    }

    #[inline]
    pub fn calculate_path_coordinates_global(&self, pathfinder: Pathfinder)
                                             -> Option<(Vec<CellIndex2d>, u32)> {
        self.find_path_points(pathfinder)
    }

    #[inline]
    fn calculate_path_normalized(&self, pathfinder: Pathfinder) -> NavigationPath {
        //log what is going to happen, printing out the area as well
        println!("Calculating path from {} to {} in area {:?}", pathfinder.start, pathfinder.end, self.area);
        let width = self.area.width();
        a_star_search(grid_calculations::calculate_1d_index(pathfinder.start, width),
                      grid_calculations::calculate_1d_index(pathfinder.end, width), self)
    }

    #[inline]
    fn find_path_points(&self, pathfinder: Pathfinder) -> Option<(Vec<CellIndex2d>, u32)> {
        let result: Option<(Vec<CellIndex2d>, u32)> =
            astar(&pathfinder.start,
                  |p| self.calculate_successors(p),
                  |p| p.distance(&pathfinder.end) / 3,
                  |p| *p == pathfinder.end);
        result
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
        pathfinder.start = from;

        let closest_obstacle = self.find_closest_cell_in_direction_global(from,
                                                                          direction, Occupation::Occupied);

        if closest_obstacle.is_none() {
            return None;
        }
        let closest_empty_cell = self.find_closest_cell_in_direction_global(closest_obstacle.unwrap(),
                                                                            direction, Occupation::Free);

        if closest_empty_cell.is_some() {
            pathfinder.end = closest_empty_cell.unwrap();

            result = Some(pathfinder);
        }

        return result;
    }

    pub fn find_closest_cell_in_direction_global(&self, from_global_index: CellIndex2d, direction: Direction,
                                                 occupation_state: Occupation) -> Option<CellIndex2d> {
        let mut was_found = false;
        let local_index = self.grid_segment.global_to_local_index(from_global_index);
        let mut global_index: Option<CellIndex2d> = None;

        for cell_index in CoordinateIterator::iter_area_in_line_from(local_index, direction.as_vector(),
                                                                     self.area_normalized)
        {
            if self[cell_index].occupation_state != occupation_state {
                continue;
            }

            let global_index_temp = self.grid_segment.local_to_global_index(cell_index);
            // info!("{}", "We are iterating! {global_index_temp}".green());

            was_found = true;
            global_index = Some(global_index_temp);
            break;
        };
        return global_index;
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
            if self[outer_cell].occupation_state == Occupation::Free {
                successors.push((outer_cell, 1));
            }
        }

        successors
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
                exits.push((cell_index_1d as usize, 1.0));
            }
        }
        exits
    }
}
