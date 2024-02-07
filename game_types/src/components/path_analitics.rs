use bracket_pathfinding::prelude::{a_star_search, BaseMap, NavigationPath, SmallVec};
use crate::{
    components::{
        grid_components::definitions::{CellIndex1d, CellIndex2d},
        pathfinding_components::{Pathfinder, PathfindingMap},
        grid_components::definitions::Occupation,
        movement_components::Direction,
    },
    function_libs::grid_calculations,
};
use crate::components::grid_components::grid_related_traits::CoordinateIterator;

impl<'a> PathfindingMap<'a> {
    #[inline]
    pub fn calculate_path(&self, pathfinder: Pathfinder) -> NavigationPath {
        a_star_search(grid_calculations::calculate_1d_index(pathfinder.start, self.area.height()),
                      grid_calculations::calculate_1d_index(pathfinder.end, self.area.width()), self)
    }

    pub fn find_destination_in_direction(&self, from: CellIndex2d, direction: Direction) -> Pathfinder {
        let direction_vector = direction.as_vector();
        let mut pathfinder = Pathfinder::ZERO;
        pathfinder.start = from;
        for cell_index in CoordinateIterator::iter_area_in_line_from(from, direction_vector, self.area) {
            if self.grid_segment_data[&cell_index].occupation_state == Occupation::Occupied {
                println!()
            }
        }

        return pathfinder;
    }
}

impl<'a> BaseMap for PathfindingMap<'a> {
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let width = self.area.width();
        let height = self.area.height();

        let central_cell = grid_calculations::calculate_2d_index(_idx as CellIndex1d, width);

        let mut exits = SmallVec::<[(usize, f32); 10]>::new();

        for direction in [Direction::North.as_vector(), Direction::East.as_vector(), Direction::South.as_vector(), Direction::West.as_vector()]
        {
            let outer_cell = CellIndex2d::new(central_cell.x as i32 + direction.x, central_cell.y as i32 + direction.y);
            // Boundary check
            if outer_cell.x < width && outer_cell.y < height {
                // Assuming GridCellData has a traversal_cost field
                let cell_data = &self.grid_segment_data[&outer_cell];
                if cell_data.occupation_state == Occupation::Occupied {
                    continue;
                }

                let cell_index_1d = grid_calculations::calculate_1d_index(outer_cell, width);
                exits.push((cell_index_1d as usize, 1f32));
            }
        }
        exits
    }
}
