use bevy::{
    math::Quat,
    prelude::{Color, Commands, Res, SpatialBundle, Transform}
};
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{Sprite, SpriteBundle};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    path::PathBuilder,
    prelude::Path
};
use game_types::{
    components::{
        flow_field_components::{Arrow, FlowField},
        grid_components::{CellIndex, CellIndex2d, Grid2D}
    }
};
use game_types::components::movement_components::{Maneuver, SurfaceCoordinate};
use game_types::systems::{flow_driven_movement};
use crate::bundles::movables::SurfaceWalkerBundle;

pub fn visualize_flow_system(mut _commands: Commands, grid_parameter: Res<Grid2D>, flow_field: Res<FlowField>) {
    // Create a new PathBuilder for the arrow shape
    for (col, row) in grid_parameter.iterate_coordinates() {
        let coordinate = CellIndex2d::new(col, row);
        let cell_position = grid_parameter.calculate_cell_position(coordinate).extend(0.0);
        let mut new_transform = Transform::from_xyz(cell_position.x, cell_position.y, cell_position.z);

        new_transform.rotation = Quat::from_rotation_z(flow_field.get_rotation_angle_at(coordinate));
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

pub fn spawn_dummy_path_driven_actor(mut commands: Commands, grid_parameters: Res<Grid2D>) {
    let cell_index: UVec2 = UVec2::new(10, 10);

    let (coordinate, coordinate_world_transform, actor_size) =
        flow_driven_movement::calculate_coordination_data(&grid_parameters, cell_index.into());

    /*    let maneuver_points =
            vec![grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(0, 0)),
                 grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(0, 1)),
                 grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(1, 1)),
                 grid_parameters.calculate_flat_surface_coordinate_from(UVec2::new(1, 0))];*/

    let maneuver = Maneuver::spiral(&grid_parameters);

    /*    for maneuver_point in maneuver.path_points.iter() {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::AZURE,
                    custom_size: Some(actor_size),
                    ..Default::default()
                },
                transform: maneuver_point.project_surface_coordinate_on_grid(&grid_parameters),
                ..Default::default()
            }
            );
        }*/

    commands.spawn(SurfaceWalkerBundle {
        surface_coordinate: coordinate,
        occupied_cell_index: CellIndex::new(cell_index.into()),
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE_RED,
                custom_size: Some(actor_size),
                ..Default::default()
            },
            transform: coordinate_world_transform,
            ..Default::default()
        },
        ..Default::default()
    }).insert(maneuver);
}

pub(crate) fn spawn_movable_actor(commands: &mut Commands, cell_index: CellIndex2d, color: Color, actor_size: Vec2,
                                  coordinate: SurfaceCoordinate, mut coordinate_world_transform: Transform) {
    commands.spawn(SurfaceWalkerBundle {
        surface_coordinate: coordinate,
        occupied_cell_index: CellIndex::from(cell_index),
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(actor_size),
                ..Default::default()
            },
            transform: coordinate_world_transform,
            ..Default::default()
        },
        ..Default::default()
    });
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