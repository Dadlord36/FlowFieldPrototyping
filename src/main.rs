// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use bevy::{
    asset::AssetMetaCheck,
    DefaultPlugins,
    window::PrimaryWindow,
    input::mouse::MouseMotion,
    prelude::*,
};
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{
    components::{
        flow_field_components::FlowField,
        grid_components::{GridParameters, GridRelatedData},
    },
    function_libs::grid_calculations::{self, calculate_cell_index_from_position},
    systems::{
        flow_driven_movement::*,
        flow_field_manipulations::*,
        grid_related::*,
    },
};

mod function_libs;
mod systems;
mod tests;
mod components;
mod bundles;

#[derive(Resource, Default)]
struct HoverCell {
    hovered_cell: UVec2,
}

#[derive(Resource)]
struct CursorWorldPosition {
    position: Vec2,
}

#[derive(Component)]
struct SelectedCell;

impl Default for CursorWorldPosition {
    fn default() -> Self {
        crate::CursorWorldPosition {
            position: Vec2::ZERO
        }
    }
}

fn main() {
    let grid_parameters = GridParameters::new(25, 25, Vec2::new(50f32, 50f32));
    let flow_field = FlowField::form_field(grid_parameters.column_number, grid_parameters.row_number);
    let grid_related_data = GridRelatedData::new(&grid_parameters);

    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Interactive Crowd".to_string(), // ToDo
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
        // .add_plugins(bevy_game::GamePlugin)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (setup, spawned_colorized_cells_system, visualize_flow_system, spawn_moving_cubes).chain())
        .add_systems(Update, (capture_cursor_position, reset_cells_colorization, mouse_hover_system, cell_occupation_highlight_system, apply_color_to_cell).chain())
        .add_systems(Update, (adjust_coordinate_system, apply_surface_coordinate_system, grid_relation_system).chain())
        .add_systems(Update, (flow_explosion_system, rotate_flow_arrows_system))
        .insert_resource(grid_parameters)
        .insert_resource(grid_related_data)
        .insert_resource(flow_field)
        .insert_resource(HoverCell::default())
        .insert_resource(CursorWorldPosition {
            position: Vec2::ZERO,
        })
        // .add_systems(Startup, set_window_icon)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn mouse_hover_system(mut cursor_moved_events: EventReader<CursorMoved>, cursor_world_position: Res<CursorWorldPosition>,
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
        let hovered_cell_index = calculate_cell_index_from_position(&grid_parameters, world_pos);

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

fn capture_cursor_position(mut mouse_motion_events: EventReader<MouseMotion>,
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
            let hovered_cell_index = calculate_cell_index_from_position(&grid_parameters, world_position);
            hover_cell.hovered_cell = hovered_cell_index;
        }
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


