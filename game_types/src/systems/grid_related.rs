use bevy::{
    prelude::{Color, Query, Res, ResMut, With},
    sprite::Sprite,
};
use bevy::prelude::Time;
use crate::components::directions::Direction;

use crate::components::grid_components::definitions::{CellIndex, ElapsedTimeTracker, Grid2D, GridCellTag,
                                                      GridRelatedData};

pub fn reset_cells_colorization(grid: Res<Grid2D>, mut grid_cell_data: ResMut<GridRelatedData>) {
    let mut color1 = Color::YELLOW_GREEN;
    color1.set_a(0.2);
    let mut color2 = Color::GRAY;
    color2.set_a(0.2);

    for cell_index2d in grid.iter_coordinates() {
        let mut color = if (cell_index2d.x + cell_index2d.y) % 2 == 0 { color1 } else { color2 };
        for segment in grid.segments.iter() {
            if segment.1.contains(cell_index2d.into()) {
                color = get_color_for(segment.0.clone());
                break;
            }
        }
        grid_cell_data.get_data_at_mut(&cell_index2d).color = color;
    }
}

pub fn apply_color_to_cell(grid_cell_data: Res<GridRelatedData>,
                           mut cells_query: Query<(&CellIndex, &mut Sprite), With<GridCellTag>>) {
    for (cell_index, mut sprite) in cells_query.iter_mut() {
        let cell_data = grid_cell_data.get_data_at(cell_index.as_ref());
        sprite.color = cell_data.color;
    }
}

pub fn visualize_grid_data_in_log(time: Res<Time>, mut elapsed_time_tracker: ResMut<ElapsedTimeTracker>,
                                  grid2d: Res<Grid2D>, grid_data: Res<GridRelatedData>) {
    if time.elapsed_seconds() - elapsed_time_tracker.time_stamp > 2.5 {
        // grid_data.visualize_on_grid(&*grid2d);
        grid2d.visualize_indexes_in_log();
        elapsed_time_tracker.time_stamp = time.elapsed_seconds();
    }
}

pub fn visualize_grid_in_log(grid2d: Res<Grid2D>)
{
    grid2d.visualize_indexes_in_log();
}

fn get_color_for(direction: Direction) -> Color {
    match direction {
        Direction::North => Color::rgb(1.0, 0.0, 0.0),
        Direction::NorthEast => Color::rgb(1.0, 1.0, 0.0),
        Direction::East => Color::rgb(0.0, 1.0, 0.0),
        Direction::SouthEast => Color::rgb(0.0, 1.0, 1.0),
        Direction::South => Color::rgb(0.0, 0.0, 1.0),
        Direction::SouthWest => Color::rgb(1.0, 0.0, 1.0),
        Direction::West => Color::rgb(1.0, 0.5, 0.0),
        Direction::NorthWest => Color::rgb(0.5, 0.5, 0.5),
    }
}