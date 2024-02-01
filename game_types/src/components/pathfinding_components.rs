use std::ops::RangeInclusive;

use bevy::prelude::{Component, Resource};
use bracket_pathfinding::prelude::{a_star_search, BaseMap, NavigationPath, SmallVec};
use ndarray::ArrayView2;

use crate::components::grid_components::{CellIndex1d, CellIndex2d, Grid2D, GridCellData};
use crate::function_libs::grid_calculations;

#[derive(Component)]
pub struct Pathfinder {
    pub referenced_grid: Grid2D,
    pub start: CellIndex2d,
    pub end: CellIndex2d,
}

impl Pathfinder {
    pub fn get_start_as_index_1d(&self) -> CellIndex1d {
        self.referenced_grid.calc_cell_index_1d_at(self.start)
    }

    pub fn get_end_as_index_1d(&self) -> CellIndex1d {
        self.referenced_grid.calc_cell_index_1d_at(self.end)
    }
}

#[derive(Resource)]
pub struct PathfindingMap<'a> {
    pub grid_segment_data: ArrayView2<'a, GridCellData>,
    pub width: u32,
    pub height: u32,
}

impl<'a> PathfindingMap<'a> {
    #[inline]
    pub fn calculate_path(&self, pathfinder: Pathfinder) -> NavigationPath {
        a_star_search(grid_calculations::calculate_1d_index(pathfinder.start,self.width),
                      grid_calculations::calculate_1d_index(pathfinder.end,self.width), self)
    }
}

impl<'a> BaseMap for PathfindingMap<'a> {
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let central_cell = grid_calculations::calculate_2d_index(_idx as CellIndex1d, self.width);

        let mut exits = SmallVec::<[(usize, f32); 10]>::new();

        for (dx, dy) in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)] {
            let outer_cell = CellIndex2d::new(central_cell.x as i32 + dx, central_cell.y as i32 + dy);
            // Boundary check
            if outer_cell.x < self.width && outer_cell.y < self.height {
                // Assuming GridCellData has a traversal_cost field
                // let cell_data = &self.grid_segment_data[[outer_cell.x, outer_cell.y]];

                let cell_index_1d = grid_calculations::calculate_1d_index(outer_cell, self.width);
                exits.push((cell_index_1d as usize, 1f32));
            }
        }
        exits
    }
}