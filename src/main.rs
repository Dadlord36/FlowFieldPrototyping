// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use bevy::{
    asset::AssetMetaCheck,
    DefaultPlugins    ,
    prelude::*,
};
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::{
    components::{
        flow_field_components::FlowField,
        grid_components::{GridParameters, GridRelatedData},
        world_manipulation_components::{CursorWorldPosition, HoverCell}
    },
    systems::{
        flow_driven_movement::*,
        flow_field_manipulations::*,
        grid_related::*,
        selection_related::{capture_cursor_position, mouse_hover_system}
    },
};

mod function_libs;
mod systems;
mod components;
mod bundles;



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