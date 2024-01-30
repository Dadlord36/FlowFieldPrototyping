use bevy::{
    input::Input,
    math::Quat,
    prelude::{MouseButton, Query, Res, ResMut, Transform, With}
};
use bevy::log::info;
use crate::{
    components::{
        flow_field_components::{Arrow, ExplosionParameters, FlowField},
        grid_components::{CellIndex, Grid2D},
        world_manipulation_components::CursorWorldPosition
    }
};

pub fn rotate_flow_arrows_system(mut shapes_transform_query: Query<(&mut Transform, &CellIndex), With<Arrow>>,
                                 flow_field: Res<FlowField>) {
    for (mut transform, cell_index) in shapes_transform_query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(flow_field.get_rotation_angle_at(cell_index.index));
    }
}

pub fn flow_explosion_system(input: Res<Input<MouseButton>>, cursor_world_position: Res<CursorWorldPosition>,
                             grid_parameters: Res<Grid2D>, mut flow_field: ResMut<FlowField>) {
    if input.just_pressed(MouseButton::Left) {
        info!("LMB was pressed!");

        let world_pos = cursor_world_position.position;
        let hovered_cell_index = grid_parameters.calculate_cell_index_from_position(world_pos);
        flow_field.apply_smooth_explosion(&grid_parameters, ExplosionParameters::new(hovered_cell_index, 4.0));
    }
}

