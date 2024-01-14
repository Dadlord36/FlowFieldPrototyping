use bevy::math::{Quat, UVec2, Vec2};
use bevy::prelude::{Color, Commands, Component, Query, Res, SpatialBundle, Time, Transform, With};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::{Path, ShapeBundle},
    path::PathBuilder
};
use crate::{
    function_libs::{
        flow_field::{FlowField},
        grid_calculations,
        grid_calculations::GridParameters
    }
};

#[derive(Component)]
pub struct Arrow;

pub fn visualize_flow_system(mut _commands: Commands, grid_parameter: Res<GridParameters>, flow_field: Res<FlowField>) {
    // Create a new PathBuilder for the arrow shape
    for (col, row) in grid_parameter.coordinates() {
        let coordinate = UVec2::new(col, row);
        let cell_position = grid_calculations::calculate_cell_position(&grid_parameter, coordinate).extend(0.0);
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
        )).insert(Arrow);
    }
}

pub fn rotate_arrows_system(time: Res<Time>, mut shapes_transform_query: Query<&mut Transform, With<Arrow>>) {
    let rotation_speed: f32 = 1.5; // The speed at which the arrows will rotate (in radians per second)
    for mut transform in shapes_transform_query.iter_mut() {
        let rotation_increment = Quat::from_rotation_z(-rotation_speed * time.delta_seconds());
        transform.rotation = transform.rotation * rotation_increment;
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