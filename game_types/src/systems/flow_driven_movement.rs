use crate::components::pathfinding_components::Pathfinder;
use bevy::{
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
use bevy::log::info;

use crate::components::{
    flow_field_components::FlowField,
    grid_components::definitions::{CellIndex, CellIndex2d, Grid2D, GridRelatedData},
    movement_components::{
        Direction,
        Maneuver,
        MoveTag,
        PerformManeuver,
        SurfaceCoordinate,
    },
};

pub fn calculate_coordination_data(grid_parameters: &Res<Grid2D>, cell_index: CellIndex2d) -> (SurfaceCoordinate, Transform, Vec2) {
    let coordinate = grid_parameters.calculate_flat_surface_coordinate_from(cell_index);
    let mut coordinate_world_transform = coordinate.project_surface_coordinate_on_grid(&grid_parameters);
    coordinate_world_transform.translation.z = 10.0;
    let actor_size: Vec2 = grid_parameters.cell_size / 2.0;
    (coordinate, coordinate_world_transform, actor_size)
}

pub fn adjust_coordinate_system(time: Res<Time>, flow_field: Res<FlowField>,
                                mut query: Query<(&mut SurfaceCoordinate, &CellIndex), With<MoveTag>>)
{
    let delta: f64 = (0.1 * time.delta_seconds()) as f64;

    for (mut surface_calculations, cell_index) in query.iter_mut() {
        let direction: DVec2 = DVec2::from(flow_field.get_field_at(cell_index.as_ref()));

        surface_calculations.adjust_coordinate(direction * delta);
    }
}

pub fn apply_surface_coordinate_system(grid_parameters: Res<Grid2D>,
                                       mut query: Query<(&mut Transform, &SurfaceCoordinate), With<MoveTag>>) {
    for (mut transform, coordinate) in query.iter_mut() {
        *transform = coordinate.project_surface_coordinate_on_grid(&grid_parameters);
    }
}

pub fn avoidance_maneuver_system(grid_parameters: Res<Grid2D>, grid_related_data: Res<GridRelatedData>,
                                 main_move_direction: Res<Direction>,
                                 query: Query<(&CellIndex, &mut Maneuver), With<MoveTag>>) {
    for (cell_index, _maneuver) in query.iter() {
        let straight_path_area = grid_parameters.calculate_line_from(cell_index.index, main_move_direction.as_vector(), 5);
        if grid_related_data.has_obstacle_in(straight_path_area) {
            let area = grid_parameters.calculate_area_from(cell_index.index, main_move_direction.as_vector(), 4);
            let pathfinding_map = grid_related_data.create_pathfinding_map(area);
            pathfinding_map.find_destination_in_direction(cell_index.index, *main_move_direction);
            // pathfinding_map.calculate_path(Pathfinder::new());
            // info!("has obstacle in {}",main_move_direction.as_ref())
        }
    }
}

pub fn path_movement_system(_commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut SurfaceCoordinate,
                                                                                    &mut Maneuver),
    (With<MoveTag>, With<PerformManeuver>)>) {
    for (_entity, mut coordinate, mut maneuver) in query.iter_mut() {
        *coordinate = maneuver.catmull_rom_interpolate_along_path_ping_pong(time.elapsed_seconds() * 0.005);
        /*       if maneuver.is_done() {
                   commands.entity(entity).remove::<Maneuver>();
               }*/
    }
}

/*pub fn movement_avoidance_system(mut query: Query<()>) {

}*/

pub fn grid_relation_system(grid_parameters: Res<Grid2D>,
                            mut query: Query<(&mut CellIndex, &SurfaceCoordinate), With<MoveTag>>)
{
    for (mut cell_index, surface_calculations) in query.iter_mut() {
        cell_index.index = surface_calculations.calculate_cell_index_on_flat_surface(&grid_parameters);
    }
}

pub fn cell_occupation_highlight_system(mut grid_cell_data: ResMut<GridRelatedData>, grid_parameters: Res<Grid2D>,
                                        main_movement_direction: Res<Direction>, query: Query<&CellIndex, With<MoveTag>>)
{
    for cell_index in query.iter() {
        let segment_area = grid_parameters.calculate_line_from(cell_index.index.into(), main_movement_direction.as_vector(), 5);
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