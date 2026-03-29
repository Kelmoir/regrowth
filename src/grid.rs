use crate::components::HexPos;

/// Hexagonal grid manager
pub struct Grid {
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Convert hex position to world coordinates
    pub fn hex_to_world(&self, hex: HexPos, hex_size: f32) -> (f32, f32) {
        let q = hex.q as f32;
        let r = hex.r as f32;
        let x = hex_size * (3.0 / 2.0 * q);
        let y = hex_size * (2.0_f32.sqrt() / 2.0 * q + 2.0_f32.sqrt() * r);
        (x, y)
    }

    /// Convert world coordinates back to hex
    pub fn world_to_hex(&self, x: f32, y: f32, hex_size: f32) -> HexPos {
        let q = (x / hex_size * (2.0 / 3.0)).round();
        let r = (y / hex_size * (-1.0 / 3.0) + (x / hex_size) * (1.0 / 3.0)).round();
        HexPos::new(q as i32, r as i32)
    }

    /// Get downslope neighbors (for water flow)
    pub fn downslope_neighbors(&self, _hex: HexPos) -> Vec<HexPos> {
        // TODO: Implement based on elevation data
        vec![]
    }

    /// Check if hex is within grid bounds (basic check)
    pub fn is_valid(&self, hex: HexPos) -> bool {
        hex.q.abs() < self.width as i32 / 2 && hex.r.abs() < self.height as i32 / 2
    }
}
