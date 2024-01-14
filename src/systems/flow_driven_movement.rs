use bevy::math::{UVec2, Vec2};
use bevy::prelude::*;

use crate::{
    function_libs::{
        grid_calculations::*,
        surface_calculations,
        flow_field::FlowField,
    }
};
use crate::function_libs::surface_calculations::{CylinderParameters, SurfaceCoordinate};
use crate::systems::grid_related::CellIndex;

#[derive(Component, Clone, Default)]
pub struct MoveTag;

#[derive(Bundle, Clone, Default)]
pub struct SurfaceWalkerBundle {
    surface_coordinate: SurfaceCoordinate,
    occupied_cell_index: CellIndex,
    sprite_bundle: SpriteBundle,
    move_tag: MoveTag,
}

pub fn spawn_moving_cubes(mut commands: Commands, grid_parameter: Res<GridParameters>)
{
    let columns_num = grid_parameter.column_number;
    let rows_num = grid_parameter.row_number;

    let cell_size: Vec2 = grid_parameter.cell_size / 2.0;
    let mut cell_index = UVec2::new(0, 0);

    let color = Color::ORANGE;

    for x in 0..columns_num {
        cell_index.x = x;

        let coordinate = SurfaceCoordinate::calculate_flat_surface_coordinate_from(&grid_parameter, cell_index);

        commands.spawn(SurfaceWalkerBundle {
            surface_coordinate: coordinate.clone(),
            occupied_cell_index: CellIndex::from(cell_index),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(cell_size),
                    ..Default::default()
                },
                transform: coordinate.project_surface_coordinate_on_grid(&grid_parameter),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub fn adjust_coordinate_system(time: Res<Time>, flow_field: Res<FlowField>, grid_parameters: Res<GridParameters>,
                                mut query: Query<(&mut SurfaceCoordinate, &CellIndex), With<MoveTag>>)
{
    let delta = 10.0 * time.delta_seconds();

    for (mut surface_calculations, cell_index) in query.iter_mut() {
        let direction = -flow_field.get_field_at(&grid_parameters, cell_index.index);

        surface_calculations.adjust_coordinate(direction);
    }
}

pub fn apply_surface_coordinate_system(grid_parameters: Res<GridParameters>,
                                       mut query: Query<(&mut Transform, &SurfaceCoordinate),
                                           With<MoveTag>>) {
    for (mut transform, surface_calculations) in query.iter_mut() {
        *transform = surface_calculations.project_surface_coordinate_on_grid(&grid_parameters);
    }
}

pub fn grid_relation_system(grid_parameters: Res<GridParameters>,
                            mut query: Query<(&mut CellIndex, &SurfaceCoordinate),
                                With<MoveTag>>)
{
    for (mut cell_index, surface_calculations) in query.iter_mut() {
        cell_index.index = surface_calculations.calculate_cell_index(&grid_parameters);
    }
}
