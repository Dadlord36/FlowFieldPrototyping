use bevy::{
    prelude::{Color, Query, Res, ResMut, With},
    sprite::Sprite
};

use crate::components::grid_components::{CellIndex, CellIndex2d, Grid2D, GridCellTag, GridRelatedData};

pub fn reset_cells_colorization(grid_parameters: Res<Grid2D>, mut grid_cell_data: ResMut<GridRelatedData>) {
    let mut color1 = Color::YELLOW_GREEN;
    color1.set_a(0.2);
    let mut color2 = Color::GRAY;
    color2.set_a(0.2);

    for (i, j) in grid_parameters.iterate_coordinates() {
        let color = if (i + j) % 2 == 0 { color1 } else { color2 };
        let index = CellIndex2d::new(i, j);

        grid_cell_data.get_data_at_mut(index).unwrap().color = color;
    }
}

pub fn apply_color_to_cell(grid_cell_data: Res<GridRelatedData>, mut cells_query: Query<(&CellIndex, &mut Sprite), With<GridCellTag>>) {
    for (cell_index, mut sprite) in cells_query.iter_mut() {
        let cell_data = grid_cell_data.get_data_at(cell_index.clone().into());
        if cell_data.is_none() {
            continue;
        };
        sprite.color = cell_data.unwrap().color;
    }
}