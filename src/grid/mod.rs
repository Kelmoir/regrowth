//! Hexagonal grid system for spatial organization and queries.
//!
//! This module provides a complete hexagonal grid implementation with:
//! - **Coordinate System**: Efficient axial and cube representations
//! - **Grid Storage**: Generic `Grid<T>` container with iteration
//! - **Spatial Algorithms**: Neighbors, distance, pathfinding, and field-of-view
//!
//! # Quick Start
//!
//! ```
//! # use regrowth::grid::{Grid, coordinates::AxialCoord, algorithms};
//! // Create a 50×50 hexagonal grid
//! let mut grid = Grid::new(50, 50);
//!
//! // Access tiles
//! let hex = AxialCoord::new(25, 25);
//! grid.set_tile(hex, "terrain_data").ok();
//!
//! // Query neighbors
//! let neighbors_list = algorithms::neighbors(hex);
//! assert_eq!(neighbors_list.len(), 6);
//!
//! // Pathfinding
//! let start = AxialCoord::new(0, 0);
//! let target = AxialCoord::new(10, 10);
//! let path = algorithms::pathfind(start, target, |_| true);
//! ```

pub mod algorithms;
pub mod coordinates;
pub mod grid;

// Re-export main types for convenience
pub use coordinates::{AxialCoord, CubeCoord};
pub use grid::Grid;

/// Creates a new hexagonal grid with specified dimensions.
///
/// # Arguments
/// * `width` - Number of columns (typically 50 for standard grid)
/// * `height` - Number of rows (typically 50 for standard grid)
///
/// # Example
/// ```
/// # use regrowth::grid;
/// let grid = grid::create_grid::<u32>(50, 50);
/// assert_eq!(grid.width(), 50);
/// assert_eq!(grid.height(), 50);
/// ```
pub fn create_grid<T: Clone + Default + bevy::reflect::Reflect>(width: usize, height: usize) -> Grid<T> {
    Grid::new(width, height)
}
