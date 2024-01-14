use bevy::prelude::{Color, Commands, Component, Res, Sprite, SpriteBundle, Transform};
use bevy::math::{UVec2, Vec2};
use crate::function_libs::grid_calculations::GridParameters;

#[derive(Component, Clone, Default)]
pub struct CellIndex {
    index: UVec2,
}

impl From<UVec2> for CellIndex {
    fn from(item: UVec2) -> Self {
        CellIndex {
            index: item,
        }
    }
}

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

    for (i, j) in grid_parameter.coordinates() {
        let color = if (i + j) % 2 == 0 { color1 } else { color2 };

        // Adjust the cell's position so the grid is centered at (0, 0)
        let position = Vec2::new((i as f32 * (cell_size.x + cell_spacing)) - grid_size_x / 2.0 + cell_size.x / 2.0,
                                 (j as f32 * (cell_size.y + cell_spacing)) - grid_size_y / 2.0 + cell_size.y / 2.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(cell_size),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        });
    }
}




