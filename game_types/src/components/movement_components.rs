use bevy::prelude::{Component, IVec2, Resource, Vec2};
use bracket_pathfinding::prelude::NavigationPath;
use derive_more::Display;
use crate::components::pathfinding_components::Pathfinder;

pub type Coordinate = f32;

#[derive(Copy, Clone, Component, Resource, Display)]
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

impl IntoIterator for Direction {
    type Item = Direction;
    type IntoIter = DirectionIntoIter;

    fn into_iter(self) -> DirectionIntoIter {
        DirectionIntoIter(self)
    }
}

pub struct DirectionIntoIter(Direction);

impl Iterator for DirectionIntoIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Direction::North => {
                self.0 = Direction::NorthEast;
                Some(Direction::North)
            }
            Direction::NorthEast => {
                self.0 = Direction::East;
                Some(Direction::NorthEast)
            }
            Direction::East => {
                self.0 = Direction::SouthEast;
                Some(Direction::East)
            }
            Direction::SouthEast => {
                self.0 = Direction::South;
                Some(Direction::SouthEast)
            }
            Direction::South => {
                self.0 = Direction::SouthWest;
                Some(Direction::South)
            }
            Direction::SouthWest => {
                self.0 = Direction::West;
                Some(Direction::SouthWest)
            }
            Direction::West => {
                self.0 = Direction::NorthWest;
                Some(Direction::West)
            }
            Direction::NorthWest => {
                self.0 = Direction::North;
                Some(Direction::NorthWest)
            }
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

pub const DIRECTIONS: [IVec2; 8] =
    [
        IVec2::new(0, 1),   // North
        IVec2::new(1, 1),   // North-East
        IVec2::new(1, 0),   // East
        IVec2::new(1, -1),  // South-East
        IVec2::new(0, -1),  // South
        IVec2::new(-1, -1), // South-West
        IVec2::new(-1, 0),  // West
        IVec2::new(-1, 1),  // North-West
    ];

impl Direction {
    pub fn as_vector(&self) -> IVec2 {
        match self {
            Self::North => DIRECTIONS[0],
            Self::NorthEast => DIRECTIONS[1],
            Self::East => DIRECTIONS[2],
            Self::SouthEast => DIRECTIONS[3],
            Self::South => DIRECTIONS[4],
            Self::SouthWest => DIRECTIONS[5],
            Self::West => DIRECTIONS[6],
            Self::NorthWest => DIRECTIONS[7],
        }
    }
}

impl Into<IVec2> for Direction {
    fn into(self) -> IVec2 {
        self.as_vector()
    }
}

#[derive(Component, Clone, Default)]
pub struct MoveTag;

#[derive(Component, Clone, Default)]
pub struct PerformManeuver;

impl Direction {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}


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