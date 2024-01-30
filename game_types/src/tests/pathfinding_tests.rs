use bevy::log::{error, info, warn};

use bracket_pathfinding::{
    prelude::{a_star_search, Point}
};

use crate::tests::common;

/*#[test]
fn test_generate_map() {
    let map: GridParameters = common::construct_default_grid();

    let pathfinder = Pathfinder {
        start: Point::new(5, 5),
        end: Point::new(95, 95),
    };

    let start_idx = map.idx(pathfinder.start);
    let end_idx = map.idx(pathfinder.end);

    if map.is_opaque(start_idx) {
        error!("Start point is non-passable.");
    }

    if map.is_opaque(end_idx) {
        error!("End point is non-passable.");
    }
    info!("Running A* Start: ({:?}), End: ({:?})", pathfinder.start, pathfinder.end);

    let path = a_star_search(start_idx, end_idx, &map);

    match path.steps.len() {
        0 => warn!("No path found."),
        _ => {
            warn!("Path found, steps:");
            for idx in path.steps.iter() {
                let Point { x, y } = map.point_from_idx(*idx);
                info!("{},{}", x, y);
            }
        }
    }
    assert!(path.success);
}*/