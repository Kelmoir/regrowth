//! Hexagonal coordinate system with axial and cube representations.
//!
//! This module provides efficient coordinate representations for hexagonal grids:
//! - **Axial coordinates** (col, row): 2D representation used for storage and iteration
//! - **Cube coordinates** (x, y, z where x+y+z=0): 3D representation used for math operations
//!
//! Hexagonal grids have 6 directions. Direction constants represent the 6 neighbors:
//! - NE (Northeast), E (East), SE (Southeast)
//! - SW (Southwest), W (West), NW (Northwest)

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Hexagon size measurement constants.
/// Represents a 5cm hexagon tile (flat-top orientation).
#[allow(dead_code)]
pub mod measurements {
    /// Size of a single hexagon tile in centimeters.
    pub const HEX_SIZE_CM: f32 = 5.0;

    /// Conversion factor from game units to centimeters.
    /// 1 game unit = 1 cm for simplicity.
    pub const GAME_UNIT_TO_CM: f32 = 1.0;

    /// Conversion factor from centimeters to game units.
    pub const CM_TO_GAME_UNIT: f32 = 1.0 / GAME_UNIT_TO_CM;

    /// Outer radius of hex in game units (distance from center to vertex).
    pub const HEX_OUTER_RADIUS: f32 = HEX_SIZE_CM * CM_TO_GAME_UNIT;

    /// Inner radius of hex in game units (distance from center to edge midpoint).
    pub const HEX_INNER_RADIUS: f32 = HEX_OUTER_RADIUS * 0.866_025_4;

    /// Width of flat-top hex in game units.
    pub const HEX_WIDTH: f32 = 2.0 * HEX_OUTER_RADIUS;

    /// Height of flat-top hex in game units.
    pub const HEX_HEIGHT: f32 = 2.0 * HEX_INNER_RADIUS;
}

/// Direction vectors in axial coordinates (col, row).
/// The 6 cardinal directions for hexagonal grids.
#[allow(dead_code)]
pub mod directions {
    use super::AxialCoord;

    /// Northeast: one step in positive column direction.
    pub const NE: AxialCoord = AxialCoord { col: 1, row: 0 };

    /// East: one step in positive column and negative row direction.
    pub const E: AxialCoord = AxialCoord { col: 1, row: -1 };

    /// Southeast: one step in negative row direction.
    pub const SE: AxialCoord = AxialCoord { col: 0, row: -1 };

    /// Southwest: one step in negative column direction.
    pub const SW: AxialCoord = AxialCoord { col: -1, row: 0 };

    /// West: one step in negative column and positive row direction.
    pub const W: AxialCoord = AxialCoord { col: -1, row: 1 };

    /// Northwest: one step in positive row direction.
    pub const NW: AxialCoord = AxialCoord { col: 0, row: 1 };

    /// All 6 directions in order.
    pub const ALL: [AxialCoord; 6] = [NE, E, SE, SW, W, NW];
}

/// Axial coordinate system for hexagonal grids.
///
/// Axial coordinates use just two dimensions (col, row) and are efficient for storage
/// and iteration. The third coordinate (s = -col - row) is derived implicitly.
///
/// Advantages:
/// - Compact 2D representation
/// - Efficient storage and iteration
/// - Simple bounds checking for rectangular grid
///
/// # Example
/// ```
/// # use regrowth::grid::coordinates::AxialCoord;
/// let hex = AxialCoord { col: 5, row: 3 };
/// let cube = hex.to_cube();
/// assert_eq!(cube.z, -8);
/// ```
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Hash, Default, Reflect, Component, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct AxialCoord {
    /// Column index (x-axis in axial space).
    pub col: i32,

    /// Row index (y-axis in axial space).
    pub row: i32,
}

impl AxialCoord {
    /// Creates a new axial coordinate.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::AxialCoord;
    /// let hex = AxialCoord::new(2, -3);
    /// assert_eq!(hex.col, 2);
    /// assert_eq!(hex.row, -3);
    /// ```
    pub const fn new(col: i32, row: i32) -> Self {
        Self { col, row }
    }

    /// Returns the derived third coordinate in axial space.
    ///
    /// In axial system, s = -col - row. This is derived from the cube constraint x+y+z=0.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::AxialCoord;
    /// let hex = AxialCoord::new(2, 3);
    /// assert_eq!(hex.s(), -5);
    /// ```
    pub const fn s(&self) -> i32 {
        -self.col - self.row
    }

    /// Converts to cube coordinates.
    ///
    /// Cube coordinates (x, y, z) satisfy the constraint x + y + z = 0 and enable
    /// efficient distance calculations and rotations.
    ///
    /// Conversion:
    /// - x = col
    /// - y = row
    /// - z = -col - row (i.e., s)
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::{AxialCoord, CubeCoord};
    /// let axial = AxialCoord::new(2, 3);
    /// let cube = axial.to_cube();
    /// assert_eq!(cube.x, 2);
    /// assert_eq!(cube.y, 3);
    /// assert_eq!(cube.z, -5);
    /// assert_eq!(cube.x + cube.y + cube.z, 0);
    /// ```
    pub const fn to_cube(self) -> CubeCoord {
        CubeCoord {
            x: self.col,
            y: self.row,
            z: self.s(),
        }
    }

    /// Adds a direction vector to this coordinate.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::{AxialCoord, directions};
    /// let hex = AxialCoord::new(0, 0);
    /// let neighbor = hex.add_direction(directions::E);
    /// assert_eq!(neighbor, AxialCoord::new(1, -1));
    /// ```
    pub const fn add_direction(self, dir: AxialCoord) -> AxialCoord {
        AxialCoord {
            col: self.col + dir.col,
            row: self.row + dir.row,
        }
    }

    /// Computes the hexagonal distance to another coordinate.
    ///
    /// Uses the cube coordinate distance formula: distance = (|x| + |y| + |z|) / 2
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::AxialCoord;
    /// let a = AxialCoord::new(0, 0);
    /// let b = AxialCoord::new(2, 1);
    /// assert_eq!(a.distance_to(b), 3);
    /// ```
    pub fn distance_to(self, other: AxialCoord) -> usize {
        let cube_self = self.to_cube();
        let cube_other = other.to_cube();
        cube_self.distance_to(cube_other)
    }

    /// Converts to world coordinates for rendering.
    ///
    /// Assumes flat-top hexagon orientation and uses pointy-based layout.
    /// Returns (x, y) in world space.
    ///
    /// # Arguments
    /// * `hex_size` - The radius of the hexagon in world units
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::AxialCoord;
    /// let hex = AxialCoord::new(1, 0);
    /// let (x, y) = hex.to_world(10.0);
    /// assert!((x - 20.0).abs() < 0.1); // Approximate
    /// ```
    pub fn to_world(self, hex_size: f32) -> (f32, f32) {
        let col_f = self.col as f32;
        let row_f = self.row as f32;

        let x = hex_size * (3.0 / 2.0 * col_f);
        let y = hex_size * (3.0_f32.sqrt() / 2.0 * col_f + 3.0_f32.sqrt() * row_f);

        (x, y)
    }

    /// Converts from world coordinates to the nearest hex.
    ///
    /// Uses cube coordinate rounding for accuracy.
    ///
    /// # Arguments
    /// * `x` - X coordinate in world space
    /// * `y` - Y coordinate in world space
    /// * `hex_size` - The radius of the hexagon in world units
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::AxialCoord;
    /// let hex = AxialCoord::new(3, 2);
    /// let (x, y) = hex.to_world(5.0);
    /// let hex_again = AxialCoord::from_world(x, y, 5.0);
    /// assert!(hex.distance_to(hex_again) <= 1); // Within one hex due to rounding
    /// ```
    pub fn from_world(x: f32, y: f32, hex_size: f32) -> Self {
        // Convert to cube-like fractional coordinates
        let col_f = 2.0 / 3.0 * x / hex_size;
        let row_f = -1.0 / 3.0 * x / hex_size + 3.0_f32.sqrt() / 3.0 * y / hex_size;
        let s_f = -col_f - row_f;

        // Round to nearest hex using cube coordinate rounding
        let cube = CubeCoord {
            x: col_f.round() as i32,
            y: row_f.round() as i32,
            z: s_f.round() as i32,
        };

        cube.normalize().to_axial()
    }
}

impl fmt::Display for AxialCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

impl std::ops::Add for AxialCoord {
    type Output = AxialCoord;

    fn add(self, rhs: AxialCoord) -> AxialCoord {
        AxialCoord {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl std::ops::Sub for AxialCoord {
    type Output = AxialCoord;

    fn sub(self, rhs: AxialCoord) -> AxialCoord {
        AxialCoord {
            col: self.col - rhs.col,
            row: self.row - rhs.row,
        }
    }
}

/// Cube coordinate system for hexagonal grids.
///
/// Cube coordinates use three dimensions (x, y, z) with the constraint x + y + z = 0.
/// While more memory-intensive than axial, they simplify mathematical operations like
/// distance calculations, rotations, and line-of-sight.
///
/// Advantages:
/// - Elegant distance metric: (|x| + |y| + |z|) / 2
/// - Easy rotation and symmetry operations
/// - Useful for raycasting and pathfinding
///
/// # Example
/// ```
/// # use regrowth::grid::coordinates::CubeCoord;
/// let cube = CubeCoord { x: 2, y: 3, z: -5 };
/// assert_eq!(cube.x + cube.y + cube.z, 0); // Constraint satisfied
/// let distance = cube.distance_to(CubeCoord::new(0, 0, 0));
/// assert_eq!(distance, 5);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect)]
pub struct CubeCoord {
    /// X coordinate.
    pub x: i32,

    /// Y coordinate.
    pub y: i32,

    /// Z coordinate (derived: z = -x - y).
    pub z: i32,
}

impl CubeCoord {
    /// Creates a new cube coordinate.
    ///
    /// # Panics
    /// Panics if x + y + z ≠ 0 (constraint violation).
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::CubeCoord;
    /// let cube = CubeCoord::new(2, 3, -5);
    /// assert_eq!(cube.z, -5);
    /// ```
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        debug_assert_eq!(x + y + z, 0, "Cube coordinate constraint violated: x + y + z must be 0");
        Self { x, y, z }
    }

    /// Converts to axial coordinates.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::{CubeCoord, AxialCoord};
    /// let cube = CubeCoord::new(2, 3, -5);
    /// let axial = cube.to_axial();
    /// assert_eq!(axial, AxialCoord::new(2, 3));
    /// ```
    pub const fn to_axial(self) -> AxialCoord {
        AxialCoord {
            col: self.x,
            row: self.y,
        }
    }

    /// Computes the hexagonal distance to another cube coordinate.
    ///
    /// Uses the cube distance formula: distance = (|x| + |y| + |z|) / 2
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::CubeCoord;
    /// let a = CubeCoord::new(0, 0, 0);
    /// let b = CubeCoord::new(2, 3, -5);
    /// assert_eq!(a.distance_to(b), 5);
    /// ```
    pub fn distance_to(self, other: CubeCoord) -> usize {
        ((self.x - other.x).unsigned_abs() as usize
            + (self.y - other.y).unsigned_abs() as usize
            + (self.z - other.z).unsigned_abs() as usize)
            / 2
    }

    /// Normalizes the cube coordinate by fixing the z-component.
    ///
    /// Useful when working with fractional cube coordinates that need to snap
    /// to the nearest integer hex.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::CubeCoord;
    /// let mut cube = CubeCoord { x: 2, y: 3, z: -4 }; // z should be -5
    /// cube = cube.normalize();
    /// assert_eq!(cube.z, -5);
    /// ```
    pub fn normalize(mut self) -> CubeCoord {
        self.z = -self.x - self.y;
        self
    }

    /// Adds two cube coordinates.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::CubeCoord;
    /// let a = CubeCoord::new(1, 0, -1);
    /// let b = CubeCoord::new(0, 1, -1);
    /// let result = a.add(b);
    /// assert_eq!(result, CubeCoord::new(1, 1, -2));
    /// ```
    pub fn add(self, other: CubeCoord) -> CubeCoord {
        CubeCoord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// Subtracts another cube coordinate from this one.
    ///
    /// # Example
    /// ```
    /// # use regrowth::grid::coordinates::CubeCoord;
    /// let a = CubeCoord::new(2, 1, -3);
    /// let b = CubeCoord::new(1, 0, -1);
    /// let result = a.subtract(b);
    /// assert_eq!(result, CubeCoord::new(1, 1, -2));
    /// ```
    pub fn subtract(self, other: CubeCoord) -> CubeCoord {
        CubeCoord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl fmt::Display for CubeCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axial_coord_creation() {
        let hex = AxialCoord::new(2, 3);
        assert_eq!(hex.col, 2);
        assert_eq!(hex.row, 3);
    }

    #[test]
    fn test_axial_coord_s_calculation() {
        let hex = AxialCoord::new(2, 3);
        assert_eq!(hex.s(), -5);

        let hex2 = AxialCoord::new(-1, -1);
        assert_eq!(hex2.s(), 2);
    }

    #[test]
    fn test_axial_to_cube_conversion() {
        let axial = AxialCoord::new(2, 3);
        let cube = axial.to_cube();
        assert_eq!(cube.x, 2);
        assert_eq!(cube.y, 3);
        assert_eq!(cube.z, -5);
        assert_eq!(cube.x + cube.y + cube.z, 0);
    }

    #[test]
    fn test_cube_to_axial_conversion() {
        let cube = CubeCoord::new(2, 3, -5);
        let axial = cube.to_axial();
        assert_eq!(axial.col, 2);
        assert_eq!(axial.row, 3);
    }

    #[test]
    fn test_axial_distance_origin() {
        let origin = AxialCoord::new(0, 0);
        let hex = AxialCoord::new(2, 1);
        assert_eq!(origin.distance_to(hex), 3);
    }

    #[test]
    fn test_cube_distance() {
        let a = CubeCoord::new(0, 0, 0);
        let b = CubeCoord::new(2, 3, -5);
        assert_eq!(a.distance_to(b), 5);
    }

    #[test]
    fn test_direction_addition() {
        let hex = AxialCoord::new(0, 0);
        let neighbor = hex.add_direction(directions::E);
        assert_eq!(neighbor, AxialCoord::new(1, -1));
    }

    #[test]
    fn test_all_directions() {
        let origin = AxialCoord::new(0, 0);
        for dir in directions::ALL {
            let neighbor = origin.add_direction(dir);
            assert_eq!(origin.distance_to(neighbor), 1);
        }
    }

    #[test]
    fn test_world_conversion_roundtrip() {
        let hex = AxialCoord::new(3, 2);
        let hex_size = 10.0;
        let (x, y) = hex.to_world(hex_size);
        let hex_again = AxialCoord::from_world(x, y, hex_size);
        assert_eq!(hex, hex_again);
    }

    #[test]
    fn test_axial_operators() {
        let a = AxialCoord::new(1, 2);
        let b = AxialCoord::new(3, -1);
        let sum = a + b;
        assert_eq!(sum, AxialCoord::new(4, 1));

        let diff = b - a;
        assert_eq!(diff, AxialCoord::new(2, -3));
    }

    #[test]
    fn test_axial_display() {
        let hex = AxialCoord::new(3, -5);
        let display_str = format!("{}", hex);
        assert_eq!(display_str, "(3, -5)");
    }

    #[test]
    fn test_cube_coord_add() {
        let a = CubeCoord::new(1, 0, -1);
        let b = CubeCoord::new(0, 1, -1);
        let result = a.add(b);
        assert_eq!(result, CubeCoord::new(1, 1, -2));
        assert_eq!(result.x + result.y + result.z, 0); // Constraint satisfied
    }

    #[test]
    fn test_cube_coord_subtract() {
        let a = CubeCoord::new(2, 1, -3);
        let b = CubeCoord::new(1, 0, -1);
        let result = a.subtract(b);
        assert_eq!(result, CubeCoord::new(1, 1, -2));
        assert_eq!(result.x + result.y + result.z, 0);
    }

    #[test]
    fn test_cube_coord_normalize() {
        let cube = CubeCoord { x: 2, y: 3, z: -4 }; // z is wrong, should be -5
        let normalized = cube.normalize();
        assert_eq!(normalized.z, -5);
        assert_eq!(normalized.x + normalized.y + normalized.z, 0);
    }

    #[test]
    fn test_cube_display() {
        let cube = CubeCoord::new(1, -2, 1);
        let display_str = format!("{}", cube);
        assert_eq!(display_str, "(1, -2, 1)");
    }

    #[test]
    fn test_negative_coordinates() {
        let hex = AxialCoord::new(-5, -3);
        assert_eq!(hex.col, -5);
        assert_eq!(hex.row, -3);
        assert_eq!(hex.s(), 8);
        
        let cube = hex.to_cube();
        assert_eq!(cube.x + cube.y + cube.z, 0);
    }

    #[test]
    fn test_distance_same_point() {
        let hex = AxialCoord::new(3, 2);
        assert_eq!(hex.distance_to(hex), 0);
    }

    #[test]
    fn test_distance_large_separation() {
        let a = AxialCoord::new(0, 0);
        let b = AxialCoord::new(10, 10);
        let dist = a.distance_to(b);
        assert!(dist > 0);
        
        // Verify symmetry: distance(a, b) == distance(b, a)
        assert_eq!(dist, b.distance_to(a));
    }
}
