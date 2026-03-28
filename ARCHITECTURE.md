# Regrowth Architecture

This document describes the technical architecture of Regrowth, including core data structures, simulation systems, and design patterns.

## Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Core Data Structures](#core-data-structures)
4. [Simulation Systems](#simulation-systems)
5. [Module Organization](#module-organization)
6. [Entity-Component-System (ECS)](#entity-component-system-ecs)
7. [Rendering Pipeline](#rendering-pipeline)
8. [Performance Considerations](#performance-considerations)

---

## Overview

Regrowth's architecture is built around:

- **Hexagonal grid**: Efficient spatial partitioning for terrain and simulation
- **Vertical tile profiles**: Subsurface storage for soil, water, and organisms
- **Modular simulation systems**: Soil, water, plants, organisms, and labor systems operate semi-independently
- **Data-driven design**: Configuration and scenario data separate from code
- **ECS patterns**: For dynamic entities like characters and structures

### Design Principles

1. **Separation of concerns**: Simulation logic ≠ rendering ≠ UI
2. **Data locality**: Tile-based data enables efficient spatial iteration
3. **Incremental updates**: Each frame updates one simulation layer (water, then soil, then organisms)
4. **Extensibility**: New mechanics can be added as new simulation layers

---

## Project Structure

```
regrowth/
├── src/
│   ├── main.rs                    # Entry point; initializes game state
│   │
│   ├── simulation/                # Core simulation systems
│   │   ├── mod.rs                 # Re-exports all subsystems
│   │   ├── grid.rs                # Hexagonal grid data structure
│   │   ├── tile.rs                # Tile (cell) definition
│   │   │
│   │   ├── soil.rs                # Soil composition and properties
│   │   ├── soil_nutrient.rs        # Nutrient cycling simulation
│   │   ├── soil_structure.rs       # Soil structure and compaction
│   │   │
│   │   ├── water.rs               # Water state and properties
│   │   ├── water_flow.rs          # Water flow and infiltration
│   │   ├── water_transport.rs     # Sediment/nutrient transport
│   │   │
│   │   ├── plant.rs               # Plant growth and lifecycle
│   │   ├── organism.rs            # Generic organism (fauna, microbes)
│   │   ├── ecosystem.rs           # Biodiversity and interactions
│   │   │
│   │   └── labor.rs               # Character labor and tasks
│   │
│   ├── entities/                  # Dynamic game objects (ECS)
│   │   ├── mod.rs                 # Entity registry and component definitions
│   │   ├── character.rs           # Character data structure and behavior
│   │   ├── structure.rs           # Buildings (swales, terraces, tanks, etc.)
│   │   └── components.rs          # Component definitions for ECS
│   │
│   ├── ui/                        # User interface and rendering
│   │   ├── mod.rs                 # UI system coordinator
│   │   ├── renderer.rs            # Hexagon rendering, isometric projection
│   │   ├── input.rs               # Keyboard/mouse input handling
│   │   └── panels/                # UI panels (status, menu, goals, etc.)
│   │
│   ├── scenarios/                 # Scenario definitions and data
│   │   ├── mod.rs                 # Scenario loader
│   │   ├── great_green_wall.rs    # Example scenario
│   │   ├── family_farm.rs         # Example scenario
│   │   └── scenarios.toml         # Scenario metadata and configuration
│   │
│   ├── game.rs                    # Main game state and update loop
│   ├── config.rs                  # Configuration constants
│   └── util/                      # Helpers and utilities
│       ├── mod.rs
│       ├── math.rs                # Vector math, distance calculations
│       ├── random.rs              # RNG for stochastic events
│       └── serialization.rs       # Save/load game state
│
├── tests/                         # Integration tests
│   ├── soil_water_interaction.rs  # Multi-system test
│   ├── full_game_loop.rs          # Complete simulation scenario
│   └── scaling.rs                 # Performance/scaling tests
│
├── benches/                       # Performance benchmarks
│   ├── water_flow.rs              # Water flow algorithm benchmarks
│   ├── soil_update.rs             # Soil system benchmarks
│   └── full_simulation.rs         # End-to-end simulation benchmarks
│
├── assets/                        # Game data and resources
│   ├── scenarios/                 # Scenario TOML files
│   ├── sprites/                   # Hexagon sprites, terrain textures
│   └── data/                      # Game balancing constants
│
├── Cargo.toml                     # Rust dependencies
├── README.md                      # Game overview and user guide
├── CONTRIBUTING.md                # Contribution guidelines
├── ARCHITECTURE.md                # This file
├── CODE_OF_CONDUCT.md             # Community standards
└── LICENSE                        # MIT License
```

---

## Core Data Structures

### Hexagonal Grid

The game world is represented as a hexagonal grid. Hexagons are superior to squares for simulation because:

- **6 neighbors**: More natural flow patterns (water, migration)
- **Fewer distortions**: Better for line-of-sight and diagonal distance
- **Natural stacking**: Hexagons tessellate perfectly

#### Data Structure

```rust
pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    /// Get tile at axial coordinates (q, r) in cubic coordinate system
    pub fn get_tile(&self, q: i32, r: i32) -> Option<&Tile> { }
    
    /// Get mutable tile
    pub fn get_tile_mut(&mut self, q: i32, r: i32) -> Option<&mut Tile> { }
    
    /// Get neighbors in all 6 directions
    pub fn neighbors(&self, q: i32, r: i32) -> Vec<(i32, i32)> { }
    
    /// Calculate distance between tiles (in steps)
    pub fn distance(&self, from: (i32, i32), to: (i32, i32)) -> usize { }
}
```

#### Coordinate System

We use **axial coordinates** (q, r) with **cube coordinates** (q, r, s where q+r+s=0) validation:

- **q (column)**: East-West axis
- **r (row)**: Southwest-Northeast axis
- **s (vertical)**: Northwest-Southeast axis (not stored, derived as -q-r)

**Neighbor offsets** (in axial):
```
NE: (+1, 0)    NW: (0, -1)
E:  (+1, -1)   W:  (-1, +1)
SE: (0, +1)    SW: (-1, 0)
```

### Tile (Cell)

Each hexagon contains vertical "layers" representing what's in that location:

```rust
pub struct Tile {
    /// Position in axial coordinates
    position: (i32, i32),
    
    /// Terrain elevation (in meters, affects water flow)
    elevation: f32,
    
    /// Soil subsystem
    soil: Soil,
    
    /// Water subsystem
    water: Water,
    
    /// Vegetation and surface organisms
    plants: Vec<Plant>,
    
    /// Structures built on this tile (swales, terraces, etc.)
    structures: Vec<Structure>,
    
    /// Flag: has this tile been visited/modified by player?
    explored: bool,
}
```

### Soil

Soil tracks composition and properties across a vertical profile:

```rust
pub struct Soil {
    /// Depth of soil layer (in meters)
    depth: f32,
    
    /// Soil type classification
    soil_type: SoilType,  // Sand, Loam, Clay
    
    /// Organic matter content (kg/m²)
    organic_matter: f32,
    
    /// Nutrient tracking (N, P, K in kg/m²)
    nutrients: Nutrients { n: f32, p: f32, k: f32 },
    
    /// Microbial biomass and diversity index
    microbial_activity: f32,  // 0.0 to 1.0
    
    /// Soil compaction level (affects infiltration)
    compaction: f32,  // 0.0 (loose) to 1.0 (hard pan)
}

pub enum SoilType {
    Sand,     // Fast infiltration, low retention
    Loam,     // Balanced infiltration and retention
    Clay,     // Slow infiltration, high retention
}

pub struct Nutrients {
    n: f32,  // Nitrogen
    p: f32,  // Phosphorus
    k: f32,  // Potassium
}
```

### Water

Water state per tile:

```rust
pub struct Water {
    /// Surface water (m, above ground)
    surface: f32,
    
    /// Soil moisture (as fraction, 0.0 to 1.0)
    soil_moisture: f32,
    
    /// Groundwater depth (m below surface)
    groundwater_depth: f32,
    
    /// Suspended sediment (kg/m²)
    sediment: f32,
    
    /// Temperature (°C)
    temperature: f32,
}
```

### Plant

Individual plants with growth lifecycles:

```rust
pub struct Plant {
    id: u64,
    species: PlantSpecies,
    age_days: u32,
    growth_phase: GrowthPhase,  // Seedling, Juvenile, Mature
    energy: f32,  // Accumulated growth points
    roots_depth: f32,  // How deep roots extend
}

pub enum GrowthPhase {
    Seedling,
    Juvenile,
    Mature,
    Reproducing,
    Senescent,
}

pub enum PlantSpecies {
    Tree,
    Grass,
    Legume,
    // ... add game-specific species
}
```

---

## Simulation Systems

### Update Order (Per Frame)

Each frame, the simulation updates in this specific order:

1. **Water System**: Calculate water flow, infiltration, evaporation
2. **Soil System**: Decomposition, nutrient cycling, microbial activity
3. **Plant System**: Growth, photosynthesis, reproduction
4. **Organism System**: Fauna/microbe behavior, interactions
5. **Labor System**: Character tasks, terraforming effects
6. **Rendering**: Compute visual state for display

This ordering ensures:
- Water changes affect soil before soil affects plants
- Plants' needs are met only after water/soil update
- Labor effects (terraforming) propagate fully the next frame

### Water Flow Algorithm

Water flows from higher elevations to lower elevations. Each frame:

1. **Calculate flow direction**: For each tile with surface water, determine which neighbors it flows to
2. **Distribute flow**: Surface water moves downslope, infiltrates based on soil type
3. **Update groundwater**: Infiltrated water adds to groundwater; flows downslope underground
4. **Check retention**: Structures (swales, mulch) increase water retention

**Pseudocode**:
```rust
pub fn update_water_flow(grid: &mut Grid, dt: f32) {
    // Stage 1: Calculate infiltration for each tile
    for tile in grid.tiles_mut() {
        let infiltration_rate = calculate_infiltration_rate(
            tile.soil.soil_type,
            tile.soil.compaction,
            tile.water.soil_moisture
        );
        let infiltrated = min(tile.water.surface, infiltration_rate * dt);
        tile.water.surface -= infiltrated;
        tile.water.soil_moisture += infiltrated / tile.soil.depth;
        tile.water.groundwater_depth -= infiltrated; // Positive = deeper
    }

    // Stage 2: Calculate surface flow to downslope neighbors
    for tile in grid.tiles() {
        let elevation = tile.elevation;
        let downslope_neighbors = grid.neighbors_sorted_by_elevation(tile.position);
        
        for (neighbor_pos, _) in downslope_neighbors {
            if tile.water.surface > 0.0 {
                let flow_amount = calculate_flow(
                    tile.water.surface,
                    tile.elevation,
                    neighbor_elevation
                );
                //  Apply flow
                grid.transfer_water_between_tiles(tile.position, neighbor_pos, flow_amount);
            }
        }
    }

    // Stage 3: Evapotranspiration
    for tile in grid.tiles_mut() {
        let evaporation_rate = calculate_evaporation(tile.temperature, tile.water.soil_moisture);
        tile.water.soil_moisture -= evaporation_rate * dt;
    }
}
```

### Soil Nutrient Cycling

Nutrients cycle through decomposition and plant uptake:

```rust
pub fn update_soil_nutrients(tile: &mut Tile, dt: f32) {
    // Decomposition: organic_matter → nutrients + CO2
    let decomposition_rate = calculate_decomposition_rate(
        tile.water.soil_moisture,
        temperature,
        tile.soil.microbial_activity
    );
    let decomposed = tile.soil.organic_matter * decomposition_rate * dt;
    tile.soil.organic_matter -= decomposed;
    tile.soil.nutrients.n += decomposed * 0.05;  // ~5% of matter is N
    tile.soil.nutrients.p += decomposed * 0.01;
    tile.soil.nutrients.k += decomposed * 0.02;

    // Plant uptake: nutrients → plant growth
    for plant in tile.plants.iter_mut() {
        let uptake = calculate_plant_nutrient_uptake(plant, &tile.soil);
        for nutrient in [&mut tile.soil.nutrients.n, &mut tile.soil.nutrients.p, &mut tile.soil.nutrients.k] {
            *nutrient -= uptake;
            plant.energy += uptake;
        }
    }

    // Microbial activity: increases with organic matter and moisture
    let activity_input = min(1.0, tile.soil.organic_matter / 100.0) * tile.water.soil_moisture;
    tile.soil.microbial_activity = tile.soil.microbial_activity * 0.95 + activity_input * 0.05;
}
```

### Plant Growth

Plants grow based on available water, nutrients, and light:

```rust
pub fn update_plant_growth(plant: &mut Plant, tile: &Tile, dt: f32) {
    // Check viability
    if tile.water.soil_moisture < plant.species.min_moisture() {
        plant.energy -= 0.1 * dt;  // Stress
    }

    let nutrient_availability = (tile.soil.nutrients.n + tile.soil.nutrients.p + tile.soil.nutrients.k) / 3.0;
    if nutrient_availability < plant.species.min_nutrients() {
        plant.energy -= 0.05 * dt;  // Stress
    }

    // Growth
    if plant.energy > plant.species.growth_cost(plant.growth_phase) {
        plant.age_days += 1;
        plant.energy -= plant.species.growth_cost(plant.growth_phase);

        // Advance growth phase
        if plant.age_days > plant.species.maturity_age() {
            plant.growth_phase = GrowthPhase::Mature;
        }
    }

    // Death
    if plant.energy <= 0.0 {
        // Mark for removal
    }
}
```

---

## Module Organization

### `simulation/` Module

Contains all pure simulation logic (no UI/rendering):

- **`grid.rs`**: Hexagonal grid data structure and neighbor iteration
- **`tile.rs`**: Tile definition and accessors
- **`soil.rs`**: Soil composition and methods
- **`soil_nutrient.rs`**: Nutrient cycling algorithms
- **`soil_structure.rs`**: Soil structure, compaction, infiltration
- **`water.rs`**: Water state and properties
- **`water_flow.rs`**: Water flow simulation
- **`water_transport.rs`**: Sediment/nutrient transport with water
- **`plant.rs`**: Plant types, growth phases, species definitions
- **`organism.rs`**: Fauna, microbes, pollinator behavior
- **`ecosystem.rs`**: Biodiversity tracking and species interactions
- **`labor.rs`**: Character labor task simulation

**Design principle**: `simulation/` is testable standalone (no graphics, UI, or I/O).

### `entities/` Module

ECS-style dynamic entities (characters, structures):

- **`components.rs`**: Component struct definitions
- **`character.rs`**: Character data, stats, tasks
- **`structure.rs`**: Building definitions (swales, terraces, etc.)

**Pattern**: Components are simple data; systems (in `game.rs` or `ui/`) orchestrate behavior.

### `ui/` Module

Rendering, input handling, and UI presentation:

- **`renderer.rs`**: Hexagon rendering, isometric projection, sprite batching
- **`input.rs`**: Keyboard/mouse input parsing
- **`panels/`**: UI panel systems (status readouts, menus, etc.)

**Design principle**: `ui/` consumes data from `simulation/` and `entities/`; never modifies directly (let `game.rs` mediate).

### `game.rs`

Main game state and orchestration:

```rust
pub struct Game {
    grid: Grid,
    characters: Vec<Character>,
    structures: Vec<Structure>,
    scenario: Scenario,
    time_elapsed: Duration,
    paused: bool,
    ui_state: UIState,
}

impl Game {
    /// Update all systems
    pub fn update(&mut self, dt: f32) {
        if self.paused { return; }
        
        // Simulation updates
        self.grid.update_water_flow(dt);
        self.grid.update_soil_nutrients(dt);
        self.grid.update_plants(dt);
        self.grid.update_organisms(dt);
        self.update_labor(dt);
        self.update_scenario_progress();
    }

    /// Render current state
    pub fn render(&self) -> RenderCommands {
        // ... Prepare rendering commands
    }

    /// Handle input (player action)
    pub fn handle_input(&mut self, input: InputEvent) {
        // ... Queue labor tasks, pause, unpause, etc.
    }
}
```

---

## Entity-Component-System (ECS)

While the core simulation uses a tile-based grid, dynamic entities (characters, structures) use ECS:

### Components

```rust
pub struct Position {
    tile_pos: (i32, i32),
}

pub struct Character {
    id: u64,
    name: String,
    energy: f32,
    skill_level: u32,
}

pub struct Structure {
    id: u64,
    struct_type: StructureType,  // Swale, Terrace, Tank, etc.
    construction_progress: f32,
}

pub struct Task {
    task_type: TaskType,
    location: (i32, i32),
    progress: f32,
}
```

### Systems

Systems operate on entities with matching components:

```rust
pub fn character_update_system(
    characters: &mut [Character],
    positions: &[Position],
    tasks: &[Task],
    dt: f32,
) {
    for character in characters {
        if character.energy > 0.0 {
            character.energy -= 0.05 * dt;  // Fatigue
        }
    }
}

pub fn labor_execution_system(
    characters: &mut [Character],
    grid: &mut Grid,
    tasks: &mut [Task],
    dt: f32,
) {
    for task in tasks {
        if let Some(character) = characters.iter_mut().find(|c| c.current_task_id == task.id) {
            // Execute task (e.g., dig swale, plant trees)
            task.progress += character.work_speed() * dt;
            if task.progress >= task.work_required {
                // Complete task; apply effects to grid
                apply_task_effects(grid, task);
                task.completed = true;
            }
        }
    }
}
```

---

## Rendering Pipeline

### Isometric Projection

Hexagons are rendered in isometric perspective (viewed from above-left):

```rust
fn world_to_screen(hex_q: i32, hex_r: i32, tile_size: f32) -> (f32, f32) {
    let x = tile_size * (3.0/2.0 * hex_q);
    let y = tile_size * (sqrt(3.0)/2.0 * hex_q + sqrt(3.0) * hex_r);
    (x, y)
}
```

### Rendering Order

1. **Terrain base**: Hexagon shapes with elevation-based coloring
2. **Soil visualization**: Texture/color based on soil quality, moisture
3. **Water**: Semi-transparent overlay if surface water present
4. **Vegetation**: Plant sprites at appropriate growth stages
5. **Structures**: Swale, terrace, tank sprites
6. **Characters**: Character sprites with task indicators
7. **UI overlays**: Buttons, status panels, tool tips

### Sprite Batching

All hexagons of the same type are batched into a single draw call for performance:

```rust
pub struct SpriteBatch {
    texture: Texture,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl SpriteBatch {
    pub fn add_hex(&mut self, screen_pos: (f32, f32), tile_data: &Tile) {
        let sprite_id = get_sprite_id_for_tile(tile_data);
        // Add quad to batch
    }

    pub fn render(&self, renderer: &Renderer) {
        renderer.draw_batch(self);
    }
}
```

---

## Performance Considerations

### Optimization Strategies

1. **Spatial partitioning**: Hexagonal grid naturally partitions space
2. **Dirty flagging**: Only re-compute when tiles change
3. **Level of detail (LOD)**: Render only visible tiles; cull off-screen
4. **Batch rendering**: Group draw calls by sprite type
5. **Incremental simulation**: Update a subset of tiles each frame (stagger water, soil, plants)

### Benchmarking

Key benchmarks to track:

- **Water flow update**: 1000+ tiles per frame at 60 FPS
- **Soil nutrient cycling**: Affect all tiles regularly
- **Plant growth**: Scale from 100 to 10,000+ organisms per frame
- **Rendering**: 60 FPS at 1080p with 1000+ visible tiles

Run benchmarks regularly:
```bash
cargo bench --release
```

### Memory Layout

Tile data is laid out linearly in a `Vec`, enabling cache-friendly iteration:

```rust
pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,  // Linear layout for cache efficiency
}

// Iterate all non-border tiles (typical simulation)
pub fn tiles_inner(&mut self) -> impl Iterator<Item = &mut Tile> {
    // Skip borders, iterate contiguous memory
}
```

---

## Adding New Systems

To add a new simulation system (e.g., pollinator behavior):

1. **Define data structures** in `simulation/pollinator.rs`
2. **Implement update logic**:
   ```rust
   pub fn update_pollinators(grid: &mut Grid, dt: f32) {
       for tile in grid.tiles_mut() {
           // Update pollinator state
       }
   }
   ```
3. **Integrate into `game.rs`** update order:
   ```rust
   pub fn update(&mut self, dt: f32) {
       // ... after plant updates
       self.grid.update_pollinators(dt);
   }
   ```
4. **Add tests** in `tests/` directory
5. **Document** with rustdoc comments
6. **Benchmark** if CPU-intensive

---

## References

- **Hexagonal grids**: [Red Blob Games - Hexagonal Grids](https://www.redblobgames.com/grids/hexagons/)
- **ECS pattern**: [Bevy ECS book](https://bevyengine.org/)
- **Permaculture design**: [Sepp Holzer's works](https://www.sepp-holzer.at/)
- **Soil ecology**: Academic literature on soil carbon, nutrient cycling, microbial ecology
