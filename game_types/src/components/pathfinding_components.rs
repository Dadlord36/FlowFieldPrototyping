use bevy::prelude::{Component, Resource};
use bracket_pathfinding::prelude::Point;
use ndarray::ArrayView2;

use crate::components::grid_components::GridCellData;

#[derive(Component)]
pub struct Pathfinder {
    pub start: Point,
    pub end: Point,
}

#[derive(Resource)]
pub struct PathfindingMap<'a> {
    pub grid_segment_data: ArrayView2<'a, GridCellData>,
    pub width: i32,
    pub height: i32,
}

/*impl<'a> BaseMap for PathfindingMap<'a> {
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let index2d = grid_calculations::calculate_2d_index(_idx, self.width);
        let x = index2d.x;
        let y = index2d.y;

        let mut exits = Vec::new();

        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            // Boundary check
            if new_x < self.width && new_y < self.height {
                // Assuming GridCellData has a traversal_cost field
                let cell = self.grid_segment_data[[new_x, new_y]];

                exits.push((NavPoint::new(new_x, new_y).to_one_dim(self.width), cell.traversal_cost));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        todo!()
    }
}*/