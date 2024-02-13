use bracket_pathfinding::prelude::{a_star_search, BaseMap, NavigationPath, SmallVec};

use crate::{
    components::{
        grid_components::{
            definitions::{
                CellIndex1d,
                CellIndex2d,
                Occupation,
            },
            grid_related_iterators::CoordinateIterator
            ,
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
use crate::components::grid_components::definitions::Grid2D;

impl<'a> PathfindingMap<'a> {
    #[inline]
    pub fn calculate_path_normalized(&self, pathfinder: Pathfinder) -> NavigationPath {
        //log what is going to happen, printing out the area as well
        println!("Calculating path from {} to {} in area {:?}", pathfinder.start, pathfinder.end, self.area);
        let width = self.area.width();
        let normalized_start = pathfinder.start;
        let normalized_target = pathfinder.end;
        a_star_search(grid_calculations::calculate_1d_index(normalized_start, width),
                      grid_calculations::calculate_1d_index(normalized_target, width), self)
    }

    pub fn calculate_path_coordinates_global(&self, grid2d: &Grid2D, pathfinder: Pathfinder) -> (NavigationPath, Vec<SurfaceCoordinate>) {
        let path = self.calculate_path_normalized(pathfinder);
        let global_points = &self.convert_normalized_to_global_points(&path.steps);
        (path, grid2d.calculate_surface_coordinates_for_2d(&global_points))
    }

    fn convert_normalized_to_global_points(&self, normalized_points: &Vec<usize>) -> Vec<CellIndex2d> {
        //preallocate the array of the same size
        let mut global_points = Vec::with_capacity(normalized_points.len());

        for normalized_point in normalized_points.iter() {
            let normalized_cell_index1d: CellIndex1d = normalized_point.clone() as CellIndex1d;
            let global_cell_index2d = self.grid_segment.normalized_to_global_index(normalized_cell_index1d);
            global_points.push(global_cell_index2d)
        }

        global_points
    }

    pub fn find_destination_in_direction(&self, from: CellIndex2d, direction: Direction) -> Option<Pathfinder> {
        let mut result: Option<Pathfinder> = None;
        let mut pathfinder = Pathfinder::ZERO;
        pathfinder.start = self.grid_segment.from_global_to_normalized_index(from);

        let closest_obstacle = self.find_closest_cell_in_direction_global(from, direction,
                                                                          Occupation::Occupied);
        if closest_obstacle.is_none() {
            return None;
        }
        let closest_empty_cell = self.find_closest_cell_in_direction_global(closest_obstacle.unwrap(),
                                                                            direction, Occupation::Free);

        if closest_empty_cell.is_some() {
            pathfinder.end = closest_empty_cell.unwrap();

            result = Some(pathfinder);
        }

        // asset that pathfinder points are within bounds of the area, printing out message if not
        /*        assert!(self.area.contains(pathfinder.start.into()), "Start point is outside the area");
                assert!(self.area.contains(pathfinder.end.into()), "End point is outside the area");*/

        return result;
    }

    pub fn find_closest_cell_in_direction_global(&self, from_global_index: CellIndex2d, direction: Direction,
                                                 occupation_state: Occupation) -> Option<CellIndex2d> {
        // let mut log_collector: String = Default::default();
        let mut was_found = false;
        let local_index = self.grid_segment.to_local_index(from_global_index);
        let mut global_index: Option<CellIndex2d> = None;

        for cell_index in CoordinateIterator::iter_area_fully_from(local_index,
                                                                   direction.as_vector(),
                                                                   normalize_rect(self.area))
        {
            // Collecting the info messages
            /*            let log_msg = format!("Iterating through indexes: {:?}\n", cell_index);
                        log_collector.push_str(&log_msg);
                        info!(log_msg);*/
            if self[cell_index].occupation_state != occupation_state {
                continue;
            }

            /*            let log_msg_2 = format!("Closest occupied cell:{:?}\n", cell_index);
                        log_collector.push_str(&log_msg_2);*/
            was_found = true;
            global_index = Some(self.grid_segment.from_local_to_global_index(cell_index));
        };
        return global_index;
    }
}


impl<'a> BaseMap for PathfindingMap<'a> {
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let width = self.area.width();

        let central_cell = grid_calculations::calculate_2d_index(_idx as CellIndex1d, width);
        assert!(self.grid_segment.contains_normalized(central_cell), "Cell index {central_cell} is outside of {:?}", self.area);

        let mut exits = SmallVec::<[(usize, f32); 10]>::new();

        for direction in [Direction::North.as_vector(), Direction::East.as_vector(),
            Direction::South.as_vector(), Direction::West.as_vector()]
        {
            let outer_cell = central_cell + direction;

            // Boundary check
            if !self.grid_segment.contains_normalized(outer_cell.into()) || self[outer_cell].occupation_state != Occupation::Free {
                continue;
            }

            let cell_index_1d = grid_calculations::calculate_1d_index(outer_cell, width);
            exits.push((cell_index_1d as usize, 1f32));
        }
        exits
    }
}
