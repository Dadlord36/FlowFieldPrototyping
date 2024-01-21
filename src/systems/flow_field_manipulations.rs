use bevy::{
    log::info,
    input::Input,
    math::{Quat, UVec2, Vec2},
    prelude::{Color, Commands, MouseButton, Query, Res, ResMut, SpatialBundle, Transform, With},
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    path::PathBuilder,
};

use crate::{
    function_libs::{
        grid_calculations::{
            self
        },
    },
    components::{
        flow_field_components::{
            Arrow,
            FlowField,
            ExplosionParameters,
        },
        grid_components::{
            GridParameters,
            CellIndex,
        },
        world_manipulation_components::CursorWorldPosition,
    },
};

pub fn visualize_flow_system(mut _commands: Commands, grid_parameter: Res<GridParameters>, flow_field: Res<FlowField>) {
    // Create a new PathBuilder for the arrow shape
    for (col, row) in grid_parameter.coordinates() {
        let coordinate = UVec2::new(col, row);
        let cell_position = grid_parameter.calculate_cell_position(coordinate).extend(0.0);
        let mut new_transform = Transform::from_xyz(cell_position.x, cell_position.y, cell_position.z);

        new_transform.rotation = Quat::from_rotation_z(flow_field.get_rotation_angle_at(&grid_parameter, coordinate));
        // Spawn an entity with the arrow shape, positioned at the cell's location
        // and rotated to match the flow direction
        _commands.spawn((ShapeBundle {
            path: build_arrow_shape(25f32, 10f32),
            spatial: SpatialBundle {
                transform: new_transform,
                ..Default::default()
            },
            ..Default::default()
        }, Stroke::new(Color::BLACK, 1.0), Fill::color(Color::RED),
        )).insert(Arrow).insert(CellIndex::new(coordinate));
    }
}

pub fn rotate_flow_arrows_system(mut shapes_transform_query: Query<(&mut Transform, &CellIndex), With<Arrow>>,
                                 grid_parameter: Res<GridParameters>, flow_field: Res<FlowField>) {
    for (mut transform, cell_index) in shapes_transform_query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(flow_field.get_rotation_angle_at(&grid_parameter, cell_index.index));
    }
}

pub fn flow_explosion_system(input: Res<Input<MouseButton>>, cursor_world_position: Res<CursorWorldPosition>,
                             grid_parameters: Res<GridParameters>, mut flow_field: ResMut<FlowField>) {
    if input.just_pressed(MouseButton::Left) {
        info!("LMB was pressed!");

        let world_pos = cursor_world_position.position;
        let hovered_cell_index = grid_parameters.calculate_cell_index_from_position(world_pos);
        flow_field.apply_smooth_explosion(&grid_parameters, ExplosionParameters::new(hovered_cell_index, 4.0));
    }
}

fn build_arrow_shape(length: f32, width: f32) -> Path {
    // Create a new PathBuilder for the arrow shape
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0., 0.)); // base of the arrow
    path_builder.line_to(Vec2::new(width / 2., -length / 3.)); // left wing of the arrow
    path_builder.line_to(Vec2::new(0., -length)); // top of the arrow - now points down
    path_builder.line_to(Vec2::new(-width / 2., -length / 3.)); // right wing of the arrow
    path_builder.line_to(Vec2::new(0., 0.)); // closing the path back at base
    path_builder.close();
    path_builder.build()
}