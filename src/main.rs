// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use bevy::{
    asset::AssetMetaCheck,
    DefaultPlugins,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_prototype_lyon::prelude::*;

use bevy_game;
use function_libs::flow_field::FlowField;
use function_libs::grid_calculations;
use function_libs::grid_calculations::GridParameters;

mod function_libs;

// ToDo: Replace bevy_game with your new crate name.
/*use std::io::Cursor;
use bevy::input::mouse::MouseMotion;
use bevy::pbr;
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use winit::keyboard::NamedKey::Info;*/

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
        .add_systems(Update, (moving_system, rotate_arrows_system))
        .insert_resource(grid_parameters)
        .insert_resource(flow_field)
        .insert_resource(CursorWorldPosition {
            position: Vec2::ZERO,
        })
        // .add_systems(Startup, set_window_icon)
        .run();
}

fn visualize_flow_system(mut _commands: Commands, grid_parameter: Res<GridParameters>) {
    // Create a new PathBuilder for the arrow shape
    for (col, row) in grid_parameter.coordinates() {
        let coordinate = UVec2::new(col, row);
        let cell_position = grid_calculations::calculate_cell_position(&grid_parameter, Vec2::ZERO, coordinate).extend(0.0);
        let mut new_transform = Transform::from_xyz(cell_position.x, cell_position.y, cell_position.z);
        new_transform.rotation = Quat::from_rotation_z(90.0_f32.to_radians());
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

fn rotate_arrows_system(time: Res<Time>, mut shapes_transform_query: Query<&mut Transform, With<Arrow>>) {
    let rotation_speed: f32 = 1.5; // The speed at which the arrows will rotate (in radians per second)
    for mut transform in shapes_transform_query.iter_mut() {
        let rotation_increment = Quat::from_rotation_z(-rotation_speed * time.delta_seconds());
        transform.rotation = transform.rotation * rotation_increment;
    }
}

fn spawned_colorized_cells_system(mut commands: Commands, grid_parameter: Res<GridParameters>)
{
    let columns_num = grid_parameter.column_number;
    let rows_num = grid_parameter.row_number;

    let cell_size: Vec2 = grid_parameter.cell_size;
    let cell_spacing = grid_parameter.cells_spacing;

    // calculate the total size of the grid
    let grid_size_x = columns_num as f32 * (cell_size.x + cell_spacing);
    let grid_size_y = rows_num as f32 * (cell_size.y + cell_spacing);

    let mut color1 = Color::YELLOW_GREEN;
    color1.set_a(0.2);
    let mut color2 = Color::GRAY;
    color2.set_a(0.2);

    for (i, j) in grid_parameter.coordinates() {
        let color = if (i + j) % 2 == 0 { color1 } else { color2 };

        // Adjust the cell's position so the grid is centered at (0, 0)
        let position = Vec2::new((i as f32 * (cell_size.x + cell_spacing)) - grid_size_x / 2.0 + cell_size.x / 2.0,
                                 (j as f32 * (cell_size.y + cell_spacing)) - grid_size_y / 2.0 + cell_size.y / 2.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(cell_size),
                ..Default::default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        });
    }
}

fn spawn_moving_cubes(mut commands: Commands, grid_parameter: Res<GridParameters>)
{
    let columns_num = grid_parameter.column_number;
    let rows_num = grid_parameter.row_number;

    let cell_size: Vec2 = grid_parameter.cell_size / 2.0;
    let mut cell_index = UVec2::new(columns_num - 1, 0);

    let color = Color::ORANGE;

    for y in 0..rows_num {
        cell_index.y = y;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(cell_size),
                ..Default::default()
            },
            transform: Transform::from_translation(grid_calculations::calculate_cell_position(&grid_parameter, Vec2::ZERO, cell_index).extend(0.0)),
            ..Default::default()
        }).insert(MoveTag);
    }
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
            let cell_index = grid_calculations::calculate_cell_index_from_position(&grid_parameter, grid_center, world_pos);

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

fn moving_system(time: Res<Time>, mut query: Query<(&MoveTag, &mut Transform)>) {
    let delta = 100.0 * time.delta_seconds();  // adjust accordingly for your desired move speed

    for (_, mut transform) in query.iter_mut() {
        transform.translation.x -= delta;
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

#[derive(Component)]
struct MoveTag;

#[derive(Component)]
struct Arrow;

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