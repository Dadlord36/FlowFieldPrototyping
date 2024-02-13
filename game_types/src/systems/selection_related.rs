use bevy::{
    log::warn,
    prelude::{Color, CursorMoved, EventReader, Res, ResMut},
};
use crate::{
    components::{
        grid_components::definitions::{Grid2D, GridRelatedData},
        world_manipulation_components::CursorWorldPosition,
    },
    function_libs::grid_calculations,
};

pub fn mouse_hover_system(_cursor_moved_events: EventReader<CursorMoved>, cursor_world_position: Res<CursorWorldPosition>,
                          mut grid_cell_data: ResMut<GridRelatedData>, grid_parameters: Res<Grid2D>)
{
    // Since the mouse can move multiple times per frame, only keep the last position
    /*    if let Some(_cursor_moved) = cursor_moved_events.read_with_id().last()
        {*/
    let mut selection_color = Color::ORANGE;
    selection_color.set_a(0.3);

    let world_pos = cursor_world_position.position;
    // Calculate the cell index
    if grid_parameters.shape_rect.contains(world_pos) {
        let hovered_cell_index = grid_parameters.calculate_cell_index_from_position(world_pos);

        /*       if state.prev_cell != hovered_cell_index
               {
                   state.prev_cell = hovered_cell_index;*/

        let cells_in_range = grid_calculations::calculate_indexes_in_circle_from_index(&grid_parameters,
                                                                                       hovered_cell_index, 3);
        for cell_index in cells_in_range {
            let cell_data = grid_cell_data.get_data_at_mut(&cell_index);
            cell_data.color = selection_color;
        }
        // }
    }
    // }
}

