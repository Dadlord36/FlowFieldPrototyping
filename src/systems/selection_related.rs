use bevy::input::mouse::MouseMotion;
use bevy::log::warn;
use bevy::prelude::{Camera2d, Color, CursorMoved, EventReader, GlobalTransform, Query, Res, ResMut, Transform, Window, With};
use bevy::window::PrimaryWindow;
use crate::{
    components::{
        world_manipulation_components::{
            HoverCell,
            CursorWorldPosition,
        },
        grid_components::{GridParameters, GridRelatedData},
    },
    function_libs::{
        grid_calculations::{
            self
        },
        coordinates_calculations::screen_to_world,
    },
};

pub fn mouse_hover_system(mut cursor_moved_events: EventReader<CursorMoved>, cursor_world_position: Res<CursorWorldPosition>,
                          mut grid_cell_data: ResMut<GridRelatedData>, grid_parameters: Res<GridParameters>)
{
    // Since the mouse can move multiple times per frame, only keep the last position
    /*    if let Some(_cursor_moved) = cursor_moved_events.read_with_id().last()
        {*/
    let mut selection_color = Color::ORANGE;
    selection_color.set_a(0.3);

    let world_pos = cursor_world_position.position;
    // Calculate the cell index
    if grid_parameters.rect.contains(world_pos) {
        let hovered_cell_index = grid_calculations::calculate_cell_index_from_position(&grid_parameters, world_pos);

        /*       if state.prev_cell != hovered_cell_index
               {
                   state.prev_cell = hovered_cell_index;*/

        let cells_in_range = grid_calculations::calculate_indexes_in_circle_from_index(&grid_parameters,
                                                                                       hovered_cell_index, 3);
        for cell_index in cells_in_range {
            let mut cell_data = grid_cell_data.get_data_at_mut(&grid_parameters, cell_index);
            if cell_data.is_none() {
                warn!("Cell data is invalid!");
                continue;
            }
            cell_data.unwrap().color = selection_color;
        }
        // }
    }
    // }
}

pub fn capture_cursor_position(mut mouse_motion_events: EventReader<MouseMotion>,
                               grid_parameters: Res<GridParameters>, mut hover_cell: ResMut<HoverCell>,
                               mut cursor_position: ResMut<CursorWorldPosition>,
                               q_windows: Query<&Window, With<PrimaryWindow>>,
                               camera_query: Query<(&Transform, &GlobalTransform), With<Camera2d>>)
{
    for _ in mouse_motion_events.read() {
        // Access the main window
        let window = q_windows.single();

        if let Some(position) = q_windows.single().cursor_position() {
            // Get the camera transform.
            let (camera_transform, _global_transform) = camera_query.single();
            // Calculate the world position.
            let world_position = screen_to_world(position, window, camera_transform);
            cursor_position.position = world_position;
            let hovered_cell_index = grid_calculations::calculate_cell_index_from_position(&grid_parameters, world_position);
            hover_cell.hovered_cell = hovered_cell_index;
        }
    }
}