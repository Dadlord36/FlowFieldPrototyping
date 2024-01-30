use std::{fmt, iter};
use std::ops::RangeInclusive;
use bevy::{
    math::{Rect, Vec2},
    prelude::{Color, Component, Resource},
};


use derive_more::{Add, Sub, Mul, Div, Rem, Display, Into, Constructor, From};
use ndarray::Array2;
use num_traits::{AsPrimitive, Pow};


#[derive(Default, Copy, Clone, Display, Constructor, Add, Sub, Div, Rem, From, Into,
Eq, PartialEq, PartialOrd)]
pub struct CellIndex1d {
    pub value: u32,
}

impl CellIndex1d {
    pub fn sqrt(&self) -> f32 {
        (self.value as f32).sqrt()
    }
}

impl AsPrimitive<f32> for CellIndex1d {
    fn as_(self) -> f32 {
        self.value.as_()
    }
}

impl AsPrimitive<f64> for CellIndex1d {
    fn as_(self) -> f64 {
        self.value.as_()
    }
}

impl AsPrimitive<u32> for CellIndex1d {
    fn as_(self) -> u32 {
        self.value.as_()
    }
}

impl From<f32> for CellIndex1d {
    fn from(value: f32) -> Self {
        value.into()
    }
}

impl From<f64> for CellIndex1d {
    fn from(value: f64) -> Self {
        value.into()
    }
}

impl Pow<u32> for CellIndex1d {
    type Output = Self;

    fn pow(self, rhs: u32) -> Self::Output {
        CellIndex1d {
            value: self.value.pow(rhs),
        }
    }
}

impl Mul<u32> for CellIndex1d {
    type Output = CellIndex1d;

    fn mul(self, rhs: u32) -> Self::Output {
        CellIndex1d {
            value: self.value * rhs,
        }
    }
}

impl Ord for CellIndex1d {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Copy, Clone)]
pub enum Occupation {
    Free,
    Occupied,
}

impl Default for Occupation {
    fn default() -> Self {
        Occupation::Free
    }
}

#[derive(Clone, Copy, Default, From, Into, Eq, PartialEq)]
pub struct CellIndex2d {
    pub x: CellIndex1d,
    pub y: CellIndex1d,
}

impl fmt::Display for CellIndex2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x.value, self.y.value)
    }
}

impl CellIndex2d {
    pub fn new<T: AsPrimitive<u32>>(x: T, y: T) -> Self {
        CellIndex2d {
            x: x.as_().into(),
            y: y.as_().into(),
        }
    }

    pub fn normalize(&self) -> Self {
        let length = (self.x.pow(2) + self.y.pow(2)).sqrt();
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();

        Self {
            x: CellIndex1d::from(x / length),
            y: CellIndex1d::from(y / length),
        }
    }

    pub fn euclidean_distance(&self, other: &CellIndex2d) -> f32 {
        let x_distance = other.x.value as f64 - self.x.value as f64;
        let y_distance = other.y.value as f64 - self.y.value as f64;

        (x_distance.powi(2) + y_distance.powi(2)).sqrt() as f32
    }
}

#[derive(Resource)]
pub struct Grid2D {
    pub column_number: u32,
    pub row_number: u32,
    pub cell_size: Vec2,
    pub grid_size: Vec2,
    pub cells_spacing: f32,
    pub rect: Rect,
    pub max_row_index: u32,
    pub max_column_index: u32,
}

#[derive(Clone, Default)]
pub struct GridCellData {
    pub color: Color,
    pub occupation_state: Occupation,
}

#[derive(Resource)]
pub struct GridRelatedData {
    pub data: Array2<GridCellData>,
}

#[derive(Component, Clone, Copy, Default, Constructor, From, Into)]
pub struct CellIndex {
    pub index: CellIndex2d,
}

#[derive(Component)]
pub struct GridCellTag;