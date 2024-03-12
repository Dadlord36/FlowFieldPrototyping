use bevy::{
    math::IVec2,
    prelude::{Component, Resource},
};

use derive_more::Display;
use serde::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize, Component, Resource, Display)]
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

impl Direction {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl IntoIterator for Direction {
    type Item = Direction;
    type IntoIter = DirectionIntoIter;

    fn into_iter(self) -> DirectionIntoIter {
        DirectionIntoIter(self)
    }
}