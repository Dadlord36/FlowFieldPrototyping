use std::borrow::Borrow;

use bevy::{
    log::info,
    math::DVec2,
    prelude::{
        Color,
        Commands,
        Entity,
        Query,
        Res,
        ResMut,
        Time,
        Transform,
        Vec2,
        With,
    },
};
use bevy::prelude::Without;

use crate::{
    components::{
        flow_field_components::FlowField,
        grid_components::definitions::{
            CellIndex,
            CellIndex2d,
            Grid2D,
            GridRelatedData,
            Occupation,
        },
        movement_components::{
            Maneuver,
            MoveTag,
            PerformManeuver,
            SurfaceCoordinate,
        },
        pathfinding_components::{
            MovementSpeed,
            PathfindingMap,
        },
    },
    systems::{CELLS_IN_FRONT, PATHFINDING_RECT},
};
use crate::components::directions::Direction;
use crate::components::pathfinding_components::Pathfinder;

pub fn calculate_coordination_data(grid_parameters: &Res<Grid2D>, cell_index: CellIndex2d) -> (SurfaceCoordinate, Transform, Vec2) {
    let coordinate = grid_parameters.calculate_flat_surface_coordinate_from_2d(cell_index);
    let mut coordinate_world_transform = coordinate.project_surface_coordinate_on_grid(&grid_parameters);
    coordinate_world_transform.translation.z = 10.0;
    let actor_size: Vec2 = grid_parameters.cell_size / 2.0;
    (coordinate, coordinate_world_transform, actor_size)
}

pub fn adjust_coordinate_system(time: Res<Time>, flow_field: Res<FlowField>,
                                mut query: Query<(&mut SurfaceCoordinate, &CellIndex, &MovementSpeed),
                                    With<MoveTag>>)
{
    for (mut surface_calculations, cell_index, speed)
    in query.iter_mut() {
        let direction: DVec2 = DVec2::from(flow_field.get_field_at(cell_index.as_ref()));

        let speed_mul = (speed.value * time.delta_seconds()) as f64;
        surface_calculations.adjust_coordinate(direction * speed_mul);
    }
}

pub fn apply_surface_coordinate_system(grid_parameters: Res<Grid2D>,
                                       mut query: Query<(&mut Transform,
                                                         &SurfaceCoordinate), With<MoveTag>>) {
    for (mut transform, coordinate) in query.iter_mut() {
        *transform = coordinate.project_surface_coordinate_on_grid(&grid_parameters);
    }
}

pub fn avoidance_maneuver_system(mut _commands: Commands, grid: Res<Grid2D>,
                                 mut grid_related_data: ResMut<GridRelatedData>,
                                 main_move_direction: Res<Direction>,
                                 mut query: Query<(Entity, &CellIndex, &mut Maneuver),
                                     (With<MoveTag>, Without<PerformManeuver>)>) {
    for (entity, cell_index, mut _maneuver) in query.iter_mut() {
        let straight_path_area = grid.calculate_line_infront_from(cell_index.index,
                                                                  main_move_direction.as_vector(),
                                                                  CELLS_IN_FRONT);

        if grid_related_data.has_obstacle_in(straight_path_area) {
            let area = grid.calculate_square_area_wrapped_from(cell_index.index, PATHFINDING_RECT);
            // grid_related_data.set_color_for_area(area, Color::GRAY);

            let pathfinding_map: PathfindingMap = grid_related_data.create_pathfinding_map_on(&grid, area);
            let path_description_local: Option<Pathfinder> =
                pathfinding_map.find_destination_in_direction(cell_index.index, *main_move_direction);

            if path_description_local.is_none() {
                // info!("No valid destination found");
                continue;
            }
            let path_description_local = path_description_local.unwrap();
            if path_description_local == _maneuver.last_destination {
                continue;
            }
            _maneuver.last_destination = path_description_local.clone();
            let path_description_global = pathfinding_map.convert_to_global(path_description_local);

            let nav_path: Option<Vec<CellIndex2d>> =
                pathfinding_map.calculate_path_coordinates_global(path_description_local);

            if nav_path.is_none() {
                info!("No valid path found");
                pathfinding_map.visualize_key_points_on_grid(&grid, &path_description_global, &grid_related_data);
                continue;
            }
            let path_points_global = nav_path.unwrap();
            /* pathfinding_map.visualize_path_on_grid(&grid, &path_description_global,
                                                    &grid_related_data, &path_points_global);*/

            let global_path_points =
                grid.calculate_surface_coordinates_for_2d(&path_points_global);

            grid_related_data.set_color_for_index(&path_description_local.start, Color::RED);
            grid_related_data.set_color_for_index(&path_description_local.end, Color::MIDNIGHT_BLUE);

            _maneuver.set_coordinates(global_path_points);
            _commands.entity(entity).insert(PerformManeuver::default());
        }
    }
}

pub fn path_movement_system(mut _commands: Commands,
                            time: Res<Time>,
                            mut query: Query<(Entity, &mut SurfaceCoordinate, &mut Maneuver),
                                (With<MoveTag>, With<PerformManeuver>)>) {
    for (_entity, mut coordinate, mut maneuver) in query.iter_mut() {
        *coordinate = maneuver.catmull_rom_interpolate_along_path(0.005);
        if maneuver.is_done() {
            _commands.entity(_entity).remove::<PerformManeuver>();
        }
    }
}

pub fn grid_relation_system(grid_parameters: Res<Grid2D>,
                            mut query: Query<(&mut CellIndex, &SurfaceCoordinate), With<MoveTag>>)
{
    for (mut cell_index, surface_calculations) in query.iter_mut() {
        cell_index.index = surface_calculations.calculate_cell_index_on_flat_surface(&grid_parameters);
    }
}

pub fn cell_occupation_highlight_system(mut grid_cell_data: ResMut<GridRelatedData>,
                                        grid_parameters: Res<Grid2D>,
                                        main_movement_direction: Res<Direction>,
                                        query: Query<&CellIndex, With<MoveTag>>)
{
    for cell_index in query.iter() {
        let segment_area = grid_parameters.calculate_line_from(cell_index.index.into(),
                                                               main_movement_direction.as_vector(),
                                                               CELLS_IN_FRONT);
        let segment_view = grid_parameters.get_indexes_segment(segment_area);

        for segment_cell_index in segment_view {
            let cell_data = grid_cell_data.get_data_at_mut(segment_cell_index);
            cell_data.color = Color::LIME_GREEN;
        }

        {
            let cell_data = &mut grid_cell_data.get_data_at_mut(&cell_index.index);
            cell_data.color = Color::BLUE;
        }
    }
}