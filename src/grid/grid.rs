//! Generic hexagonal grid data structure with storage and iteration.
//!
//! This module provides the `Grid<T>` generic container that stores tile data and
//! handles indexing, iteration, and bounds checking.

use crate::grid::coordinates::AxialCoord;
use bevy::reflect::Reflect;
use std::fmt;

/// A generic hexagonal grid container.
///
/// Stores tile data in a rectangular region of hexagonal space using axial coordinates.
/// The grid is efficient for iteration and bounds checking, as it uses a flat rectangular
/// array internally.
///
/// # Representation
/// - Storage: Flat vector indexed by (col * height + row)
/// - Bounds: All hexes (col, row) where 0 <= col < width and 0 <= row < height
/// - Origin: (0, 0) is at index 0
///
/// # Example
/// ```
/// # use regrowth::grid::{Grid, coordinates::AxialCoord};
/// let mut grid = Grid::new(10, 10);
/// grid.set_tile(AxialCoord::new(5, 5), "tile_data").ok();
/// if let Some(data) = grid.get_tile(AxialCoord::new(5, 5)) {
///     println!("Found: {}", data);
/// }
/// ```
#[derive(Clone, Reflect)]
pub struct Grid<T: Clone + Default + Reflect> {
    /// Width of the grid (number of columns).
    width: usize,

    /// Height of the grid (number of rows).
    height: usize,

    /// Flat vector storing all tiles. Indexed as: col * height + row
    tiles: Vec<T>,
}

impl<T: Clone + Default + Reflect> Grid<T> {
    /// Creates a new hexagonal grid with the specified dimensions.
    ///
    /// All tiles are initialized with `T::default()`.
    ///
    /// # Arguments
    /// * `width` - Number of columns (must be > 0)
    /// * `height` - Number of rows (must be > 0)
    ///
    /// # Panics
    /// Panics if width or height is 0.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::Grid;
    /// let grid: Grid<u32> = Grid::new(50, 50);
    /// assert_eq!(grid.width(), 50);
    /// assert_eq!(grid.height(), 50);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "Grid dimensions must be > 0");
        let size = width * height;
        let tiles = vec![T::default(); size];
        Self {
            width,
            height,
            tiles,
        }
    }

    /// Returns the width (number of columns) of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height (number of rows) of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the total number of tiles in the grid.
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Checks if the grid is empty (always false for valid grids).
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Checks if a coordinate is within the grid bounds.
    ///
    /// Returns true if 0 <= col < width and 0 <= row < height.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::{Grid, coordinates::AxialCoord};
    /// let grid: Grid<u32> = Grid::new(10, 10);
    /// assert!(grid.contains(AxialCoord::new(5, 5)));
    /// assert!(!grid.contains(AxialCoord::new(10, 10)));
    /// ```
    pub fn contains(&self, coord: AxialCoord) -> bool {
        coord.col >= 0
            && (coord.col as usize) < self.width
            && coord.row >= 0
            && (coord.row as usize) < self.height
    }

    /// Converts a coordinate to a linear index in the tile vector.
    ///
    /// Returns `None` if the coordinate is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::{Grid, coordinates::AxialCoord};
    /// let grid: Grid<u32> = Grid::new(10, 10);
    /// let index = grid.coord_to_index(AxialCoord::new(2, 3));
    /// assert!(index.is_some());
    /// ```
    fn coord_to_index(&self, coord: AxialCoord) -> Option<usize> {
        if self.contains(coord) {
            let col = coord.col as usize;
            let row = coord.row as usize;
            Some(col * self.height + row)
        } else {
            None
        }
    }

    /// Converts a linear index back to a coordinate.
    ///
    /// # Panics
    /// Panics if index is out of bounds.
    fn index_to_coord(&self, index: usize) -> AxialCoord {
        debug_assert!(index < self.tiles.len(), "Index out of bounds");
        let col = (index / self.height) as i32;
        let row = (index % self.height) as i32;
        AxialCoord::new(col, row)
    }

    /// Gets a reference to the tile at the given coordinate.
    ///
    /// Returns `None` if the coordinate is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::{Grid, coordinates::AxialCoord};
    /// let mut grid = Grid::new(10, 10);
    /// grid.set_tile(AxialCoord::new(5, 5), 42).ok();
    /// assert_eq!(grid.get_tile(AxialCoord::new(5, 5)), Some(&42));
    /// ```
    pub fn get_tile(&self, coord: AxialCoord) -> Option<&T> {
        self.coord_to_index(coord)
            .and_then(|index| self.tiles.get(index))
    }

    /// Gets a mutable reference to the tile at the given coordinate.
    ///
    /// Returns `None` if the coordinate is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::{Grid, coordinates::AxialCoord};
    /// let mut grid = Grid::new(10, 10);
    /// if let Some(tile) = grid.get_tile_mut(AxialCoord::new(5, 5)) {
    ///     *tile = 100;
    /// }
    /// ```
    pub fn get_tile_mut(&mut self, coord: AxialCoord) -> Option<&mut T> {
        if let Some(index) = self.coord_to_index(coord) {
            self.tiles.get_mut(index)
        } else {
            None
        }
    }

    /// Sets the tile at the given coordinate.
    ///
    /// Returns `Ok(())` on success, or `Err(())` if the coordinate is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::{Grid, coordinates::AxialCoord};
    /// let mut grid = Grid::new(10, 10);
    /// assert!(grid.set_tile(AxialCoord::new(5, 5), 42).is_ok());
    /// assert!(grid.set_tile(AxialCoord::new(100, 100), 42).is_err());
    /// ```
    pub fn set_tile(&mut self, coord: AxialCoord, tile: T) -> Result<(), ()> {
        if let Some(index) = self.coord_to_index(coord) {
            self.tiles[index] = tile;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Returns an iterator over all tiles with their coordinates.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::{Grid, coordinates::AxialCoord};
    /// let grid: Grid<u32> = Grid::new(5, 5);
    /// for (coord, tile) in grid.iter() {
    ///     println!("Tile at {}: {}", coord, tile);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (AxialCoord, &T)> {
        self.tiles.iter().enumerate().map(move |(index, tile)| {
            let coord = self.index_to_coord(index);
            (coord, tile)
        })
    }

    /// Returns a mutable iterator over all tiles with their coordinates.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::{Grid, coordinates::AxialCoord};
    /// let mut grid = Grid::new(5, 5);
    /// for (_coord, tile) in grid.iter_mut() {
    ///     *tile += 1;
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (AxialCoord, &mut T)> {
        let height = self.height;
        self.tiles
            .iter_mut()
            .enumerate()
            .map(move |(index, tile)| {
                let col = (index / height) as i32;
                let row = (index % height) as i32;
                (AxialCoord::new(col, row), tile)
            })
    }

    /// Fills the entire grid with a single tile value.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::Grid;
    /// let mut grid = Grid::new(10, 10);
    /// grid.fill(42);
    /// for (_, tile) in grid.iter() {
    ///     assert_eq!(tile, &42);
    /// }
    /// ```
    pub fn fill(&mut self, tile: T) {
        self.tiles.fill(tile);
    }

    /// Clears the grid by resetting all tiles to their default values.
    pub fn clear(&mut self) {
        self.tiles.fill(T::default());
    }
}

impl<T: Clone + Default + Reflect + fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("tile_count", &self.tiles.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid: Grid<u32> = Grid::new(10, 20);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 20);
        assert_eq!(grid.len(), 200);
    }

    #[test]
    #[should_panic]
    fn test_grid_zero_dimension() {
        let _grid: Grid<u32> = Grid::new(0, 10);
    }

    #[test]
    fn test_grid_contains() {
        let grid: Grid<u32> = Grid::new(10, 10);
        assert!(grid.contains(AxialCoord::new(0, 0)));
        assert!(grid.contains(AxialCoord::new(9, 9)));
        assert!(!grid.contains(AxialCoord::new(10, 10)));
        assert!(!grid.contains(AxialCoord::new(-1, 0)));
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(10, 10);
        let coord = AxialCoord::new(5, 5);
        assert_eq!(grid.get_tile(coord), Some(&0)); // Default value

        assert!(grid.set_tile(coord, 42).is_ok());
        assert_eq!(grid.get_tile(coord), Some(&42));

        assert!(grid.set_tile(AxialCoord::new(100, 100), 99).is_err());
    }

    #[test]
    fn test_grid_get_mut() {
        let mut grid = Grid::new(10, 10);
        let coord = AxialCoord::new(5, 5);
        if let Some(tile) = grid.get_tile_mut(coord) {
            *tile = 100;
        }
        assert_eq!(grid.get_tile(coord), Some(&100));
    }

    #[test]
    fn test_grid_iteration() {
        let mut grid = Grid::new(5, 5);
        for (_coord, tile) in grid.iter_mut() {
            *tile = 10;
        }

        let mut count = 0;
        for (_coord, tile) in grid.iter() {
            assert_eq!(tile, &10);
            count += 1;
        }
        assert_eq!(count, 25);
    }

    #[test]
    fn test_grid_fill() {
        let mut grid = Grid::new(5, 5);
        grid.fill(42);
        for (_coord, tile) in grid.iter() {
            assert_eq!(tile, &42);
        }
    }

    #[test]
    fn test_grid_corner_coordinates() {
        let grid: Grid<u32> = Grid::new(50, 50);
        assert!(grid.contains(AxialCoord::new(0, 0))); // Top-left
        assert!(grid.contains(AxialCoord::new(49, 49))); // Bottom-right
        assert!(!grid.contains(AxialCoord::new(50, 0))); // Out of bounds
        assert!(!grid.contains(AxialCoord::new(0, 50))); // Out of bounds
    }

    #[test]
    fn test_grid_index_conversion() {
        let grid: Grid<u32> = Grid::new(10, 20);
        let coord = AxialCoord::new(5, 10);
        assert!(grid.contains(coord));

        // This tests internal consistency (no public API for index conversion)
        let index = grid.coord_to_index(coord).unwrap();
        let coord2 = grid.index_to_coord(index);
        assert_eq!(coord, coord2);
    }

    #[test]
    fn test_grid_is_empty() {
        let grid: Grid<u32> = Grid::new(10, 10);
        // Valid grid with dimensions > 0 should never be "empty"
        assert!(!grid.is_empty());
    }

    #[test]
    fn test_grid_clear() {
        let mut grid = Grid::new(5, 5);
        grid.fill(42);
        
        // Verify all tiles are set to 42
        for (_coord, tile) in grid.iter() {
            assert_eq!(*tile, 42);
        }
        
        // Clear the grid
        grid.clear();
        
        // Verify all tiles are reset to default (0)
        for (_coord, tile) in grid.iter() {
            assert_eq!(*tile, 0);
        }
    }

    #[test]
    fn test_grid_debug_format() {
        let grid: Grid<u32> = Grid::new(5, 10);
        let debug_str = format!("{:?}", grid);
        assert!(debug_str.contains("Grid"));
        assert!(debug_str.contains("width"));
        assert!(debug_str.contains("height"));
    }

    #[test]
    fn test_large_grid_creation() {
        let grid: Grid<u32> = Grid::new(50, 50);
        assert_eq!(grid.width(), 50);
        assert_eq!(grid.height(), 50);
        assert_eq!(grid.len(), 2500);
        
        // Verify corners are in bounds
        assert!(grid.contains(AxialCoord::new(0, 0)));
        assert!(grid.contains(AxialCoord::new(49, 49)));
    }
}
