use std::cmp::min;
use std::collections::HashMap;

use bevy::{
    math::{IVec2, Rect, URect, UVec2, Vec2},
};
use colored::{Color, ColoredString, Colorize};
use ndarray::{Array2, ArrayView2};

use crate::{
    components::{
        directions::Direction,
        grid_components::{
            definitions::{CellIndex1d, CellIndex2d, Grid2D, GridSegment},
            grid_related_iterators::CoordinateIterator,
        },
        movement_components::{
            Coordinate,
            SurfaceCoordinate,
        },
    },
    function_libs::grid_calculations::{
        self
    },
};

impl Grid2D {
    pub fn iter_coordinates(&self) -> CoordinateIterator {
        CoordinateIterator::new(0, self.max_column_index, 0, self.max_row_index)
    }

    pub fn iter_coordinates_range(&self, min: UVec2, max: UVec2) -> CoordinateIterator {
        assert!(max.x <= self.column_number);
        assert!(max.y <= self.row_number);
        CoordinateIterator::new(min.x, max.x, min.y, max.y)
    }

    pub fn iter_coordinates_in_area(&self, area: URect) -> CoordinateIterator {
        assert!(area.max.x <= self.column_number - 1, "max.x is out of bounds");
        assert!(area.max.y <= self.row_number - 1, "max.y is out of bounds");
        CoordinateIterator::new(area.min.x, area.max.x, area.min.y, area.max.y)
    }

    pub fn new(column_number: u32, row_number: u32, cell_size: Vec2) -> Self {
        let grid_size = Vec2::new(column_number as f32 * cell_size.x,
                                  row_number as f32 * cell_size.y);

        let max_column_index = column_number - 1;
        let max_row_index = row_number - 1;

        let mut grid = Grid2D {
            column_number,
            row_number,
            cell_size,
            grid_size,
            cells_spacing: 0.0,
            shape_rect: Rect::from_center_size(Vec2::ZERO, grid_size),
            max_column_index,
            max_row_index,
            indexes: Default::default(),
            indexes_rect: URect::from_corners(UVec2::ZERO, UVec2::new(max_column_index, max_row_index)),
            segments: Default::default(),
        };
        grid.segments = grid_calculations::split_grid_in_compass_directions(&grid.indexes_rect);
        grid.indexes = grid.get_indexes();
        return grid;
    }

    pub fn calculate_surface_coordinates_for_1d(&self, cell_indexes: &Vec<usize>) -> Vec<SurfaceCoordinate> {
        cell_indexes
            .iter()
            .map(|&index| {
                self.calculate_flat_surface_coordinate_from_1d(index as CellIndex1d)
            }).collect()
    }

    pub fn get_central_cell(&self) -> CellIndex2d {
        let center_x = self.column_number / 2;
        let center_y = self.row_number / 2;
        CellIndex2d::new(center_x, center_y)
    }

    pub fn calculate_surface_coordinates_for_2d(&self, cell_indexes: &Vec<CellIndex2d>) -> Vec<SurfaceCoordinate> {
        cell_indexes
            .iter()
            .map(|&index| {
                self.calculate_flat_surface_coordinate_from_2d(index)
            }).collect()
    }

    pub fn form_segment_for(&self, area: URect) -> GridSegment {
        GridSegment::new(self.indexes_rect, area)
    }

    pub fn form_segment_from(&self, origin: CellIndex2d, in_direction: IVec2, num_cells: u32) -> GridSegment {
        let segment_grid = URect {
            min: origin.into(),
            max: UVec2::from(origin + in_direction * (num_cells as i32 - 1)),
        };
        let parent_grid = self.indexes_rect;
        GridSegment::new(parent_grid, segment_grid)
    }

    // Calculate an index that is in given direction by given cells number distance
    pub fn calculate_cell_index_in_direction_from(&self, origin: CellIndex2d, in_direction: IVec2,
                                                  num_cells: u32) -> CellIndex2d
    {
        let new_index = origin + (in_direction * num_cells as i32);
        new_index
    }

    /// Calculates a clamped rectangle area from a given center point and size.
    ///
    /// # Arguments
    ///
    /// * `center` - The center point of the area.
    /// * `size` - The size of the area.
    ///
    /// # Returns
    ///
    /// The clamped rectangle area defined by the center point and size.
    ///
    pub fn calculate_area_clamped_from_center(&self, center: &CellIndex2d, size: UVec2) -> URect {
        // Calculate half sizes
        let half_width = size.x / 2;
        let half_height = size.y / 2;

        // Calculate top left cell positions ensuring it doesn't go beyond grid boundaries
        let min_x = center.x.saturating_sub(half_width);
        let min_y = center.y.saturating_sub(half_height);

        // Calculate bottom right cell positions ensuring it doesn't go beyond grid boundaries
        let max_x = (center.x + half_width).min(self.max_column_index);
        let max_y = (center.y + half_height).min(self.max_row_index);

        // Creating the URect
        let rect = URect::from_corners(UVec2::new(min_x, min_y), UVec2::new(max_x, max_y));

        rect
    }

    /// Calculates the area based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `point` - The starting point for the area calculation.
    /// * `in_direction` - The direction in which the area should be calculated.
    /// * `num_cells` - The number of cells in the specified direction that should be included in the area.
    ///
    /// # Returns
    ///
    /// The calculated area as a `URect` (unsigned rectangle) object.
    pub fn calculate_area_from(&self, point: CellIndex2d, in_direction: IVec2, num_cells: u32) -> URect {
        let adjusted_direction = IVec2::new(in_direction.x * num_cells as i32,
                                            in_direction.y * num_cells as i32);

        let start_x = ((point.x as i32) - adjusted_direction.x.abs()).max(0) as u32;
        let start_y = ((point.y as i32) - adjusted_direction.y.abs()).max(0) as u32;
        let start = UVec2::new(start_x, start_y);

        let end_x = (start_x + adjusted_direction.x.abs() as u32 * 2).min(self.max_column_index);
        let end_y = (start_y + adjusted_direction.y.abs() as u32 * 2).min(self.max_row_index);
        let end = UVec2::new(end_x, end_y);

        let rect = URect::from_corners(start, end);

        rect
    }

    #[inline]
    pub fn calculate_square_area_wrapped_from(&self, from: CellIndex2d, area_size: UVec2) -> URect {
        let rect = URect::from_center_size(from.into(), area_size);
        self.clamp_rect_to_grid_bounds(rect)
    }

    /// Calculate a line from a given point in a specified direction for a certain number of cells.
    ///
    /// # Arguments
    ///
    /// * `point` - The starting point for the line calculation.
    /// * `in_direction` - The direction in which the line should be calculated.
    /// * `num_cells` - The number of cells to include in the line calculation.
    ///
    /// # Returns
    ///
    /// The resulting `URect` that represents the line.
    pub fn calculate_line_from(&self, point: CellIndex2d, in_direction: IVec2, num_cells: u32) -> URect {
        // Calculate the new positions in i32 to allow for negative changes
        let new_x = (point.x as i32 + (in_direction.x * num_cells as i32)).clamp(0, self.max_column_index as i32) as u32;
        let new_y = (point.y as i32 + (in_direction.y * num_cells as i32)).clamp(0, self.max_row_index as i32) as u32;

        // Creating the URect
        URect::from_corners(point.into(), UVec2::new(new_x, new_y))
    }

    #[inline]
    pub fn clamp_rect_to_grid_bounds(&self, rect: URect) -> URect {
        let min_x = rect.min.x.clamp(0, self.max_column_index);
        let min_y = rect.min.y.clamp(0, self.max_row_index);
        let max_x = rect.max.x.clamp(0, self.max_column_index);
        let max_y = rect.max.y.clamp(0, self.max_row_index);

        URect::from_corners(UVec2::new(min_x, min_y), UVec2::new(max_x, max_y))
    }

    pub fn calculate_line_infront_from(&self, point: CellIndex2d, in_direction: IVec2, num_cells: u32) -> URect {
        // Calculate the new positions for the start and end points
        let start_x = (point.x as i32 + in_direction.x).clamp(0, self.max_column_index as i32) as u32;
        let start_y = (point.y as i32 + in_direction.y).clamp(0, self.max_row_index as i32) as u32;

        let end_x = (start_x as i32 + (in_direction.x * (num_cells as i32 - 1))).clamp(0, self.max_column_index as i32) as u32;
        let end_y = (start_y as i32 + (in_direction.y * (num_cells as i32 - 1))).clamp(0, self.max_row_index as i32) as u32;

        // Creating the URect
        URect::from_corners(UVec2::new(start_x, start_y), UVec2::new(end_x, end_y))
    }

    fn get_indexes(&self) -> Array2<CellIndex2d> {
        let mut arr = Array2::from_elem((self.row_number as usize,
                                         self.column_number as usize), CellIndex2d { x: 0, y: 0 });

        for ((col, row), cell_index_2d) in arr.indexed_iter_mut() {
            *cell_index_2d = CellIndex2d {
                x: col as CellIndex1d,
                y: row as CellIndex1d,
            };
        }

        arr
    }

    pub fn get_indexes_segment(&self, area: URect) -> ArrayView2<CellIndex2d> {
        grid_calculations::slice_2d_array(&self.indexes, area)
    }

    #[inline]
    pub fn calculate_cell_position(&self, cell_index: CellIndex2d) -> Vec2 {
        let cell_size = self.cell_size;
        let grid_center = self.shape_rect.center();
        let cell_index_float: Vec2 = cell_index.into();

        Vec2::new(grid_center.x - (self.grid_size.x / 2.0) + (cell_index_float.x * cell_size.x) + (cell_size.x / 2.0),
                  grid_center.y - (self.grid_size.y / 2.0) + (cell_index_float.y * cell_size.y) + (cell_size.y / 2.0))
    }

    #[inline]
    pub fn calculate_indexes_limits_in_rang(&self, center_cell_index: CellIndex2d, radius: u32) -> (CellIndex2d, CellIndex2d) {
        let cell_index_float: UVec2 = center_cell_index.into();

        let min_x = cell_index_float.x.saturating_sub(radius);
        let min_y = cell_index_float.y.saturating_sub(radius);

        let max_x = min(cell_index_float.x + radius, self.max_column_index);
        let max_y = min(cell_index_float.y + radius, self.max_row_index);

        return (CellIndex2d::new(min_x, min_y), CellIndex2d::new(max_x, max_y));
    }


    #[inline]
    pub fn calculate_cell_index_from_position(&self, position: Vec2) -> CellIndex2d {
        let grid_center = self.shape_rect.center();

        let cell_index_x = ((position.x + self.grid_size.x / 2.0 - grid_center.x) / self.cell_size.x).floor() as u32;
        let cell_index_y = ((position.y + self.grid_size.y / 2.0 - grid_center.y) / self.cell_size.y).floor() as u32;

        self.form_grid_bound_cell_index(cell_index_x, cell_index_y)
    }

    #[inline]
    pub fn calc_cell_index_1d_at(&self, cell_index2d: CellIndex2d) -> CellIndex1d {
        grid_calculations::calculate_1d_index(cell_index2d, self.column_number)
    }

    #[inline]
    pub fn calc_cell_index_2d_at(&self, cell_index1d: CellIndex1d) -> CellIndex2d {
        grid_calculations::calculate_2d_index(cell_index1d, self.column_number)
    }

    #[inline]
    pub fn form_grid_bound_cell_index(&self, cell_index_x: u32, cell_index_y: u32) -> CellIndex2d {
        CellIndex2d::new(cell_index_x.clamp(0u32, self.max_column_index), cell_index_y.clamp(0u32, self.max_row_index))
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from_2d(&self, cell_index2d: CellIndex2d) -> SurfaceCoordinate
    {
        let latitude: Coordinate = cell_index2d.x as f32 / self.max_column_index as f32;
        let longitude: Coordinate = cell_index2d.y as f32 / self.max_row_index as f32;
        SurfaceCoordinate { latitude, longitude }
    }

    #[inline]
    pub fn calculate_flat_surface_coordinate_from_1d(&self, cell_index1d: CellIndex1d) -> SurfaceCoordinate {
        self.calculate_flat_surface_coordinate_from_2d(self.calc_cell_index_2d_at(cell_index1d))
    }

    #[inline]
    pub fn is_cell_index_in_grid_bounds(&self, cell_index: CellIndex2d) -> bool {
        u32::from(cell_index.x) < self.column_number && u32::from(cell_index.y) < self.row_number
    }

    #[inline]
    pub fn is_position_in_grid_bounds(&self, position: Vec2) -> bool {
        self.shape_rect.contains(position)
    }

    pub fn visualize_indexes_in_log(&self) {
        println!("{}", "Visualizing grid...".yellow());

        let mut output: Vec<ColoredString> = Vec::new();

        for row in (0..self.row_number).rev() {
            for col in 0..self.column_number {
                let cell_index2d = CellIndex2d::new(col, row);
                let cell_repr: ColoredString = determine_cell_type(&self.segments, &cell_index2d);
                output.push(format!("|{}| ", cell_repr).normal());
            }
            output.push("\n".normal());
        }

        for colored_string in output {
            print!("{}", colored_string);
        }

        for (direction, segment) in self.segments.iter() {
            let direction_repr = format!("{:?}", direction).to_lowercase();
            let segment_repr = format!("{:?}", segment);
            println!("Direction: {}, Segment: {}", direction_repr, segment_repr);
        }
    }
    pub fn visualize_segments_in_log(&self) {
        println!("{}", "Visualizing grid...".yellow());

        let mut output: Vec<ColoredString> = Vec::new();
        //reserve elements
        output.reserve(self.row_number as usize * (self.column_number + 1) as usize);

        for row in (0..self.row_number).rev() {
            for col in 0..self.column_number {
                let cell_index2d = CellIndex2d::new(col, row);
                let cell_repr = determine_cell_symbol(&self.segments, &cell_index2d);
                output.push(format!("|{:2}| ", cell_repr).normal());
            }
            output.push("\n".normal());
        }

        for colored_string in output {
            print!("{}", colored_string);
        }
    }
}

fn determine_cell_type(grid_segments: &HashMap<Direction, URect>, cell_index2d: &CellIndex2d) -> ColoredString
{
    let number = cell_index2d.to_string();
    for segment_rect in grid_segments.iter() {
        if segment_rect.1.contains((*cell_index2d).into()) {
            return number.color(get_color_for(segment_rect.0.clone()));
        }
    }

    number.color(Color::Red)
}

//Get color that corresponds to direction. Colors should be maximum contrasted
pub fn get_color_for(direction: Direction) -> Color {
    match direction {
        Direction::North => Color::Green,
        Direction::NorthEast => Color::BrightGreen,
        Direction::East => Color::Blue,
        Direction::SouthEast => Color::Yellow,
        Direction::South => Color::Magenta,
        Direction::SouthWest => Color::Cyan,
        Direction::West => Color::White,
        Direction::NorthWest => Color::Black,
    }
}

fn determine_cell_symbol(grid_segments: &HashMap<Direction, URect>, cell_index2d: &CellIndex2d) -> String {
    for segment_rect in grid_segments.iter() {
        if segment_rect.1.contains((*cell_index2d).into()) {
            return get_segment_symbol(segment_rect.0.clone());
        }
    }
    return String::default();
}

fn get_direction_symbol(direction: Direction) -> String {
    match direction {
        Direction::North => "↑",
        Direction::NorthEast => "↗",
        Direction::East => "→",
        Direction::SouthEast => "↘",
        Direction::South => "↓",
        Direction::SouthWest => "↙",
        Direction::West => "←",
        Direction::NorthWest => "↖",
    }.to_string()
}

//Get symbol corresponding to direction so, that it will look well in blocks
fn get_segment_symbol(direction: Direction) -> String {
    match direction {
        Direction::North => "N",
        Direction::NorthEast => "NE",
        Direction::East => "E",
        Direction::SouthEast => "SE",
        Direction::South => "S",
        Direction::SouthWest => "SW",
        Direction::West => "W",
        Direction::NorthWest => "NW",
    }.to_string()
}