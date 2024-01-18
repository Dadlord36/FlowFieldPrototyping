use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Color, Commands, Query, Res, ResMut, Sprite, SpriteBundle, Transform, With};

use crate::components::grid_components::{CellIndex, GridCellTag, GridParameters, GridRelatedData};

pub fn spawned_colorized_cells_system(mut commands: Commands, grid_parameter: Res<GridParameters>)
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

    for (col, row) in grid_parameter.coordinates() {
        let color = if (col + row) % 2 == 0 { color1 } else { color2 };

        // Adjust the cell's position so the grid is centered at (0, 0)
        let position = Vec2::new((col as f32 * (cell_size.x + cell_spacing)) - grid_size_x / 2.0 + cell_size.x / 2.0,
                                 (row as f32 * (cell_size.y + cell_spacing)) - grid_size_y / 2.0 + cell_size.y / 2.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(cell_size),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        }).insert(GridCellTag).insert(CellIndex::from(UVec2::new(col, row)));
    }
}

pub fn reset_cells_colorization(grid_parameters: Res<GridParameters>, mut grid_cell_data: ResMut<GridRelatedData>) {
    let mut color1 = Color::YELLOW_GREEN;
    color1.set_a(0.2);
    let mut color2 = Color::GRAY;
    color2.set_a(0.2);

    for (i, j) in grid_parameters.coordinates() {
        let color = if (i + j) % 2 == 0 { color1 } else { color2 };
        let index = UVec2::new(i, j);

        grid_cell_data.get_data_at_mut(&grid_parameters, index).unwrap().color = color;
    }
}

pub fn apply_color_to_cell(grid_parameters: Res<GridParameters>, grid_cell_data: Res<GridRelatedData>,
                           mut cells_query: Query<(&CellIndex, &mut Sprite), With<GridCellTag>>) {
    for (cell_index, mut sprite) in cells_query.iter_mut() {
        let cell_data = grid_cell_data.get_data_at(&grid_parameters, cell_index.index);
        if cell_data.is_none() {
            continue;
        };
        sprite.color = cell_data.unwrap().color;
    }
}