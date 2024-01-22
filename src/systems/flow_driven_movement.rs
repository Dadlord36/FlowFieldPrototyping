use bevy::{
    math::{UVec2, Vec2},
    prelude::*,
};
use bevy::math::DVec2;

use crate::{
    bundles::movables::SurfaceWalkerBundle,
    components::{
        grid_components::{
            CellIndex, GridParameters, GridRelatedData,
        },
        movement_components::{
            MoveTag,
            SurfaceCoordinate,
        },
    },
    components::flow_field_components::FlowField,
};

pub fn spawn_moving_cubes(mut commands: Commands, grid_parameters: Res<GridParameters>)
{
    let columns_num = grid_parameters.column_number;
    let rows_num = grid_parameters.row_number;

    let cell_size: Vec2 = grid_parameters.cell_size / 2.0;
    let mut cell_index = UVec2::new(columns_num - 1, 0);

    let color = Color::ORANGE;

    for y in 0..rows_num {
        cell_index.y = y;

        let coordinate = SurfaceCoordinate::calculate_flat_surface_coordinate_from(&grid_parameters, cell_index);
        let mut coordinate_world_transform = coordinate.project_surface_coordinate_on_grid(&grid_parameters);
        coordinate_world_transform.translation.z = 10.0;

        commands.spawn(SurfaceWalkerBundle {
            surface_coordinate: coordinate,
            occupied_cell_index: CellIndex::from(cell_index),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(cell_size),
                    ..Default::default()
                },
                transform: coordinate_world_transform,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub fn adjust_coordinate_system(time: Res<Time>, flow_field: Res<FlowField>, grid_parameters: Res<GridParameters>,
                                mut query: Query<(&mut SurfaceCoordinate, &CellIndex), With<MoveTag>>)
{
    let delta: f64 = (0.1 * time.delta_seconds()) as f64;

    for (mut surface_calculations, cell_index) in query.iter_mut() {
        let direction: DVec2 = DVec2::from(flow_field.get_field_at(&grid_parameters, cell_index.index));

        surface_calculations.adjust_coordinate(direction * delta);
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
        cell_index.index = surface_calculations.calculate_cell_index_on_flat_surface(&grid_parameters);
    }
}

pub fn cell_occupation_highlight_system(grid_parameters: Res<GridParameters>, mut grid_cell_data: ResMut<GridRelatedData>,
                                        query: Query<&CellIndex, With<MoveTag>>)
{
    for cell_index in query.iter() {
        let cell_data = grid_cell_data.get_data_at_mut(&grid_parameters, cell_index.index);
        if cell_data.is_none() {
            continue;
        }
        cell_data.unwrap().color = Color::BLACK;
    }
}