//! Hexagonal grid algorithms for neighbors, distance, raycasting, and rings.
//!
//! This module provides efficient algorithms for spatial queries on hexagonal grids:
//! - Neighbor finding: All 6 adjacent hexes
//! - Distance calculation: Hexagonal distance metric
//! - Line-of-sight: Raycasting between two hexes
//! - Ring enumeration: All hexes at a specific distance

use crate::grid::coordinates::{AxialCoord, CubeCoord, directions};
use std::collections::VecDeque;

/// Finds all 6 immediate neighbors of a given hex coordinate.
///
/// Returns an array of the 6 adjacent hexagon coordinates in the order:
/// NE, E, SE, SW, W, NW.
///
/// # Example
/// ```
/// # use regrowth::grid::algorithms::neighbors;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let center = AxialCoord::new(0, 0);
/// let neighbors_list = neighbors(center);
/// assert_eq!(neighbors_list.len(), 6);
/// for neighbor in neighbors_list {
///     assert_eq!(center.distance_to(neighbor), 1);
/// }
/// ```
#[allow(dead_code)]
pub fn neighbors(coord: AxialCoord) -> [AxialCoord; 6] {
    [
        coord.add_direction(directions::NE),
        coord.add_direction(directions::E),
        coord.add_direction(directions::SE),
        coord.add_direction(directions::SW),
        coord.add_direction(directions::W),
        coord.add_direction(directions::NW),
    ]
}

/// Returns all hexes at exactly the specified distance from a center hex.
///
/// Uses a ring-walking algorithm: starts at distance away from the center
/// and walks around the perimeter.
///
/// # Arguments
/// * `center` - The hexagon to measure from
/// * `distance` - The desired distance (0 returns vec![center])
///
/// # Example
/// ```
/// # use regrowth::grid::algorithms::ring_at;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let center = AxialCoord::new(0, 0);
/// let ring_1 = ring_at(center, 1);
/// assert_eq!(ring_1.len(), 6); // Distance 1 has 6 hexes
///
/// let ring_2 = ring_at(center, 2);
/// assert_eq!(ring_2.len(), 12); // Distance 2 has 12 hexes
/// ```
#[allow(dead_code)]
pub fn ring_at(center: AxialCoord, distance: usize) -> Vec<AxialCoord> {
    if distance == 0 {
        return vec![center];
    }

    let mut result = Vec::new();

    // Walk around the 6 edges of the hex ring
    for direction_idx in 0..6 {
        // Start position for this edge: distance steps in the radial direction
        let radial_dir = directions::ALL[direction_idx];
        let mut current = AxialCoord::new(
            center.col + radial_dir.col * distance as i32,
            center.row + radial_dir.row * distance as i32,
        );

        // Direction to walk along the edge: 2 directions counterclockwise from radial
        let edge_dir = directions::ALL[(direction_idx + 2) % 6];

        // Walk along this edge for "distance" steps
        for _ in 0..distance {
            result.push(current);
            current = current + edge_dir;
        }
    }

    result
}

/// Returns all hexes within the specified distance from a center hex.
///
/// Returns a vector of all coordinates where distance <= max_distance,
/// including the center itself.
///
/// # Example
/// ```
/// # use regrowth::grid::algorithms::disk_at;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let center = AxialCoord::new(0, 0);
/// let disk = disk_at(center, 2);
/// assert_eq!(disk.len(), 1 + 6 + 12); // Center + ring 1 + ring 2
/// ```
#[allow(dead_code)]
pub fn disk_at(center: AxialCoord, max_distance: usize) -> Vec<AxialCoord> {
    let mut result = Vec::new();
    for distance in 0..=max_distance {
        result.extend(ring_at(center, distance));
    }
    result
}

/// Checks if the border between two adjacent tiles can be crossed based on elevation.
///
/// Uses the elevation difference between tiles to determine passability. If the difference
/// exceeds the maximum crossable elevation, the border is impassable.
///
/// # Arguments
/// * `from` - The starting tile coordinate
/// * `to` - The destination tile coordinate
/// * `get_elevation` - Function returning the elevation (meters) for a coordinate
/// * `max_elevation_diff` - Maximum elevation difference in meters to allow crossing
///
/// # Returns
/// True if the border can be crossed, false if elevation difference is too steep.
///
/// # Example
/// ```ignore
/// # use regrowth::grid::algorithms::can_cross_border;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let from = AxialCoord::new(0, 0);
/// let to = AxialCoord::new(1, 0);
/// let can_cross = can_cross_border(from, to, |coord| {
///     // Return elevation for coord from your world state
///     10.0
/// }, 2.5); // Max 2.5m elevation difference
/// ```
#[allow(dead_code)]
fn can_cross_border<F>(
    from: AxialCoord,
    to: AxialCoord,
    get_elevation: F,
    max_elevation_diff: f32,
) -> bool
where
    F: Fn(AxialCoord) -> f32,
{
    let from_elev = get_elevation(from);
    let to_elev = get_elevation(to);
    let diff = (from_elev - to_elev).abs();
    diff <= max_elevation_diff
}

/// Combined passability check for terrain AND elevation across a border.
///
/// Validates that a tile can be entered from a source tile based on both terrain type
/// and elevation difference. This is the primary function to use for complete passability 
/// validation in pathfinding and movement.
///
/// # Arguments
/// * `from` - Source coordinate (the tile we're leaving)
/// * `to` - Destination coordinate (the tile we're entering) 
/// * `is_terrain_passable` - Function returning true if a tile's terrain allows entry
/// * `get_elevation` - Function returning elevation (meters) for a coordinate
/// * `max_elevation_diff` - Maximum elevation difference in meters to allow crossing
///
/// # Returns
/// True if both terrain permits entry AND elevation difference is within limits.
///
/// # Example
/// ```ignore
/// # use regrowth::grid::algorithms::can_enter_from;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let from = AxialCoord::new(0, 0);
/// let to = AxialCoord::new(1, 0);
///
/// let can_move = can_enter_from(
///     from, to,
///     |hex| !is_water(hex) && !is_fence(hex),  // terrain check
///     |hex| get_elevation(hex),                 // elevation lookup
///     2.5                                        // max 2.5m slope
/// );
/// ```
#[allow(dead_code)]
pub fn can_enter_from<F1, F2>(
    from: AxialCoord,
    to: AxialCoord,
    is_terrain_passable: F1,
    get_elevation: F2,
    max_elevation_diff: f32,
) -> bool
where
    F1: Fn(AxialCoord) -> bool,
    F2: Fn(AxialCoord) -> f32,
{
    // Destination terrain must be passable
    if !is_terrain_passable(to) {
        return false;
    }

    // Elevation difference between tiles must be within limits
    can_cross_border(from, to, get_elevation, max_elevation_diff)
}

/// Computes the line-of-sight path between two hexes using linear interpolation.
///
/// Uses a Bresenham-like algorithm adapted for cube coordinates. Returns all hexes
/// along the line from `start` to `end`, inclusive.
///
/// # Guarantees
/// - Starts with `start` and ends with `end`
/// - Returns all hexes along the straight line
/// - Useful for raycasting and visibility checks
///
/// # Example
/// ```
/// # use regrowth::grid::algorithms::line_to;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let start = AxialCoord::new(0, 0);
/// let end = AxialCoord::new(3, 1);
/// let path = line_to(start, end);
/// assert_eq!(path[0], start);
/// assert_eq!(path[path.len() - 1], end);
/// ```
pub fn line_to(start: AxialCoord, end: AxialCoord) -> Vec<AxialCoord> {
    let distance = start.distance_to(end);
    if distance == 0 {
        return vec![start];
    }

    let mut result = Vec::with_capacity(distance + 1);

    let start_cube = start.to_cube();
    let end_cube = end.to_cube();

    let distance_f = distance as f32;

    for step in 0..=distance {
        let t = step as f32 / distance_f;

        // Linear interpolation in cube space
        let x = start_cube.x as f32 + (end_cube.x as f32 - start_cube.x as f32) * t;
        let y = start_cube.y as f32 + (end_cube.y as f32 - start_cube.y as f32) * t;
        let z = start_cube.z as f32 + (end_cube.z as f32 - start_cube.z as f32) * t;

        // Round to nearest cube coordinate
        let rx = x.round() as i32;
        let ry = y.round() as i32;
        let rz = z.round() as i32;

        // Fix rounding errors to maintain x+y+z=0 constraint
        let (rx, ry, rz) = if (rx + ry + rz).abs() > 0 {
            let dx = (x.round() - x).abs();
            let dy = (y.round() - y).abs();
            let dz = (z.round() - z).abs();

            if dx > dy && dx > dz {
                (-ry - rz, ry, rz)
            } else if dy > dz {
                (rx, -rx - rz, rz)
            } else {
                (rx, ry, -rx - ry)
            }
        } else {
            (rx, ry, rz)
        };

        let cube = CubeCoord { x: rx, y: ry, z: rz };
        result.push(cube.to_axial());
    }

    // Remove duplicates that can occur from rounding
    result.dedup();

    result
}

/// Performs breadth-first pathfinding from start to target, avoiding obstacles.
///
/// Returns the shortest path in terms of hexagonal distance. Uses a provided
/// function to check if a hex is walkable. The `is_walkable` closure is the key
/// to implementing terrain and elevation constraints.
///
/// # Arguments
/// * `start` - Starting hex
/// * `target` - Target hex
/// * `is_walkable` - Function returning true if a hex can be traversed
///
/// # Returns
/// - `Some(path)` containing all hexes from start to target (inclusive)
/// - `None` if no path exists
///
/// # Basic Example
/// ```
/// # use regrowth::grid::algorithms::pathfind;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let start = AxialCoord::new(0, 0);
/// let target = AxialCoord::new(5, 5);
/// let path = pathfind(start, target, |_hex| true); // All hexes walkable
/// assert!(path.is_some());
/// ```
///
/// # Example with Terrain Constraints
///
/// This example shows how to implement terrain-aware pathfinding by providing
/// a closure that checks both terrain type and elevation:
///
/// ```ignore
/// # use regrowth::grid::algorithms::pathfind;
/// # use regrowth::grid::coordinates::AxialCoord;
/// # use regrowth::components::{Terrain, TerrainType, Elevation, MovementRules};
/// # use bevy::prelude::*;
///
/// // Assume these are available from your ECS world:
/// // - grid: Grid that stores entities/tiles
/// // - terrain_query: Query to get Terrain and Elevation components
/// // - movement_rules: Resource with passability rules
///
/// let start = AxialCoord::new(0, 0);
/// let target = AxialCoord::new(20, 15);
///
/// let path = pathfind(start, target, |hex| {
///     // Check 1: Terrain passability - is this tile's terrain type traversable?
///     let terrain_ok = if let Ok(terrain) = terrain_query.get_component::<Terrain>(tile_entity) {
///         !terrain.terrain_type.is_impassable()
///     } else {
///         true // Default to passable if no terrain component
///     };
///
///     // Check 2: Elevation reasonableness - not too steep for this tile alone
///     // (Border crossing checks happen implicitly by checking neighbors)
///     let elev_ok = if let Ok(elevation) = terrain_query.get_component::<Elevation>(tile_entity) {
///         elevation.height_meters < 100.0 // Arbitrary steep cliff threshold
///     } else {
///         true
///     };
///
///     terrain_ok && elev_ok
/// });
///
/// if let Some(path) = path {
///     println!("Found path with {} steps", path.len());
/// } else {
///     println!("No passable path exists");
/// }
/// ```
///
/// # Advanced: Border Crossing with Elevation
///
/// For even stricter elevation constraints where movement across tile *borders*
/// is blocked by steep slopes, you would need to modify this to pass additional
/// context (prior coordinate) to the closure, or implement a custom pathfinding
/// that has access to both current and neighbor elevations:
///
/// ```ignore
/// // Pseudo-code for stricter elevation checking:
/// let max_slope = movement_rules.max_crossable_elevation_diff;
///
/// for neighbor in neighbors(current) {
///     let current_elev = get_elevation(current);
///     let neighbor_elev = get_elevation(neighbor);
///     let slope = (current_elev - neighbor_elev).abs();
///
///     // Only traverse if slope is passable AND terrain is traversable
///     if slope <= max_slope && is_terrain_passable(neighbor) {
///         // Add neighbor to pathfinding queue
///     }
/// }
/// ```
pub fn pathfind<F>(
    start: AxialCoord,
    target: AxialCoord,
    is_walkable: F,
) -> Option<Vec<AxialCoord>>
where
    F: Fn(AxialCoord) -> bool,
{
    if !is_walkable(start) || !is_walkable(target) {
        return None;
    }

    if start == target {
        return Some(vec![start]);
    }

    use std::collections::HashMap;

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back(start);
    visited.insert(start, None);

    while let Some(current) = queue.pop_front() {
        if current == target {
            // Reconstruct path
            let mut path = vec![current];
            let mut prev = visited[&current];

            while let Some(p) = prev {
                path.push(p);
                prev = visited[&p];
            }

            path.reverse();
            return Some(path);
        }

        for neighbor in neighbors(current) {
            if !visited.contains_key(&neighbor) && is_walkable(neighbor) {
                visited.insert(neighbor, Some(current));
                queue.push_back(neighbor);
            }
        }
    }

    None
}

/// Field-of-view computation using the shadow-casting algorithm.
///
/// Returns all hexes visible from origin within max_distance, treating obstacles
/// as blocking sight.
///
/// # Arguments
/// * `origin` - The viewing position
/// * `max_distance` - Maximum sight distance
/// * `is_blocked` - Function returning true if hex blocks vision
///
/// # Returns
/// Vector of all visible hex coordinates (including origin if not blocked)
///
/// # Example
/// ```
/// # use regrowth::grid::algorithms::field_of_view;
/// # use regrowth::grid::coordinates::AxialCoord;
/// let origin = AxialCoord::new(0, 0);
/// let fov = field_of_view(origin, 5, |_hex| false); // No obstacles
/// // Should contain roughly PI * radius^2 hexes (disk)
/// ```
#[allow(dead_code)]
pub fn field_of_view<F>(
    origin: AxialCoord,
    max_distance: usize,
    is_blocked: F,
) -> Vec<AxialCoord>
where
    F: Fn(AxialCoord) -> bool,
{
    let mut visible = Vec::new();
    let mut blocked_lines = Vec::new();

    for distance in 0..=max_distance {
        let ring = ring_at(origin, distance);

        for hex in ring {
            if is_blocked(hex) {
                blocked_lines.push(line_to(origin, hex));
                continue;
            }

            let mut is_visible = true;
            let line = line_to(origin, hex);

            for blocked_line in &blocked_lines {
                // Check if line passes through any blocked hex
                for blocked_hex in blocked_line {
                    if line.contains(blocked_hex) {
                        is_visible = false;
                        break;
                    }
                }
                if !is_visible {
                    break;
                }
            }

            if is_visible {
                visible.push(hex);
            }
        }
    }

    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let center = AxialCoord::new(0, 0);
        let neighbors_list = neighbors(center);
        assert_eq!(neighbors_list.len(), 6);

        for neighbor in neighbors_list {
            assert_eq!(center.distance_to(neighbor), 1);
        }
    }

    #[test]
    fn test_ring_at_distance_1() {
        let center = AxialCoord::new(0, 0);
        let ring = ring_at(center, 1);
        assert_eq!(ring.len(), 6);

        for hex in ring {
            assert_eq!(center.distance_to(hex), 1);
        }
    }

    #[test]
    fn test_ring_at_distance_2() {
        let center = AxialCoord::new(0, 0);
        let ring = ring_at(center, 2);
        assert_eq!(ring.len(), 12);

        for hex in ring {
            assert_eq!(center.distance_to(hex), 2);
        }
    }

    #[test]
    fn test_ring_at_zero_distance() {
        let center = AxialCoord::new(5, 5);
        let ring = ring_at(center, 0);
        assert_eq!(ring.len(), 1);
        assert_eq!(ring[0], center);
    }

    #[test]
    fn test_disk_at() {
        let center = AxialCoord::new(0, 0);
        let disk = disk_at(center, 2);
        assert_eq!(disk.len(), 1 + 6 + 12); // Center + ring 1 + ring 2 = 19
    }

    #[test]
    fn test_line_to_same_point() {
        let hex = AxialCoord::new(5, 5);
        let line = line_to(hex, hex);
        assert_eq!(line.len(), 1);
        assert_eq!(line[0], hex);
    }

    #[test]
    fn test_line_to_neighbor() {
        let start = AxialCoord::new(0, 0);
        let end = AxialCoord::new(1, 0);
        let line = line_to(start, end);
        assert!(line.len() >= 2);
        assert_eq!(line[0], start);
        assert_eq!(line[line.len() - 1], end);
    }

    #[test]
    fn test_line_to_distance_3() {
        let start = AxialCoord::new(0, 0);
        let end = AxialCoord::new(3, 0);
        let line = line_to(start, end);
        assert_eq!(line.len(), 4); // 4 hexes including start and end
        assert_eq!(line[0], start);
        assert_eq!(line[line.len() - 1], end);
    }

    #[test]
    fn test_pathfind_no_obstacles() {
        let start = AxialCoord::new(0, 0);
        let target = AxialCoord::new(3, 2);
        let path = pathfind(start, target, |_hex| true);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], target);
    }

    #[test]
    fn test_pathfind_unreachable() {
        let start = AxialCoord::new(0, 0);
        let target = AxialCoord::new(1, 0);
        // Block the target
        let path = pathfind(start, target, |hex| hex != target);
        assert!(path.is_none());
    }

    #[test]
    fn test_pathfind_start_equals_target() {
        let hex = AxialCoord::new(5, 5);
        let path = pathfind(hex, hex, |_| true);
        assert_eq!(path, Some(vec![hex]));
    }

    #[test]
    fn test_field_of_view_no_obstacles() {
        let origin = AxialCoord::new(0, 0);
        let fov = field_of_view(origin, 3, |_hex| false);
        let expected = disk_at(origin, 3);
        // FOV should contain all hexes in disk (no obstacles)
        assert_eq!(fov.len(), expected.len());
    }

    #[test]
    fn test_field_of_view_with_obstacles() {
        let origin = AxialCoord::new(0, 0);
        // Block one specific hex
        let blocked_hex = AxialCoord::new(1, 0);
        let fov = field_of_view(origin, 3, |hex| hex != blocked_hex);
        
        // FOV should have fewer hexes than a disk due to blocking
        let expected = disk_at(origin, 3);
        assert!(fov.len() < expected.len());
        // The blocked hex itself should not be visible
        assert!(!fov.contains(&blocked_hex));
    }

    #[test]
    fn test_line_to_negative_coordinates() {
        let start = AxialCoord::new(-3, -2);
        let end = AxialCoord::new(1, 1);
        let line = line_to(start, end);
        assert_eq!(line[0], start);
        assert_eq!(line[line.len() - 1], end);
        
        // Verify all hexes on the line have appropriate distance
        for (i, hex) in line.iter().enumerate() {
            let dist_from_start = start.distance_to(*hex);
            assert_eq!(dist_from_start, i);
        }
    }

    #[test]
    fn test_line_to_diagonal() {
        let start = AxialCoord::new(0, 0);
        let end = AxialCoord::new(5, 5);
        let line = line_to(start, end);
        assert_eq!(line[0], start);
        assert_eq!(line[line.len() - 1], end);
        
        // Line should not have duplicates after dedup
        let mut sorted_line = line.clone();
        sorted_line.sort_by_key(|c| (c.col, c.row));
        sorted_line.dedup();
        assert_eq!(line.len(), sorted_line.len());
    }

    #[test]
    fn test_disk_at_large_distance() {
        let center = AxialCoord::new(10, 10);
        let disk = disk_at(center, 5);
        
        // Verify all hexes in disk are within correct distance
        for hex in disk.iter() {
            assert!(center.distance_to(*hex) <= 5);
        }
        
        // Verify disk size growth: 1 + 6 + 12 + 18 + 24 + 30
        assert_eq!(disk.len(), 1 + 6 + 12 + 18 + 24 + 30);
    }

    #[test]
    fn test_ring_at_off_center() {
        let center = AxialCoord::new(10, 15);
        let ring = ring_at(center, 3);
        
        // Ring at distance 3 should have 18 hexes (6 * distance)
        assert_eq!(ring.len(), 18);
        
        for hex in ring.iter() {
            assert_eq!(center.distance_to(*hex), 3);
        }
    }

    #[test]
    fn test_pathfind_with_wall() {
        let start = AxialCoord::new(0, 0);
        let target = AxialCoord::new(3, 0);
        
        // Create a "wall" of blocked hexes
        let wall_hex = AxialCoord::new(1, 0);
        let path = pathfind(start, target, |hex| hex != wall_hex);
        
        // Should find alternative path around the wall
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], target);
        // Path should not go through wall
        assert!(!path.contains(&wall_hex));
    }

    #[test]
    fn test_pathfind_start_blocked() {
        let start = AxialCoord::new(0, 0);
        let target = AxialCoord::new(3, 2);
        // Block the start position
        let path = pathfind(start, target, |hex| hex != start);
        assert!(path.is_none());
    }

    #[test]
    fn test_neighbors_at_boundaries() {
        // Test neighbors at various positions
        let corner = AxialCoord::new(0, 0);
        let neighbors_list = neighbors(corner);
        
        // All neighbors should be exactly distance 1 from corner
        for neighbor in neighbors_list {
            assert_eq!(corner.distance_to(neighbor), 1);
        }
        
        // Test neighbors at different location
        let offset = AxialCoord::new(100, 100);
        let neighbors_offset = neighbors(offset);
        for neighbor in neighbors_offset {
            assert_eq!(offset.distance_to(neighbor), 1);
        }
    }

    #[test]
    fn test_line_to_many_steps() {
        let start = AxialCoord::new(0, 0);
        let end = AxialCoord::new(10, -5);
        let distance = start.distance_to(end);
        let line = line_to(start, end);
        
        // Line should have appropriate length
        assert!(line.len() <= distance + 1);
        assert_eq!(line[0], start);
        assert_eq!(line[line.len() - 1], end);
    }

    // ==================== Passability System Integration Tests ====================

    #[test]
    fn test_can_cross_border_within_limit() {
        let from = AxialCoord::new(0, 0);
        let to = AxialCoord::new(1, 0);
        
        // Mock elevation function: from at 10m, to at 12m (2m difference)
        let get_elevation = |hex: AxialCoord| {
            if hex == from { 10.0 } else { 12.0 }
        };
        
        // Max elevation difference is 2.5m
        let can_cross = can_cross_border(from, to, get_elevation, 2.5);
        assert!(can_cross, "Should cross border with 2m difference when limit is 2.5m");
    }

    #[test]
    fn test_can_cross_border_at_limit() {
        let from = AxialCoord::new(0, 0);
        let to = AxialCoord::new(1, 0);
        
        // Exactly at the limit: 2.5m difference
        let get_elevation = |hex: AxialCoord| {
            if hex == from { 10.0 } else { 12.5 }
        };
        
        let can_cross = can_cross_border(from, to, get_elevation, 2.5);
        assert!(can_cross, "Should cross border at exactly the elevation limit");
    }

    #[test]
    fn test_can_cross_border_exceeds_limit() {
        let from = AxialCoord::new(0, 0);
        let to = AxialCoord::new(1, 0);
        
        // Exceeds limit: 3m difference > 2.5m max
        let get_elevation = |hex: AxialCoord| {
            if hex == from { 10.0 } else { 13.0 }
        };
        
        let can_cross = can_cross_border(from, to, get_elevation, 2.5);
        assert!(!can_cross, "Should NOT cross border when elevation difference exceeds limit");
    }

    #[test]
    fn test_can_cross_border_negative_elevation() {
        let from = AxialCoord::new(0, 0);
        let to = AxialCoord::new(1, 0);
        
        // Both negative but within limit
        let get_elevation = |hex: AxialCoord| {
            if hex == from { -5.0 } else { -3.5 }
        };
        
        let can_cross = can_cross_border(from, to, get_elevation, 2.5);
        assert!(can_cross, "Should work with negative elevations");
    }

    #[test]
    fn test_can_enter_from_passable_terrain_and_elevation() {
        let from = AxialCoord::new(0, 0);
        let to = AxialCoord::new(1, 0);
        
        let terrain_map = |hex: AxialCoord| {
            hex == to  // Only `to` is passable
        };
        
        let elevation_map = |hex: AxialCoord| {
            if hex == from { 10.0 } else { 11.0 }
        };
        
        // Should allow entry: terrain is passable and elevation is within limit
        let can_enter = can_enter_from(from, to, terrain_map, elevation_map, 2.5);
        assert!(can_enter, "Should allow entry with passable terrain and acceptable elevation");
    }

    #[test]
    fn test_can_enter_from_impassable_terrain() {
        let from = AxialCoord::new(0, 0);
        let to = AxialCoord::new(1, 0);
        
        let terrain_map = |_hex: AxialCoord| false;  // All terrain impassable
        
        let elevation_map = |hex: AxialCoord| {
            if hex == from { 10.0 } else { 11.0 }
        };
        
        // Should block entry: terrain is impassable (even though elevation is fine)
        let can_enter = can_enter_from(from, to, terrain_map, elevation_map, 2.5);
        assert!(!can_enter, "Should block entry when terrain is impassable");
    }

    #[test]
    fn test_can_enter_from_steep_elevation() {
        let from = AxialCoord::new(0, 0);
        let to = AxialCoord::new(1, 0);
        
        let terrain_map = |_hex: AxialCoord| true;  // All terrain passable
        
        let elevation_map = |hex: AxialCoord| {
            if hex == from { 10.0 } else { 15.0 }  // 5m difference
        };
        
        // Should block entry: elevation difference too great (even though terrain is passable)
        let can_enter = can_enter_from(from, to, terrain_map, elevation_map, 2.5);
        assert!(!can_enter, "Should block entry when elevation difference exceeds limit");
    }

    #[test]
    fn test_can_enter_from_comprehensive() {
        // Simulate a pathfinding scenario with terrain and elevation constraints
        let start = AxialCoord::new(0, 0);
        let _target = AxialCoord::new(5, 5);
        let barrier = AxialCoord::new(2, 2);
        let steep = AxialCoord::new(3, 3);
        
        let terrain_map = |hex: AxialCoord| {
            hex != barrier  // Only barrier is impassable
        };
        
        let elevation_map = |hex: AxialCoord| {
            match hex {
                AxialCoord { col: 3, row: 3 } => 20.0,  // steep hex at high elevation
                _ => 10.0,  // flat terrain elsewhere
            }
        };
        
        // Can enter adjacent flat tile
        let adjacent_flat = AxialCoord::new(1, 0);
        let can_enter_flat = can_enter_from(start, adjacent_flat, terrain_map, elevation_map, 2.5);
        assert!(can_enter_flat, "Should enter flat adjacent tile");
        
        // Cannot enter barrier (even from flat)
        let can_enter_barrier = can_enter_from(
            AxialCoord::new(1, 1),
            barrier,
            terrain_map,
            elevation_map,
            2.5,
        );
        assert!(!can_enter_barrier, "Should not enter impassable barrier");
        
        // Cannot enter steep tile from flat (10m > 2.5m limit)
        let can_enter_steep = can_enter_from(
            AxialCoord::new(2, 2),
            steep,
            terrain_map,
            elevation_map,
            2.5,
        );
        assert!(!can_enter_steep, "Should not enter steep tile with 10m elevation jump");
    }
}

