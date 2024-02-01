use std::{
    fmt::{self, Debug, Formatter},
    ops::{
        Add,
        Sub,
        Mul
    }
};

use bevy::prelude::{UVec2, Vec2};
use ndarray::{Ix2, NdIndex};
use num_traits::AsPrimitive;

use crate::components::grid_components::CellIndex2d;

impl From<UVec2> for CellIndex2d {
    fn from(vec: UVec2) -> Self {
        CellIndex2d::new(vec.x, vec.y)
    }
}

impl From<CellIndex2d> for Vec2 {
    fn from(index: CellIndex2d) -> Self {
        Vec2 { x: index.x as f32, y: index.y as f32 }
    }
}

impl From<CellIndex2d> for UVec2 {
    fn from(value: CellIndex2d) -> Self {
        UVec2 { x: value.x.into(), y: value.y.into() }
    }
}

impl Mul<Vec2> for CellIndex2d {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(
            (self.x as f32) * rhs.x,
            (self.y as f32) * rhs.y,
        )
    }
}

// Implement addition for GridCellIndex
impl Add for CellIndex2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implement subtraction for GridCellIndex
impl Sub for CellIndex2d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<u32> for CellIndex2d {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f64> for CellIndex2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64 = self.x.as_();
        let y: f64 = self.y.as_();
        Self {
            x: (x * rhs) as u32,
            y: (y * rhs) as u32,
        }
    }
}

impl Mul<i32> for CellIndex2d {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as u32,
            y: self.y * rhs as u32,
        }
    }
}

impl Debug for CellIndex2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GridCellIndex {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl fmt::Display for CellIndex2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl CellIndex2d {
    pub fn new<T: AsPrimitive<u32>>(x: T, y: T) -> Self {
        CellIndex2d {
            x: x.as_(),
            y: y.as_(),
        }
    }

    pub fn normalize(&self) -> Self {
        let length = ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt() as u32;
        Self {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn euclidean_distance(&self, other: &CellIndex2d) -> f32 {
        let x_distance = other.x as f64 - self.x as f64;
        let y_distance = other.y as f64 - self.y as f64;

        (x_distance.powi(2) + y_distance.powi(2)).sqrt() as f32
    }
}

unsafe impl NdIndex<Ix2> for CellIndex2d {
    fn index_checked(&self, dim: &Ix2, strides: &Ix2) -> Option<isize> {
        if (self.x as usize) < dim[0] && (self.y as usize) < dim[1] {
            Some(self.index_unchecked(strides))
        } else {
            None
        }
    }

    fn index_unchecked(&self, strides: &Ix2) -> isize {
        (self.x as isize * strides[0] as isize) + (self.y as isize * strides[1] as isize)
    }
}