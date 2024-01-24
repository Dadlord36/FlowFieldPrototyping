use bevy::prelude::{Bundle, Component, Transform, Vec2};

pub type Coordinate = f32;

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Component, Clone, Default)]
pub struct MoveTag;

impl Direction {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, 0),  // North
    (-1, 1),  // North-East
    (0, 1),   // East
    (1, 1),   // South-East
    (1, 0),   // South
    (1, -1),  // South-West
    (0, -1),  // West
    (-1, -1), // North-West
];

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
    pub(crate) path_points: Vec<SurfaceCoordinate>,
    pub progress: f32,
}