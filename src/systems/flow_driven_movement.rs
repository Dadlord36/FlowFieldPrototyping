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
    let grid_center = Vec2::ZERO;
    let columns_num = grid_parameter.max_column_index;
    let rows_num = grid_parameter.max_row_index;

    let cell_size: Vec2 = grid_parameter.cell_size / 2.0;
    let mut cell_index = UVec2::new(columns_num, 0);

    let color = Color::ORANGE;

    for y in 0..rows_num {
        cell_index.y = y;

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

pub fn moving_system(time: Res<Time>, mut query: Query<(&mut Transform), With<MoveTag>>, flow_field: Res<FlowField>) {
    let delta = 100.0 * time.delta_seconds();  // adjust accordingly for your desired move speed

    for (mut transform) in query.iter_mut() {
        transform.translation.x -= delta;
    }
}

pub fn adjust_coordinate_system(time: Res<Time>,
                                mut query: Query<(&mut SurfaceCoordinate), With<MoveTag>>)
{
    let direction = Vec2::new(0.0, 1.0);
    for (mut surface_calculations) in query.iter_mut() {
        surface_calculations.adjust_coordinate(direction);
    }
}

pub fn apply_surface_coordinate_system(cylinder_parameters: Res<CylinderParameters>,
                                       mut query: Query<(&mut Transform, &SurfaceCoordinate),
                                           With<MoveTag>>) {
    for (mut transform, surface_calculations) in query.iter_mut() {
        *transform = surface_calculations.project_surface_coordinate_on_cylinder(&cylinder_parameters);
    }
}
