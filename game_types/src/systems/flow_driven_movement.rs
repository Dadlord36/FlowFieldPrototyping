use bevy::{
    prelude::*
};
use bevy::math::DVec2;
use crate::{
    components::{
        grid_components::{
            Grid2D,
            CellIndex,
            CellIndex2d,
            GridRelatedData
        },
        movement_components::{
            Maneuver,
            MoveTag,
            SurfaceCoordinate
        },
        flow_field_components::FlowField,
    }
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
        let direction: DVec2 = DVec2::from(flow_field.get_field_at(cell_index.index));

        surface_calculations.adjust_coordinate(direction * delta);
    }
}

pub fn apply_surface_coordinate_system(grid_parameters: Res<Grid2D>,
                                       mut query: Query<(&mut Transform, &SurfaceCoordinate),
                                           With<MoveTag>>) {
    for (mut transform, coordinate) in query.iter_mut() {
        *transform = coordinate.project_surface_coordinate_on_grid(&grid_parameters);
    }
}

pub fn path_movement_system(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut SurfaceCoordinate, &mut Maneuver), With<MoveTag>>) {
    for (entity, mut coordinate, mut maneuver) in query.iter_mut() {
        *coordinate = maneuver.catmull_rom_interpolate_along_path_ping_pong(time.elapsed_seconds() * 0.5);
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

pub fn cell_occupation_highlight_system(mut grid_cell_data: ResMut<GridRelatedData>,
                                        query: Query<&CellIndex, With<MoveTag>>)
{
    for cell_index in query.iter() {
        let cell_data = grid_cell_data.get_data_at_mut(cell_index.index);
        if cell_data.is_none() {
            continue;
        }
        cell_data.unwrap().color = Color::BLACK;
    }
}