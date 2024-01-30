use std::{
    fmt::{self, Debug, Formatter},
    ops::{Add, Sub},
};
use std::ops::Mul;

use bevy::prelude::{UVec2, Vec2};
use ndarray::{
    NdIndex,
    prelude::*
};
use num_traits::AsPrimitive;

use crate::components::grid_components::{CellIndex1d, CellIndex2d};

impl From<UVec2> for CellIndex2d {
    fn from(vec: UVec2) -> Self {
        CellIndex2d::new(vec.x, vec.y)
    }
}

impl From<CellIndex2d> for Vec2 {
    fn from(index: CellIndex2d) -> Self {
        Vec2 { x: index.x.into(), y: index.y.into() }
    }
}

impl From<CellIndex2d> for UVec2{
    fn from(value: CellIndex2d) -> Self {
        UVec2 { x: value.x.into(), y: value.y.into() }
    }
}

impl Mul<Vec2> for CellIndex2d {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(
            f32::from(self.x) * rhs.x,
            f32::from(self.y) * rhs.y,
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
        let x:f64 = self.x.as_();
        let y:f64 = self.y.as_();
        Self {
            x: (x * rhs).into(),
            y: (y * rhs).into(),
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

impl Debug for CellIndex1d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "CellIndex1d : {self}")
    }
}

unsafe impl NdIndex<Ix2> for CellIndex2d {
    fn index_checked(&self, dim: &Ix2, strides: &Ix2) -> Option<isize> {
        let ix: [usize; 2] = [CellIndex1d::into(self.x), usize::from(self.y)];
        ix.index_checked(dim, strides)
    }

    fn index_unchecked(&self, strides: &Ix2) -> isize {
        let ix: [usize; 2] = [CellIndex1d::into(self.x), usize::from(self.y)];
        ix.index_unchecked(strides)
    }
}



impl From<CellIndex1d> for f32 {
    fn from(index: CellIndex1d) -> Self {
        index.value.as_()
    }
}

impl From<CellIndex1d> for usize {
    fn from(index: CellIndex1d) -> Self {
        index.value.as_()
    }
}