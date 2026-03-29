use bevy::prelude::*;
#[allow(unused_imports)]
use crate::grid::coordinates::AxialCoord;

/// Tile component for each hex
#[derive(Component, Reflect, Debug, Clone)]
pub struct Tile;

/// Elevation of terrain (meters)
#[derive(Component, Reflect, Debug, Clone)]
pub struct Elevation {
    pub height_meters: f32,
}

impl Default for Elevation {
    fn default() -> Self {
        Self {
            height_meters: 0.0,
        }
    }
}

/// Soil composition
#[derive(Component, Reflect, Debug, Clone)]
pub struct Soil {
    pub depth_meters: f32,
    pub soil_type: SoilType,
    pub organic_matter_kg_m2: f32,
    pub nutrients: Nutrients,
    pub compaction: f32,
    pub microbial_activity: f32,
}

/// Types of soil
#[derive(Debug, Clone, Copy, Reflect, Default, PartialEq, Eq)]
pub enum SoilType {
    #[default]
    Loam,
    Sand,
    Clay,
}

impl SoilType {
    /// Base infiltration rate (m/s)
    #[allow(dead_code)]
    pub fn infiltration_rate(&self) -> f32 {
        match self {
            SoilType::Sand => 0.01,
            SoilType::Loam => 0.0005,
            SoilType::Clay => 0.00001,
        }
    }
}

/// Nutrients in soil (kg/m²)
#[derive(Component, Reflect, Debug, Clone, Copy, Default)]
pub struct Nutrients {
    pub nitrogen: f32,
    pub phosphorus: f32,
    pub potassium: f32,
}

/// Terrain types that determine tile passability and movement costs.
///
/// Some terrain types are completely impassable (Water, Fence), while others
/// are passable. Combined with elevation differences, this determines if a character
/// can move to or through a tile.
#[derive(Debug, Clone, Copy, Reflect, Default, PartialEq, Eq)]
pub enum TerrainType {
    /// Grassland - easily passable
    #[default]
    Grassland,
    /// Forest - passable but slower movement
    Forest,
    /// Mountain - passable but steep, limited movement options
    Mountain,
    /// Water - completely impassable
    Water,
    /// Fence/barrier - completely impassable
    Fence,
    /// Bog/swamp - passable but very slow, consumes stamina
    Bog,
    /// Brambles/thorns - passable but damages and slows movement
    Brambles,
    /// Rock outcrop - completely impassable
    Rock,
}

impl TerrainType {
    /// Returns true if this terrain type blocks movement entirely.
    ///
    /// Impassable terrain cannot be entered by characters.
    #[allow(dead_code)]
    pub fn is_impassable(&self) -> bool {
        matches!(self, TerrainType::Water | TerrainType::Fence | TerrainType::Rock)
    }

    /// Returns the movement cost multiplier (1.0 = normal, >1.0 = slower).
    ///
    /// Used to calculate how much stamina/time it costs to traverse this terrain.
    #[allow(dead_code)]
    pub fn movement_cost(&self) -> f32 {
        match self {
            TerrainType::Grassland => 1.0,
            TerrainType::Forest => 1.5,
            TerrainType::Mountain => 2.0,
            TerrainType::Bog => 3.0,
            TerrainType::Brambles => 1.8,
            // These are impassable so cost is irrelevant, but provide fallback
            TerrainType::Water | TerrainType::Fence | TerrainType::Rock => f32::INFINITY,
        }
    }
}

/// Terrain component for each hex.
///
/// Stores the terrain type and manages passability rules for pathfinding.
#[derive(Component, Reflect, Debug, Clone, Copy)]
pub struct Terrain {
    pub terrain_type: TerrainType,
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            terrain_type: TerrainType::Grassland,
        }
    }
}

/// Configuration for movement and pathfinding rules.
///
/// This resource defines global passability constraints like maximum elevation
/// difference allowed when crossing tile borders.
#[derive(Resource, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct MovementRules {
    /// Maximum elevation difference in meters to cross between tiles.
    /// Borders with height differences exceeding this are impassable.
    /// Typical value: ~2.5 meters (45° slope at 5cm tile size).
    pub max_crossable_elevation_diff: f32,
}

impl Default for MovementRules {
    fn default() -> Self {
        Self {
            // At 5cm per tile, 2.5m difference = 45° slope
            max_crossable_elevation_diff: 2.5,
        }
    }
}


/// Water component
#[derive(Component, Reflect, Debug, Clone)]
pub struct Water {
    pub surface: f32,             // Surface water (meters)
    pub soil_moisture: f32,       // 0.0 to 1.0
    pub groundwater_depth: f32,   // Depth below surface (meters)
    pub temperature_celsius: f32,
    pub sediment_kg_m2: f32,
}

impl Default for Water {
    fn default() -> Self {
        Self {
            surface: 0.0,
            soil_moisture: 0.3,
            groundwater_depth: 2.0,
            temperature_celsius: 20.0,
            sediment_kg_m2: 0.0,
        }
    }
}

/// Vegetation on tile
#[derive(Component, Reflect, Debug, Clone)]
pub struct Plant {
    pub id: u64,
    pub species: PlantSpecies,
    pub age_days: u32,
    pub growth_phase: GrowthPhase,
    pub energy: f32,
    pub roots_depth_meters: f32,
}

#[derive(Debug, Clone, Copy, Reflect, Default, PartialEq, Eq)]
pub enum PlantSpecies {
    #[default]
    Grass,
    Oak,
    Pine,
    Legume,
}

#[derive(Debug, Clone, Copy, Reflect, Default, PartialEq, Eq)]
pub enum GrowthPhase {
    #[default]
    Seed,
    Seedling,
    Juvenile,
    Mature,
    Reproductive,
    Senescent,
}

/// Character/worker
#[derive(Component, Reflect, Debug, Clone)]
pub struct Character {
    pub id: u64,
    pub name: String,
    pub skills: CharacterSkills,
    pub energy: f32,
    pub current_task: Option<u64>,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct CharacterSkills {
    pub digging: f32,
    pub planting: f32,
    pub irrigation: f32,
    pub construction: f32,
}

impl Default for CharacterSkills {
    fn default() -> Self {
        Self {
            digging: 0.5,
            planting: 0.5,
            irrigation: 0.5,
            construction: 0.5,
        }
    }
}

/// Global game state
#[derive(Resource, Reflect, Debug, Clone)]
pub struct GameState {
    pub paused: bool,
    pub game_time: GameTime,
    pub scenario: String,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub struct GameTime {
    pub total_seconds: f32,
    pub day: u32,
    pub month: u8,
    pub year: u16,
    pub time_scale: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            paused: false,
            game_time: GameTime {
                total_seconds: 0.0,
                day: 1,
                month: 1,
                year: 1,
                time_scale: 24.0,
            },
            scenario: "default".into(),
        }
    }
}
