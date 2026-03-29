# Regrowth Technical Design Document

**Version**: 1.0  
**Date**: 29. März 2026  
**Target Audience**: AI code generators + project steering  
**Game Engine**: Bevy (latest stable, currently 0.18+)

---

## Table of Contents

1. [Overview & Architecture](#overview--architecture)
2. [Measurement System](#measurement-system)
3. [Bevy Plugin Architecture](#bevy-plugin-architecture)
4. [Core Components & Data Structures](#core-components--data-structures)
5. [Simulation Systems](#simulation-systems)
6. [Rendering Pipeline](#rendering-pipeline)
7. [Input & UI Systems](#input--ui-systems)
8. [Performance Optimization](#performance-optimization)
9. [File I/O & Save System](#file-io--save-system)

---

## Overview & Architecture

### Design Philosophy

- **Bevy-native**: Use Bevy's ECS patterns throughout; avoid fighting the engine
- **Decoupled simulation**: Core simulation runs independent of rendering/UI
- **Data-driven**: Scenario data, configuration, and balance values in TOML/JSON
- **Incremental updates**: Each system updates a subset of world per frame
- **Real-world grounding**: Measurements and timescales based on real permaculture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Bevy App                              │
├─────────────────────────────────────────────────────────┤
│ Input       │ UI Plugin     │ Rendering   │ Debug       │
├─────────────────────────────────────────────────────────┤
│ Time System │ Game State   │ Scenario    │ Resources   │
├─────────────────────────────────────────────────────────┤
│             Simulation Systems Layer                    │
│  ┌─────────────────────────────────────────────────┐   │
│  │ Water │ Soil │ Plants │ Organisms │ Labor │ UI │   │
│  └─────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────┤
│ Core Storage: Grid (Tiles) + Entities (Characters)    │
└─────────────────────────────────────────────────────────┘
```

### Execution Order (Per Frame)

1. **Input System**: Capture keyboard/mouse, queue actions
2. **Time System**: Advance in-game time (if not paused)
3. **Water System**: Calculate water flow, infiltration
4. **Soil System**: Decomposition, nutrient cycling
5. **Plant System**: Growth, reproduction
6. **Organism System**: Fauna behavior, interactions
7. **Labor System**: Character task execution, terraforming effects
8. **Rendering System**: Update camera, render tiles + entities
9. **UI System**: Update panels, tooltips

---

## Measurement System

### Real-World Scale

**Hexagon specifications**:
- **Outer radius (circumradius)**: 5 cm
- **Inner radius (inradius)**: 5 × √3/2 ≈ 4.33 cm
- **Width (point-to-point)**: 10 cm
- **Height (flat-to-flat)**: 5√3 ≈ 8.66 cm
- **Area**: (3√3/2) × 5² ≈ 64.95 cm²
- **Real-world area per tile**: ~0.0065 m² (approximately small permaculture cell)

### Game Units Conversion

```
Physical            → Game Unit              → Screen Pixels
─────────────────────────────────────────────────────────────
1 meter             → 1.0 game unit          → X pixels
1 centimeter        → 0.01 game units        → X/100 pixels
5 cm hexagon        → 0.05 game units        → SPRITE_SCALE pixels
Soil depth (30cm)   → 0.30 game units        → vertical sim data
Groundwater (2m)    → 2.0 game units         → sim depth

Game Unit → Pixel Conversion:
  PIXELS_PER_GAME_UNIT = 200  (adjustable for zoom/resolution)
  HEX_CIRCUMRADIUS_PX = 100   (5cm hexagon @ standard zoom)
  TILE_SCREEN_WIDTH = 200px   (10cm @ standard zoom)
```

### Time Scale

If the simulation is sufficiently stable, allow for higher simulation speeds, but slow down, when needed, e.g. to Simulate pooling of rain, proppery, or a new waterway is being used, etc. 

```
Real Time           → Simulation Time
─────────────────────────────────────
1 second            → 1 minute (configurable)
1 minute            → 1 hour (game time)
1 hour (real)       → ~24 hours (game)
1 day (real)        → ~1 season (game)

Typical scenario:   30-120 minutes real time = 1-2 seasons game time
```

### Environmental Parameters

**Temperature scale** (Celsius, real-world):
```
-10°C to +40°C → Game internal representation
Affects: Decomposition, evaporation, plant growth, water phase changes
```

**Precipitation** (mm/day, real-world):
```
0-50mm/day typical rainfall → Seasonal variation
Dry season: 0mm/day
Wet season: 20-50mm/day
Events: extreme weather ~100mm/day
```

**Soil nutrients** (kg/m²):
```
N (Nitrogen):   0-2.0 kg/m²
P (Phosphorus): 0-0.5 kg/m²
K (Potassium):  0-1.5 kg/m²

Poor soil:   N<0.1, P<0.05, K<0.1
Good soil:   N>0.5, P>0.1, K>0.3
Rich soil:   N>1.0, P>0.2, K>0.6
```

---

## Bevy Plugin Architecture

### Plugin Structure

```rust
// src/main.rs - Bevy App Setup
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(/* window config */))
        
        // Custom plugins in order
        .add_plugins((
            TimePlugin,               // Simulation time control
            GridPlugin,               // Hexagonal grid, tile management
            ScenarioPlugin,           // Load/manage scenarios
            
            // Simulation systems (run in order)
            WaterSimulationPlugin,    // Water flow, infiltration
            SoilSimulationPlugin,     // Nutrient cycling, decomposition
            PlantSimulationPlugin,    // Growth, reproduction
            OrganismSimulationPlugin, // Fauna, microbes
            LaborPlugin,              // Character tasks, terraforming
            
            // Rendering & UI
            RenderingPlugin,          // Hexagon rendering pipeline
            UIPlugin,                 // Status panels, menus
            InputPlugin,              // Keyboard/mouse handling
            
            #[cfg(debug_assertions)]
            DebugPlugin,              // Development tools
        ))
        .run();
}
```

### Plugin Template

```rust
// Each plugin follows this pattern:
pub struct WaterSimulationPlugin;

impl Plugin for WaterSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Water>()  // For reflection
            .init_resource::<WaterConfig>()
            .add_systems(
                Update,
                (
                    update_water_flow.run_if(not(game_paused)),
                    update_infiltration,
                    update_evapotranspiration,
                    update_water_transport,
                )
                    .chain()
                    .after(TimeSystem),
            );
    }
}
```

### Resource Management

Key resources managed by Bevy:

```rust
// Global state
pub struct GameState {
    pub paused: bool,
    pub game_time: GameTime,
    pub scenario: Scenario,
}

pub struct GameTime {
    pub total_seconds: f32,
    pub day: u32,
    pub month: u8,
    pub year: u16,
    pub time_scale: f32,  // 1.0 = 1:1 real-to-game, 24.0 = 1 day per second
}

// Configuration
pub struct WaterConfig {
    pub infiltration_base: HashMap<SoilType, f32>,
    pub evaporation_rate: f32,
    pub flow_viscosity: f32,
}

pub struct SoilConfig {
    pub decomposition_rates: HashMap<String, f32>,
    pub nutrient_ratios: Nutrients,
}
```

---

## Core Components & Data Structures

### Tile Entity & Components

Each hexagon tile is an entity with components:

```rust
#[derive(Component, Reflect, Debug, Clone)]
pub struct Tile {
    pub position: HexPos,  // Axial coordinates
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct Elevation {
    pub height_meters: f32,  // Surface elevation (meters)
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct Soil {
    pub depth: f32,  // Soil depth (meters)
    pub soil_type: SoilType,
    pub organic_matter: f32,  // kg/m²
    pub nutrients: Nutrients,
    pub compaction: f32,  // 0.0 (loose) to 1.0 (hard)
    pub microbial_activity: f32,  // 0.0 to 1.0
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct Nutrients {
    pub nitrogen: f32,      // kg/m²
    pub phosphorus: f32,
    pub potassium: f32,
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct Water {
    pub surface: f32,           // Surface water (meters)
    pub soil_moisture: f32,     // Fraction (0.0 to 1.0)
    pub groundwater_depth: f32, // Depth below surface (meters)
    pub temperature: f32,       // Celsius
    pub sediment: f32,          // kg/m²
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum SoilType {
    Sand,
    Loam,
    Clay,
    Custom { name: String, infiltration: f32 },
}
```

### Vegetation & Organisms

```rust
#[derive(Component, Reflect, Debug, Clone)]
pub struct Plant {
    pub id: u64,
    pub species: PlantSpecies,
    pub age_days: u32,
    pub growth_phase: GrowthPhase,
    pub energy: f32,
    pub roots_depth: f32,
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum PlantSpecies {
    Oak,
    Pine,
    Grass,
    Legume,
    Wheat,
    Custom(String),
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum GrowthPhase {
    Seed,
    Seedling,
    Juvenile,
    Mature,
    Reproductive,
    Senescent,
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct Organism {
    pub node_type: OrganismType,
    pub energy: f32,
    pub population: u32,  // For microbes/insects
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum OrganismType {
    Pollinator,
    SoilFauna,
    Microbe,
    Pest,
}
```

### Character & Labor

```rust
#[derive(Component, Reflect, Debug, Clone)]
pub struct Character {
    pub id: u64,
    pub name: String,
    pub skills: CharacterSkills,
    pub energy: f32,  // 0.0 to 100.0
    pub current_task: Option<u64>,  // Task entity ID
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct CharacterSkills {
    pub digging: f32,         // 0.0 to 1.0
    pub planting: f32,
    pub irrigation: f32,
    pub construction: f32,
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub task_type: TaskType,
    pub location: HexPos,
    pub required_skill: CharacterSkill,
    pub work_required: f32,  // Total work units
    pub progress: f32,       // Current progress
    pub status: TaskStatus,
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum TaskType {
    DigSwale { depth: f32 },
    BuildTerrace { width: f32 },
    PlantTree { species: PlantSpecies },
    ApplyMulch { amount: f32 },
    IrrigateLand { rate: f32 },
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum TaskStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
}
```

### Structures

```rust
#[derive(Component, Reflect, Debug, Clone)]
pub struct Structure {
    pub id: u64,
    pub structure_type: StructureType,
    pub location: HexPos,
    pub health: f32,  // % condition
    pub construction_progress: f32,  // 0.0 to 1.0
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum StructureType {
    Swale { depth: f32, length: f32 },
    Terrace { width: f32, gradient: f32 },
    WaterTank { capacity_liters: f32, current_liters: f32 },
    Pond { volume_m3: f32 },
    Guild { plants: Vec<PlantSpecies> },
}
```

### Hexagon Position System

```rust
#[derive(Component, Reflect, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HexPos {
    pub q: i32,
    pub r: i32,
    // s = -q - r (derived, not stored)
}

impl HexPos {
    pub fn s(&self) -> i32 {
        -self.q - self.r
    }
    
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
    
    pub fn distance(&self, other: &HexPos) -> usize {
        ((self.q.abs_diff(other.q) as usize
            + self.r.abs_diff(other.r) as usize
            + self.s().abs_diff(other.s()) as usize)
            / 2)
    }
}

// Convert hex coordinates to world position
pub fn hex_to_world(hex: HexPos, hex_size: f32) -> Vec2 {
    let q = hex.q as f32;
    let r = hex.r as f32;
    let x = hex_size * (3.0 / 2.0 * q);
    let y = hex_size * (f32::sqrt(3.0) / 2.0 * q + f32::sqrt(3.0) * r);
    Vec2::new(x, y)
}
```

---

## Simulation Systems

### Water Flow System

**Update frequency**: Every frame (scaled by time_scale)  
**Complexity**: O(n) for n tiles

```rust
pub fn update_water_flow(
    mut query: Query<(&HexPos, &mut Water, &Soil, &Elevation)>,
    grid: Res<Grid>,
    mut commands: Commands,
) {
    let mut transfer_map: HashMap<HexPos, f32> = HashMap::new();

    // Stage 1: Calculate infiltration
    for (hex_pos, mut water, soil, _) in query.iter_mut() {
        let infiltration_rate = calculate_infiltration_rate(&soil);
        let infiltrated = (water.surface * infiltration_rate * TIME_DELTA).min(water.surface);
        
        water.surface -= infiltrated;
        water.soil_moisture = (water.soil_moisture + infiltrated / soil.depth).min(1.0);
        water.groundwater_depth = (water.groundwater_depth - infiltrated).max(0.0);
    }

    // Stage 2: Surface flow to downslope neighbors
    for (hex_pos, water, _, elevation) in query.iter() {
        if water.surface > 0.001 {  // Minimum flow threshold
            let downslope_neighbors = grid.downslope_neighbors(*hex_pos);
            for neighbor_pos in downslope_neighbors {
                let neighbor_elevation = query
                    .iter()
                    .find(|(h, _, _, e)| h == &neighbor_pos)
                    .map(|(_, _, _, e)| e.height_meters)
                    .unwrap_or(elevation.height_meters);
                
                let flow_amount = calculate_flow(
                    water.surface,
                    elevation.height_meters,
                    neighbor_elevation,
                );
                transfer_map
                    .entry(neighbor_pos)
                    .and_modify(|f| *f += flow_amount)
                    .or_insert(flow_amount);
            }
        }
    }

    // Stage 3: Apply transfers
    for (hex_pos, mut water, _, _) in query.iter_mut() {
        if let Some(incoming) = transfer_map.get(&hex_pos) {
            water.surface += incoming;
        }
    }

    // Stage 4: Evapotranspiration
    for (_, mut water, _, _) in query.iter_mut() {
        let evaporation = calculate_evaporation(water.temperature, water.soil_moisture);
        water.soil_moisture = (water.soil_moisture - evaporation * TIME_DELTA).max(0.0);
    }
}

fn calculate_infiltration_rate(soil: &Soil) -> f32 {
    let base_rate = match soil.soil_type {
        SoilType::Sand => 0.01,   // 1cm/s (fast)
        SoilType::Loam => 0.0005, // (moderate)
        SoilType::Clay => 0.00001, // (slow)
        _ => 0.0005,
    };
    
    // Compaction reduces infiltration
    let compaction_factor = 1.0 - soil.compaction;
    // Moisture reduces infiltration (saturation)
    let moisture_factor = 1.0 - soil.soil_moisture.powi(2);
    
    base_rate * compaction_factor * moisture_factor
}

fn calculate_flow(surface_water: f32, elevation_from: f32, elevation_to: f32) -> f32 {
    if elevation_to < elevation_from {
        // Flow based on gradient and available water
        let gradient = (elevation_from - elevation_to).abs();
        surface_water * (gradient / 10.0).min(1.0) * 0.5
    } else {
        0.0
    }
}
```

### Soil Nutrient Cycling System

**Update frequency**: Every frame (longer dt than water)

```rust
pub fn update_soil_nutrients(
    mut query: Query<(&mut Soil, &Water)>,
    game_time: Res<GameTime>,
) {
    for (mut soil, water) in query.iter_mut() {
        // Decomposition
        let decomposition_rate = calculate_decomposition_rate(
            water.soil_moisture,
            water.temperature,
            soil.microbial_activity,
        );
        let decomposed = soil.organic_matter * decomposition_rate * TIME_DELTA;
        soil.organic_matter -= decomposed;
        
        // Release nutrients from decomposition
        soil.nutrients.nitrogen += decomposed * NITROGEN_RATIO;
        soil.nutrients.phosphorus += decomposed * PHOSPHORUS_RATIO;
        soil.nutrients.potassium += decomposed * POTASSIUM_RATIO;
        
        // Microbial activity depends on organic matter and moisture
        let activity_input = (soil.organic_matter / 100.0).min(1.0) * water.soil_moisture;
        soil.microbial_activity = soil.microbial_activity * 0.95 + activity_input * 0.05;
    }
}

fn calculate_decomposition_rate(moisture: f32, temp: f32, microbial: f32) -> f32 {
    // Base rate 0.001 per frame
    let base = 0.001;
    
    // Moisture effect (quadratic, peaks at 0.6)
    let moisture_factor = 1.0 - (moisture - 0.6).powi(2);
    
    // Temperature (Q10 model: rate doubles every 10°C)
    let ref_temp = 20.0;
    let temp_factor = 2.0_f32.powf((temp - ref_temp) / 10.0);
    
    // Microbial activity multiplier
    base * moisture_factor * temp_factor * (1.0 + microbial)
}
```

### Plant Growth System

```rust
pub fn update_plant_growth(
    mut query: Query<(&mut Plant, &mut Soil, &Water)>,
    game_time: Res<GameTime>,
) {
    for (mut plant, mut soil, water) in query.iter_mut() {
        if plant.energy <= 0.0 {
            // Plant dies (mark for removal in separate cleanup system)
            continue;
        }

        let nutrient_avg = (soil.nutrients.nitrogen + soil.nutrients.phosphorus + soil.nutrients.potassium) / 3.0;
        let spec = plant::species_data(plant.species);
        
        // Check growth conditions
        let moisture_ok = water.soil_moisture >= spec.min_moisture;
        let nutrients_ok = nutrient_avg >= spec.min_nutrients;
        
        if !moisture_ok || !nutrients_ok {
            plant.energy -= spec.stress_cost * TIME_DELTA;
        }
        
        // Growth
        if plant.energy > spec.growth_cost(plant.growth_phase) {
            plant.age_days += 1;
            plant.energy -= spec.growth_cost(plant.growth_phase);
            
            // Advance growth phase
            if plant.age_days > spec.maturity_age {
                plant.growth_phase = GrowthPhase::Mature;
            }
            
            // Nutrient uptake
            let uptake = spec.nutrient_requirement(plant.growth_phase);
            soil.nutrients.nitrogen -= uptake * 0.5;
            soil.nutrients.phosphorus -= uptake * 0.1;
            soil.nutrients.potassium -= uptake * 0.3;
        }
        
        // Photosynthesis (gain energy from moisture + light)
        if moisture_ok && water.surface >= 0.01 {  // Rough light proxy
            plant.energy += spec.photosynthesis_rate * TIME_DELTA;
        }
    }
}
```

### Labor & Terraforming System

```rust
pub fn execute_labor_tasks(
    mut task_query: Query<&mut Task>,
    mut character_query: Query<&mut Character>,
    mut soil_query: Query<(&HexPos, &mut Soil, &mut Elevation)>,
    grid: Res<Grid>,
) {
    for mut task in task_query.iter_mut() {
        if task.status != TaskStatus::InProgress {
            continue;
        }

        // Find assigned character
        if let Some(character) = character_query.iter_mut().find(|c| c.current_task == Some(task.id)) {
            let work_rate = match &task.task_type {
                TaskType::DigSwale { .. } => character.skills.digging * 10.0,
                TaskType::PlantTree { .. } => character.skills.planting * 8.0,
                TaskType::ApplyMulch { .. } => 5.0,  // Base rate
                _ => 5.0,
            };

            task.progress += work_rate * character.energy / 100.0 * TIME_DELTA;
            character.energy = (character.energy - 2.0 * TIME_DELTA).max(0.0);

            if task.progress >= task.work_required {
                task.status = TaskStatus::Completed;
                apply_task_effects(&mut task, &mut soil_query, &grid);
            }
        }
    }
}

fn apply_task_effects(
    task: &Task,
    soil_query: &mut Query<(&HexPos, &mut Soil, &mut Elevation)>,
    grid: &Grid,
) {
    match &task.task_type {
        TaskType::DigSwale { depth } => {
            // Lower elevation, compact soil
            if let Ok((_, mut soil, mut elevation)) = soil_query.get_mut_single() {
                elevation.height_meters -= depth * 0.1;
                soil.compaction = (soil.compaction + 0.2).min(1.0);
            }
        },
        TaskType::ApplyMulch { amount } => {
            // Increase organic matter, reduce compaction
            if let Ok((_, mut soil, _)) = soil_query.get_mut_single() {
                soil.organic_matter += amount;
                soil.compaction = (soil.compaction - 0.1).max(0.0);
            }
        },
        _ => {}
    }
}
```

---

## Rendering Pipeline

### Bevy Rendering Setup

```rust
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_tile_visuals.after(all_simulation),
            update_entity_visuals.after(all_simulation),
            camera_follow_player,
        ));
    }
}

const HEX_CIRCUMRADIUS: f32 = 5.0; // cm, real-world
const PIXELS_PER_GAME_UNIT: f32 = 200.0;
const HEX_RADIUS_PIXELS: f32 = HEX_CIRCUMRADIUS / 100.0 * PIXELS_PER_GAME_UNIT; // ~100px

pub fn update_tile_visuals(
    mut query: Query<
        (
            &HexPos,
            &Soil,
            &Water,
            &Elevation,
            &mut Sprite,
            &mut Transform,
        ),
        Changed<Soil> | Changed<Water>,
    >,
) {
    for (hex_pos, soil, water, elevation, mut sprite, mut transform) in query.iter_mut() {
        // Calculate world position
        let world_pos = hex_to_world(*hex_pos, HEX_RADIUS_PIXELS);
        transform.translation = Vec3::new(world_pos.x, world_pos.y, elevation.height_meters);

        // Determine sprite texture based on state
        let sprite_path = determine_tile_sprite(soil, water, elevation);
        sprite.image = asset_server.load(sprite_path);

        // Tint color based on moisture, nutrients, etc.
        let color = calculate_tile_color(soil, water);
        sprite.color = color;
    }
}

fn determine_tile_sprite(soil: &Soil, water: &Water, elevation: &Elevation) -> String {
    if water.surface > 0.1 {
        "sprites/hex_water.png".to_string()
    } else if soil.organic_matter > 50.0 {
        "sprites/hex_rich_soil.png".to_string()
    } else if soil.organic_matter > 20.0 {
        "sprites/hex_good_soil.png".to_string()
    } else {
        "sprites/hex_poor_soil.png".to_string()
    }
}

fn calculate_tile_color(soil: &Soil, water: &Water) -> Color {
    let nutrient_score = (soil.nutrients.nitrogen + soil.nutrients.phosphorus + soil.nutrients.potassium) / 3.0;
    
    // Color gradient: brown (poor) → dark brown (good) → black (rich)
    let color_value = (nutrient_score / 1.0).min(1.0);
    
    Color::rgb(0.5 - color_value * 0.3, 0.4 - color_value * 0.2, 0.3 - color_value * 0.1)
}
```

### Isometric Projection

```rust
// For 3D-like effect, use y-sorts and depth z-ordering
pub fn isometric_transform_system(
    mut query: Query<(&HexPos, &mut Transform)>,
) {
    for (hex_pos, mut transform) in query.iter_mut() {
        // z increases with y (screen) and decreases with elevation (to preserve depth)
        let screen_y = transform.translation.y;
        let elevation_z = transform.translation.z;
        let z_order = screen_y * 0.1 - elevation_z * 0.001;
        transform.translation.z = z_order;
    }
}
```

---

## Input & UI Systems

### Input Handling

```rust
pub fn input_system(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut task_commands: Commands,
) {
    // Toggle pause
    if keyboard.just_pressed(KeyCode::Space) {
        game_state.paused = !game_state.paused;
    }

    // Speed controls
    if keyboard.pressed(KeyCode::Plus) {
        game_state.game_time.time_scale *= 1.01;
    }
    if keyboard.pressed(KeyCode::Minus) {
        game_state.game_time.time_scale *= 0.99;
    }

    // Task assignment (on click)
    if mouse.just_pressed(MouseButton::Left) {
        let click_pos = /* get_screen_click_position */;
        let hex_clicked = screen_to_hex(click_pos);
        // Queue UI for task assignment
    }
}
```

### UI Panels

```rust
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            status_panel_system,
            goal_tracker_system,
            character_panel_system,
        ));
    }
}

pub fn status_panel_system(
    game_state: Res<GameState>,
    soil_query: Query<&Soil>,
    water_query: Query<&Water>,
    mut query: Query<&mut Text, With<StatusPanel>>,
) {
    let avg_moisture: f32 = water_query.iter().map(|w| w.soil_moisture).sum::<f32>() / water_query.iter().len() as f32;
    let avg_nutrients: f32 = soil_query
        .iter()
        .map(|s| (s.nutrients.nitrogen + s.nutrients.phosphorus + s.nutrients.potassium) / 3.0)
        .sum::<f32>() / soil_query.iter().len() as f32;

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "Day: {} | Moisture: {:.1}% | Nutrients: {:.2} kg/m²",
            game_state.game_time.day,
            avg_moisture * 100.0,
            avg_nutrients
        );
    }
}
```

---

## Performance Optimization

### Spatial Partitioning

The hexagonal grid naturally provides spatial locality:

```rust
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Entity>,  // Stored linearly for cache efficiency
}

impl Grid {
    pub fn get_tile_index(&self, hex_pos: HexPos) -> Option<usize> {
        // Convert axial to linear index
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = Entity> {
        self.tiles.iter().copied()
    }

    pub fn iter_radius(&self, center: HexPos, radius: usize) -> impl Iterator<Item = Entity> {
        // Return only tiles within radius (for water flow, etc.)
    }
}
```

### System Scheduling

```rust
// Only run simulation if not paused
pub fn run_if_not_paused(game_state: Res<GameState>) -> bool {
    !game_state.paused
}

// In plugin:
app.add_systems(
    Update,
    water_simulation_system
        .run_if(run_if_not_paused)
        .run_if(resource_changed::<GameTime>),
);
```

---

## File I/O & Save System

### Serialization Format

```toml
# save_001.regrowth (TOML format)

[game_state]
paused = false
total_seconds = 86400.0
day = 10
month = 3
year = 1
time_scale = 24.0
scenario = "great_green_wall"

[[tiles]]
q = 0
r = 0
elevation_m = 5.2
soil_type = "loam"
soil_depth = 0.3
organic_matter = 25.5
nitrogen = 0.8
phosphorus = 0.15
potassium = 0.5
compaction = 0.3

[[characters]]
id = 1001
name = "Alice"
skills.digging = 0.7
skills.planting = 0.5
energy = 45.0
```

### Save/Load System

```rust
pub fn save_game(
    mut commands: Commands,
    game_state: Res<GameState>,
    tile_query: Query<(&HexPos, &Soil, &Water, &Elevation)>,
) {
    let save_path = format!("saves/save_{}.regrowth", game_state.scenario);
    
    let mut document = toml::Document::new();
    
    // Serialize game state
    document["game_state"]["paused"] = game_state.paused.into();
    document["game_state"]["total_seconds"] = game_state.game_time.total_seconds.into();
    
    // Serialize tiles
    for (hex_pos, soil, water, elevation) in tile_query.iter() {
        // ... build TOML representation
    }
    
    fs::write(save_path, document.to_string()).expect("Failed to save game");
}
```

---

## Development Priorities

### Phase 1: Foundations (Week 1-2)
- [ ] Bevy project setup with plugin structure
- [ ] Hexagonal grid generation and coordinate system
- [ ] Basic rendering (hex sprites, isometric view)
- [ ] Water flow system (core algorithm)

### Phase 2: Core Simulation (Week 3-4)
- [ ] Soil simulation (nutrient cycling, decomposition)
- [ ] Plant growth system
- [ ] Character and labor management
- [ ] Basic UI (status panel, pause/play)

### Phase 3: Polish & Scenarios (Week 5+)
- [ ] Multiple scenarios with loading
- [ ] Advanced rendering (animations, effects)
- [ ] Save/load system
- [ ] Balance and tuning
- [ ] Testing and optimization

---

## References & Notes

**Bevy Documentation**: https://bevyengine.org/  
**Hexagonal Grid Reference**: https://www.redblobgames.com/grids/hexagons/  
**Permaculture Design**: Integration based on real restoration techniques

**Metric Conversions** (for reference):
- 5 cm = 1.97 inches
- 1 meter = 3.28 feet
- 1 kg/m² = 0.102 oz/ft²

---

**Document Version**: 1.0 (Ready for implementation)
