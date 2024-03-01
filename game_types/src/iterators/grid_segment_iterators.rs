#![feature(iterator_fold_self)] // This needs to be at the top of your file, because we're using 'try_fold' method

use std::iter::FusedIterator;

use bevy::prelude::URect;
use derive_more::{Constructor, Debug};
use crate::components::grid_components::definitions::CellIndex2d;

#[derive(Constructor)]
pub struct CellInDistance {
    cell_index: CellIndex2d,
    distance_from_center: u32,
}

// RelativeDirection represents the direction of the
// next step in the spiral.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RelativeDirection {
    Right,
    Down,
    Left,
    Up,
}

// The `SpiralIter` struct represents a spiral iterator.
#[derive(Debug, PartialEq)]
pub struct SpiralIter {
    // What point we're at. This should start at (0, 0) and will increase as the iterator progresses.
    // This tuple represents an `x` and `y` coordinate
    cursor: (isize, isize),
    parent_grid: URect,
    bounds: URect,
    // What direction to move next
    direction: RelativeDirection,
    // How many steps to take in the current direction
    steps_in_current_direction: isize,
    // How many steps we've taken so far in the current direction
    steps_taken_in_current_direction: isize,
    // How many steps do we take in total before we change direction to make a square spiral
    steps_before_changing_direction: isize,
}

impl SpiralIter {
    // New makes a new spiral iterator with `size` size
    // This means that the spiral will be `size` x `size`
    fn new(parent_grid: URect, bounds: URect) -> Self {
        SpiralIter {
            cursor: (bounds.width() as isize / 2, bounds.height() as isize / 2),
            parent_grid,
            bounds,
            direction: RelativeDirection::Right,
            steps_in_current_direction: 1,
            steps_taken_in_current_direction: 0,
            steps_before_changing_direction: 2,
        }
    }
}


// We use Euclidean distance formula here
fn distance(x1: isize, y1: isize, x2: isize, y2: isize) -> u32 {
    let x_diff = (x1 - x2).abs();
    let y_diff = (y1 - y2).abs();
    (x_diff + y_diff) as u32
}

// Implement the iterator
impl Iterator for SpiralIter {
    // This iterator yields (usize, usize) tuples
    type Item = Option<(CellInDistance)>;

    // The next method is what makes this struct an iterator
    fn next(&mut self) -> Option<Self::Item> {
        // Start by getting the next coordinate
        let (next_x, next_y) = match self.direction {
            RelativeDirection::Right => (self.cursor.0 + 1, self.cursor.1),
            RelativeDirection::Down => (self.cursor.0, self.cursor.1 + 1),
            RelativeDirection::Left => (self.cursor.0 - 1, self.cursor.1),
            RelativeDirection::Up => (self.cursor.0, self.cursor.1 - 1),
        };

        // Check whether we have reached the end of the spiral
        if next_x >= self.bounds.width() as isize || next_y >= self.bounds.height() as isize {
            return None;
        }

        // Check whether the next coordinate is valid
        if (0..self.bounds.width() as isize).contains(&next_x) &&
            (0..self.bounds.height() as isize).contains(&next_y) {

            // Update cursor to the next coordinate
            self.cursor = (next_x, next_y);

            // Increment steps_taken_in_current_direction
            self.steps_taken_in_current_direction += 1;

            // If we're at the end of current direction, change direction
            if self.steps_taken_in_current_direction == self.steps_in_current_direction {
                self.steps_taken_in_current_direction = 0;
                self.direction = match self.direction {
                    RelativeDirection::Right => {
                        self.steps_in_current_direction = self.steps_before_changing_direction;
                        RelativeDirection::Down
                    }
                    RelativeDirection::Down => {
                        self.steps_before_changing_direction += 1;
                        self.steps_in_current_direction = self.steps_before_changing_direction;
                        RelativeDirection::Left
                    }
                    RelativeDirection::Left => {
                        self.steps_in_current_direction = self.steps_before_changing_direction;
                        RelativeDirection::Up
                    }
                    RelativeDirection::Up => {
                        self.steps_before_changing_direction += 1;
                        self.steps_in_current_direction = self.steps_before_changing_direction;
                        RelativeDirection::Right
                    }
                }
            }
            // If we're at the end of a full iteration (square), increase steps in curr direction
            if self.steps_before_changing_direction == 2 {
                self.steps_in_current_direction += 1;
                self.steps_before_changing_direction = 0;
            }

            // Calculate and return the distance
            let center_x = self.bounds.width() as isize / 2;
            let center_y = self.bounds.height() as isize / 2;
            let dist = distance(next_x, next_y, center_x, center_y);

            let cell_index = CellIndex2d::new(next_x as usize, next_y as usize);
            // Push coordinates and distance to the result Vector
            let result = CellInDistance::new(cell_index, dist);
            return Some(Option::from(result));
        } else {
            // If the next coordinate is not valid, then continue with the iteration
            self.next()
        }
    }
}

#[test]
fn test_spiral_iter() {
    let parent_grid = URect::new(0, 0, 25, 25);
    let bounds = URect::new(10, 10, 15, 15);

    let spiral_iter = SpiralIter::new(parent_grid, bounds);


    for iteration in spiral_iter {
        let cell_in_distance = iteration.unwrap();
        println!("Index:{}, in distance: {}", cell_in_distance.cell_index,
                 cell_in_distance.distance_from_center);
    }
}

impl FusedIterator for SpiralIter {}