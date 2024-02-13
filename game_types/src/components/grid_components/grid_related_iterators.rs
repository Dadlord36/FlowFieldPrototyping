use bevy::math::{IVec2, URect};
use crate::components::grid_components::definitions::CellIndex2d;

pub struct AreaLineIterator {
    bounds: URect,
    current: CellIndex2d,
    direction: IVec2,
}

impl Iterator for AreaLineIterator {
    type Item = CellIndex2d;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.bounds.contains((self.current + self.direction).into())
        {
            return None;
        }

        let result = Some(self.current);
        self.current += self.direction;

        result
    }
}

pub struct AreaFullIterator {
    bounds: URect,
    current: CellIndex2d,
    direction: IVec2,
}

impl Iterator for AreaFullIterator {
    type Item = CellIndex2d;

    fn next(&mut self) -> Option<Self::Item> {
        let offset_x = IVec2::new(self.direction.x, 0);
        let offset_y = IVec2::new(0, self.direction.y);
        // let prev = self.current;

        if offset_y == IVec2::ZERO && offset_x == IVec2::ZERO {
            return None;
        }

        if offset_y != IVec2::ZERO && self.bounds.contains((self.current + offset_y).into()) {
            self.current += offset_y;
            /*   println!("prev: {prev}; offset is: {offset_y}; result is: {}", self.current)*/
        } else if offset_x != IVec2::ZERO && self.bounds.contains((self.current + offset_x).into()) {
            if self.direction.y > 0 {
                self.current.y = self.bounds.min.y;
            } else {
                self.current.y = self.bounds.max.y;
            }
            self.current += offset_x;
            /*  println!("prev: {prev}; offset is: {offset_x}; result is: {}", self.current)*/
        } else {
            return None;// Finished iterating when the offsets are out of bounds
        }

        Some(self.current)
    }
}

pub struct CoordinateIterator {
    inner: std::vec::IntoIter<CellIndex2d>,
}

impl CoordinateIterator {
    pub fn new(start_i: u32, end_i: u32, start_j: u32, end_j: u32) -> Self {
        let data: Vec<CellIndex2d> = (start_j..end_j).flat_map(move |j| (start_i..end_i).map(move |i| CellIndex2d { x: i, y: j })).collect();
        let inner = data.into_iter();
        Self { inner }
    }

    pub fn iter_area_in_line_from(start: CellIndex2d, direction: IVec2, area: URect) -> AreaLineIterator {
        AreaLineIterator {
            bounds: area,
            current: start,
            direction,
        }
    }

    pub fn iter_area_fully_from(start: CellIndex2d, direction: IVec2, area: URect) -> AreaFullIterator {
        AreaFullIterator {
            bounds: area,
            current: start,
            direction,
        }
    }
}

impl Iterator for CoordinateIterator {
    type Item = CellIndex2d;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl DoubleEndedIterator for CoordinateIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}
