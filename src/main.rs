// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use bevy::{
    asset::AssetMetaCheck,
    DefaultPlugins,
    window::PrimaryWindow,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

use bevy_game;

mod function_libs;
mod systems;

use function_libs::{
    flow_field::FlowField,
    grid_calculations,
};

use systems::{
    flow_driven_movement::*,
    flow_field_manipulations::*,
    grid_related::spawned_colorized_cells_system,
};
use crate::function_libs::grid_calculations::{calculate_cell_index_from_position, GridParameters};


fn main() {
    let grid_parameters = GridParameters::new(25, 25, Vec2::new(50f32, 50f32));
    let flow_field = FlowField::form_field(grid_parameters.column_number, grid_parameters.row_number);

    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // The canvas size is constrained in index.html and build/web/styles.css
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_game::GamePlugin)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (spawned_colorized_cells_system, spawn_moving_cubes, visualize_flow_system))
        .add_systems(Update, (capture_cursor_position, mouse_hover_system))
        .add_systems(Update, (adjust_coordinate_system, apply_surface_coordinate_system, grid_relation_system))
        .insert_resource(grid_parameters)
        .insert_resource(flow_field)
        .insert_resource(CursorWorldPosition {
            position: Vec2::ZERO,
        })
        // .add_systems(Startup, set_window_icon)
        .run();
}


fn mouse_hover_system(mut cursor_moved_events: EventReader<CursorMoved>, cursor_world_position: Res<CursorWorldPosition>,
                      grid_parameter: Res<GridParameters>, mut state: Local<HoverState>,  // Cache state
) {
    let grid_center = Vec2::ZERO;
    // Since the mouse can move multiple times per frame, only keep the last position
    if let Some(_cursor_moved) = cursor_moved_events.read_with_id().last() {
        let world_pos = cursor_world_position.position;
        // Calculate the cell index
        if grid_parameter.rect.contains(world_pos) {
            let cell_index = calculate_cell_index_from_position(&grid_parameter, grid_center, world_pos);

            if state.prev_cell != Some((cell_index.x, cell_index.y)) {
                state.prev_cell = Some((cell_index.x, cell_index.y));
                info!("Now hovering over cell ({}, {})", cell_index.x, cell_index.y);
            }
        }
    }
}

fn capture_cursor_position(
    mut cursor_position: ResMut<CursorWorldPosition>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Transform, &GlobalTransform), With<Camera2d>>) {
    // Access the main window
    let window = q_windows.single();

    if let Some(position) = q_windows.single().cursor_position() {
        // Get the camera transform.
        let (camera_transform, _global_transform) = camera_query.single();
        // Calculate the world position.
        let world_position = screen_to_world(position, window, camera_transform);
        cursor_position.position = world_position;
    }
}

// Convert function from screen space to world space
fn screen_to_world(pos: Vec2, window: &Window, camera: &Transform) -> Vec2 {
    // Get window size
    let window_size = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    // Flip Y
    let flipped_pos = Vec2::new(pos.x, window.height() - pos.y);

    // Translate the coordinate system such that the origin is at the center of the screen
    let translated_pos = flipped_pos - window_size;

    // Scale and translate the point from screen space to world space
    let world_pos = translated_pos * camera.scale.truncate() + camera.translation.truncate();

    world_pos
}


// Local state
struct HoverState {
    prev_cell: Option<(u32, u32)>,
}

impl Default for HoverState {
    fn default() -> Self {
        HoverState {
            prev_cell: None,
        }
    }
}

#[derive(Resource)]
struct CursorWorldPosition {
    position: Vec2,
}

impl Default for CursorWorldPosition {
    fn default() -> Self {
        crate::CursorWorldPosition {
            position: Vec2::ZERO
        }
    }
}