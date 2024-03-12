// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use bevy::{
    asset::AssetMetaCheck,
    DefaultPlugins,
    prelude::*,
};

use game_types::{
    components::{
        flow_field_components::FlowField,
        grid_components::definitions::{
            ElapsedTimeTracker,
            Grid2D,
            GridRelatedData,
            ObstaclesParameters,
        }
        ,
        world_manipulation_components::{CursorWorldPosition, HoverCell},
    },
    systems::{
        flow_field_manipulations::*,
        grid_related::*,
    },
};
use game_types::components::directions;
use systems::{
    flow_field_related::*,
    grid_related::*,
};

mod components;
mod bundles;
mod types_declaration;
mod systems;

fn main() {
    let grid_parameters = Grid2D::new(14, 14, Vec2::new(50f32, 50f32));
    let flow_field = FlowField::form_field(grid_parameters.column_number as usize,
                                           grid_parameters.row_number as usize);
    let mut grid_related_data = GridRelatedData::new(&grid_parameters);
    grid_related_data.fill_with_random_obstacle_pattern(&grid_parameters);
    let obstacle_parameters = ObstaclesParameters { influence_area: UVec2::new(8, 8) };

    /*    let mut main_schedule = Schedule::new(Main);
        main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        let mut fixed_update_loop_schedule = Schedule::new(RunFixedUpdateLoop);
        fixed_update_loop_schedule.set_executor_kind(ExecutorKind::SingleThreaded);*/


    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Interactive Crowd".to_owned(), // ToDo
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawned_colorized_cells_system, visualize_flow_system,
                               /*reset_cells_colorization,*/ detraction_factor_calculation_system,
                               spawn_dummy_path_driven_actor, visualize_grid_in_log).chain())
        /*        .add_systems(PreUpdate, (reset_cells_colorization, capture_cursor_position, mouse_hover_system,
                                         move_camera_system, avoidance_maneuver_system, path_movement_system,
                                         adjust_coordinate_system, apply_surface_coordinate_system,
                                         grid_relation_system).chain())
                .add_systems(Update, (cell_occupation_highlight_system, colorize_obstacles_system, apply_color_to_cell
                                      , visualize_grid_data_in_log).chain())*/
        .add_systems(Update, (flow_explosion_system, rotate_flow_arrows_system).chain())
        .add_systems(Update, (reset_cells_colorization, apply_color_to_cell).chain())
        .insert_resource(grid_parameters)
        .insert_resource(grid_related_data)
        .insert_resource(obstacle_parameters)
        .insert_resource(flow_field)
        .insert_resource(ElapsedTimeTracker::default())
        .insert_resource(HoverCell::default())
        .insert_resource(CursorWorldPosition::default())
        .insert_resource(directions::Direction::West)
        // .add_systems(Startup, set_window_icon)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}