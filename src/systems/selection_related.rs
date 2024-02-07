use bevy::{
    input::mouse::MouseMotion,
    prelude::{Camera2d, EventReader, GlobalTransform, Query, Res, ResMut, Transform, Window, With},
    window::PrimaryWindow,
};
use bevy::prelude::{Input, KeyCode, Vec3};

use game_types::{
    components::{
        grid_components::definitions::Grid2D,
        world_manipulation_components::{CursorWorldPosition, HoverCell},
    },
    function_libs::coordinates_calculations::screen_to_world,
};

pub fn capture_cursor_position(mut mouse_motion_events: EventReader<MouseMotion>,
                               grid_parameters: Res<Grid2D>, mut hover_cell: ResMut<HoverCell>,
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
            hover_cell.hovered_cell = grid_parameters.calculate_cell_index_from_position(world_position);
        }
    }
}

pub fn move_camera_system(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera2d>>) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let speed = 1.0;

        if input.pressed(KeyCode::Left) {
            direction -= Vec3::new(speed, 0.0, 0.0);
        }

        if input.pressed(KeyCode::Right) {
            direction += Vec3::new(speed, 0.0, 0.0);
        }

        if input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, speed, 0.0);
        }

        if input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, speed, 0.0);
        }

        transform.translation += direction;
    }
}