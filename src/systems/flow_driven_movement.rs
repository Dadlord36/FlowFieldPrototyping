use bevy::{
    math::UVec2,
    prelude::{Color, Commands, Res},
};
use game_types::{
    components::grid_components::definitions::{CellIndex2d, Grid2D},
    systems::flow_driven_movement::calculate_coordination_data,
};


use crate::systems::flow_field_related;

pub fn spawn_moving_cubes(mut commands: Commands, grid_parameters: Res<Grid2D>)
{
    let columns_num = grid_parameters.column_number;
    let rows_num = grid_parameters.row_number;

    let mut cell_index = UVec2::new(columns_num - 1, 0);
    let color = Color::ORANGE;

    for y in 0..rows_num {
        cell_index.y = y;

        spawn_movable_actor_on_grid(&mut commands, &grid_parameters, cell_index.into(), color);
    }
}

fn spawn_movable_actor_on_grid(mut commands: &mut Commands, grid_parameters: &Res<Grid2D>,
                               cell_index: CellIndex2d, color: Color)
{
    let (coordinate, coordinate_world_transform, actor_size) =
        calculate_coordination_data(&grid_parameters, cell_index);

    flow_field_related::spawn_movable_actor(&mut commands, cell_index, color, actor_size, coordinate,
                                            coordinate_world_transform);
}
