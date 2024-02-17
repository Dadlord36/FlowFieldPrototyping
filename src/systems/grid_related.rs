use bevy::{
    math::{Vec2},
    prelude::{Color, Commands, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};
use bevy::prelude::ResMut;

use game_types::components::grid_components::definitions::{CellIndex, Grid2D, GridCellTag, GridRelatedData, Occupation};

pub fn spawned_colorized_cells_system(mut commands: Commands, grid_parameter: Res<Grid2D>)
{
    let columns_num = grid_parameter.column_number;
    let rows_num = grid_parameter.row_number;

    let cell_size: Vec2 = grid_parameter.cell_size;
    let cell_spacing = grid_parameter.cells_spacing;

    // calculate the total size of the grid
    let grid_size_x = columns_num as f32 * (cell_size.x + cell_spacing);
    let grid_size_y = rows_num as f32 * (cell_size.y + cell_spacing);

    let mut color1 = Color::YELLOW_GREEN;
    color1.set_a(0.2);
    let mut color2 = Color::GRAY;
    color2.set_a(0.2);

    for cell_index in grid_parameter.iter_coordinates() {
        let color = if (cell_index.x + cell_index.y) % 2 == 0 { color1 } else { color2 };

        // Adjust the cell's position so the grid is centered at (0, 0)
        let position = Vec2::new((cell_index.x as f32 * (cell_size.x + cell_spacing)) - grid_size_x / 2.0 + cell_size.x / 2.0,
                                 (cell_index.y as f32 * (cell_size.y + cell_spacing)) - grid_size_y / 2.0 + cell_size.y / 2.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(cell_size),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        }).insert(GridCellTag).insert(CellIndex::new(cell_index));
    }
}

pub fn colorize_obstacles_system(grid_parameters: Res<Grid2D>,
                                 mut grid_related_data: ResMut<GridRelatedData>) {
    for coordinate in grid_parameters.iter_coordinates() {
        let grid_cell_data = grid_related_data.get_data_at_mut(&coordinate);
        let cell_color: Color;
        match grid_cell_data.occupation_state {
            Occupation::Free => { continue; }
            Occupation::Occupied => { cell_color = Color::BLACK }
        }
        grid_cell_data.color = cell_color;
    }
}