use bevy::{
    log::info,
    math::Quat,
    prelude::{
        ButtonInput,
        MouseButton,
        Query,
        Res,
        ResMut,
        Transform,
        With,
    },
};
use bevy::math::URect;

use crate::{
    components::{
        flow_field_components::{Arrow, ExplosionParameters, FlowField},
        grid_components::{
            definitions::{
                CellIndex,
                Grid2D,
                GridRelatedData,
            },
            definitions::{GridSegment, ObstaclesParameters},
        },
        movement_components::{MoveTag, ObstacleTag},
        world_manipulation_components::CursorWorldPosition,
    }
};

pub fn rotate_flow_arrows_system(mut shapes_transform_query: Query<(&mut Transform, &CellIndex), With<Arrow>>,
                                 flow_field: Res<FlowField>) {
    for (mut transform, cell_index) in shapes_transform_query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(flow_field.get_rotation_angle_at(cell_index.as_ref()));
    }
}

pub fn flow_explosion_system(input: Res<ButtonInput<MouseButton>>, cursor_world_position: Res<CursorWorldPosition>,
                             grid_parameters: Res<Grid2D>, mut flow_field: ResMut<FlowField>) {
    if input.just_pressed(MouseButton::Left) {
        info!("LMB was pressed!");

        let world_pos = cursor_world_position.position;
        let hovered_cell_index = grid_parameters.calculate_cell_index_from_position(world_pos);
        flow_field.apply_smooth_explosion(&grid_parameters, ExplosionParameters::new(hovered_cell_index,
                                                                                     4.0));
    }
}

pub fn detraction_factor_calculation_system(mut grid_cell_data: ResMut<GridRelatedData>,
                                            grid_parameters: Res<Grid2D>,
                                            obstacles_parameters: Res<ObstaclesParameters>,
                                            query: Query<&CellIndex, With<ObstacleTag>>) {
    for obstacle_index in query.iter() {
        let central_index = obstacle_index.index;
        let segment_rect = URect::from_center_size(central_index.into(),
                                                   obstacles_parameters.influence_area);
        for cell_in_segment in grid_parameters.iter_coordinates_in_area(segment_rect) {
            grid_cell_data[cell_in_segment].detraction_factor = central_index.euclidean_distance(&cell_in_segment);
        }
    }
}
