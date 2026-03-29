use bevy::prelude::*;

/// Hexagonal position using axial coordinates
#[derive(Component, Reflect, Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct HexPos {
    pub q: i32,
    pub r: i32,
}

impl HexPos {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Derived axial coordinate
    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    /// Return the 6 neighbors in axial coordinates
    pub fn neighbors(&self) -> [HexPos; 6] {
        [
            HexPos { q: self.q + 1, r: self.r },
            HexPos { q: self.q + 1, r: self.r - 1 },
            HexPos { q: self.q, r: self.r - 1 },
            HexPos { q: self.q - 1, r: self.r },
            HexPos { q: self.q - 1, r: self.r + 1 },
            HexPos { q: self.q, r: self.r + 1 },
        ]
    }

    /// Manhattan distance between two hex positions
    pub fn distance(&self, other: &HexPos) -> usize {
        (self.q.abs_diff(other.q) as usize
            + self.r.abs_diff(other.r) as usize
            + self.s().abs_diff(other.s()) as usize)
            / 2
    }
}

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
