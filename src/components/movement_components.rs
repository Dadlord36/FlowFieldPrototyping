use bevy::prelude::{Bundle, Component};

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

#[derive(Component, Clone, Default)]
pub struct MoveTag;

#[derive(Component, Clone, Default)]
pub struct SurfaceCoordinate {
    pub latitude: f32,
    pub longitude: f32,
}

pub struct CoordinateBounds {
    pub min: f32,
    pub max: f32,
}


