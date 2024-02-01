use bevy::log::{info, warn};
use bevy::math::{URect, UVec2};
use bracket_pathfinding::prelude::{a_star_search, Point};

use crate::{
    components::{
        grid_components::{
            CellIndex2d,
            Grid2D,
            CellIndex1d,
        },
        pathfinding_components::{Pathfinder, PathfindingMap},
    },
    tests::common,
};
use crate::components::grid_components::GridRelatedData;

#[test]
fn test_generate_map() {
    let grid: Grid2D = common::construct_default_grid();
    let grid_related_data = GridRelatedData::new(&grid);

    let map: PathfindingMap = grid_related_data.create_pathfinding_map(URect::from_center_size(UVec2::new(5, 5), UVec2::new(5, 5)));

    for (index, _element) in map.grid_segment_data.indexed_iter() {
        println!("{:?}", index);
    }
    let pathfinder = Pathfinder {
        referenced_grid: grid.clone(),
        start: CellIndex2d::new(0, 0),
        end: CellIndex2d::new(3, 3),
    };

    println!("Running A* Start: ({:?}), End: ({:?})", pathfinder.start, pathfinder.end);

    let path = map.calculate_path(pathfinder);

    assert!(path.steps.len() > 0, "No steps were made");
    match path.steps.len() {
        0 => warn!("No path found."),
        _ => {
            warn!("Path found, steps:");
            for idx in path.steps.iter() {
                println!("{idx}");
            }
        }
    }
    assert!(path.success);
}