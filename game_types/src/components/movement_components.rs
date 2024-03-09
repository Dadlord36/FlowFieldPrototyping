use bevy::prelude::{Component, Vec2};
use crate::components::pathfinding_components::Pathfinder;

pub type Coordinate = f32;

#[derive(Component, Clone, Default)]
pub struct MoveTag;

#[derive(Component, Clone, Default)]
pub struct ObstacleTag;

#[derive(Component, Clone, Default)]
pub struct PerformManeuver;


#[derive(Component, Clone, Copy, Default)]
pub struct SurfaceCoordinate {
    pub latitude: Coordinate,
    pub longitude: Coordinate,
}

impl From<SurfaceCoordinate> for Vec2 {
    fn from(value: SurfaceCoordinate) -> Self {
        Vec2::new(value.latitude, value.longitude)
    }
}

pub struct CoordinateBounds {
    pub min: Coordinate,
    pub max: Coordinate,
}

#[derive(Component, Default)]
pub struct Maneuver {
    pub path_points: Vec<SurfaceCoordinate>,
    pub progress: f32,
    pub last_destination: Pathfinder,
}