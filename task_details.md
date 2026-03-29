# Task Details: Regrowth Implementation

> **Document Scope**: Detailed breakdown of Phase 1 tasks only. See [project.md](project.md) for full project overview.

---

## Phase 1: Foundations (Weeks 1-2)

### 01_01 — Hexagonal Grid Generation System

**Description**:
Implement the core hexagonal grid data structure that serves as the spatial foundation for the entire game world. This system generates a 50×50 hexagonal grid (configurable), calculates hex coordinates using axial/cube coordinate systems, computes neighbor relationships, and provides efficient spatial queries.

**Prerequisites**:
- Rust project initialized with Cargo.toml
- Basic mathematical vector types (2D/3D points)

**Related Systems**:
- Core world data structure
- Tile properties system (01_03)
- Rendering system (01_04)
- All simulation systems (water, soil, plants)

**Success Criteria**:
1. ✓ Hexagonal grid generates a 50×50 grid with standard axial coordinates (50 width, 50 height)
2. ✓ Each hex tile is identified by unique (x, y) axial coordinates
3. ✓ Neighbor calculation works correctly for all 6 adjacent hexes
4. ✓ Coordinate conversion functions (axial ↔ cube ↔ pixel/screen coords) implemented
5. ✓ Distance calculation between tiles returns correct hexagonal distance
6. ✓ Line-of-sight/raycasting between hex tiles supported
7. ✓ No panics on out-of-bounds queries; returns None or handles gracefully
8. ✓ Unit tests cover edge cases (corners, center, neighbors)

**Implementation Details**:
- Use **axial coordinates** (col, row) as primary representation for efficiency
- Implement **cube coordinate** conversion for distance/rotation math
- Create `Hex` type with methods: `neighbors()`, `distance_to()`, `line_to()`, `ring_at()`
- Create `Grid<T>` generic type holding tile data
- Measurement system: 5cm hexagon tiles → game units and pixel coordinates
- Support iteration: `grid.iter()`, `grid.iter_mut()`, `grid.neighbors(hex)`

**Technical Notes**:
- Axial vs. Cube: Use axial for storage (2D), convert to cube for math
- Neighbor directions: 6 directions defined as constants
- Consider using `bevy_grid` or similar crate if available, or implement custom
- Grid generation should be fast (<100ms for 50×50)

**Estimated Duration**: 2-3 days

**Subtasks**:
- 01_01_a: Define coordinate system types and conversion functions
- 01_01_b: Implement grid data structure and iteration
- 01_01_c: Implement neighbor finding and distance calculations
- 01_01_d: Add unit tests (coordinate conversion, neighbors, distances)

---

### 01_02 — Bevy Project Setup & Component Architecture

**Description**:
Set up the Bevy 0.18 application structure with all foundational plugins, components, and resources. Establish the plugin architecture following Bevy patterns, define core ECS components (Tile, Water, Soil, Plant, Character, etc.), and set up the main update loop with proper system scheduling.

**Prerequisites**:
- Cargo.toml configured with Bevy 0.18 and required dependencies
- Basic understanding of Bevy's ECS and plugin system

**Related Systems**:
- All simulation systems (water, soil, plants)
- Rendering system (01_04)
- Input system (02_04)
- UI system (01_06)

**Success Criteria**:
1. ✓ Bevy app initializes without errors
2. ✓ All core components defined: `TileComponent`, `Water`, `Soil`, `Plant`, `Character`
3. ✓ Resources created: `GameState`, `GameConfig`, `TimeManager`
4. ✓ Plugin architecture established with clear module organization
5. ✓ System scheduling allows proper execution order (time → simulation → rendering)
6. ✓ Debug window shows basic game info (tile count, time)

**Implementation Details**:
- Define all components with `#[derive(Component)]`
- Use **bevy::reflect** for all serializable data (save/load preparation)
- Establish plugin structure:
  ```
  - GridPlugin (loads hex grid, initializes tiles)
  - TimePlugin (game time management)
  - WaterSimulationPlugin (water systems)
  - SoilSimulationPlugin (soil systems)
  - PlantSimulationPlugin (plant systems)
  - RenderingPlugin (hex rendering)
  - UIPlugin (status panel)
  - InputPlugin (keyboard/mouse)
  - (Others deferred to Phase 2+)
  ```
- Create `GameState` resource holding current simulation time, pause state, scenario data
- Implement `TimeManager` to track real time vs. simulation time with configurable speed multiplier

**Technical Notes**:
- Use Bevy 0.18 schedule API (not older `CoreStage` system)
- Components should be lightweight; avoid game logic in component definitions
- Use `SystemSet` or `#[derive(SystemSet)]` for organizing related systems
- Plugins should be small, focused, and independently testable

**Estimated Duration**: 2 days

**Subtasks**:
- 01_02_a: Define core components and resources
- 01_02_b: Create plugin structure with placeholder systems
- 01_02_c: Implement TimePlugin with pause/play mechanics
- 01_02_d: Add basic debug output system

---

### 01_03 — Core Data Structures (Tile, Grid, World State)

**Description**:
Design and implement the data structures holding tile-level world state. Each tile is a hexagonal cell with subsurface profiles (soil layers, water content, organisms) and surface features (plants, structures). Implement efficient storage and access patterns for the tile grid and establish the measurement system bridge between real-world units and game units.

**Prerequisites**:
- 01_01: Hexagonal Grid Generation System (complete)
- 01_02: Bevy Project Setup (component definitions available)

**Related Systems**:
- Grid system (01_01)
- Water system (01_05)
- Soil system (Phase 2, 02_01)
- Plant system (Phase 2, 02_02)

**Success Criteria**:
1. ✓ `Tile` struct defined with all required properties
2. ✓ Tile storage integrated with hexagonal grid
3. ✓ Measurement system: 5cm hexagons, depth layers, and game unit conversion working
4. ✓ Efficient random access and iteration over tiles (O(1) lookup)
5. ✓ Tile state serializable/deserializable (for save system)
6. ✓ Subsurface layer access (soil, water, organisms) works correctly
7. ✓ No performance degradation for 50×50 grid updates

**Implementation Details**:
- Define `Tile` with:
  ```
  - Position: enum of axial/cube coords
  - Elevation: height in game units
  - Terrain type: grass, bare, rock, water (enum)
  - Water: WaterProfile { depth, saturation, ... }
  - Soil: SoilProfile { layers, fertility, structure, ... }
  - Plants: Vec<PlantInstance> (up to 5-10 per tile)
  - Subsurface organisms: microbial count, fauna state
  ```
- Define `SoilProfile` with vertical layers (0-30cm depth, 6 layers)
- Define `WaterProfile` with saturation, infiltration rate, linked to neighbors
- Create `World` resource holding grid and additional metadata
- Implement `TileGrid<T>` wrapper supporting efficient storage (likely Vec in row-major order)
- Use **bevy::reflect** for all structures for debug inspection

**Technical Notes**:
- Tile data should be immutable from ECS perspective; use mutable borrows in systems only
- Consider using `SmallVec` or similar for plants per tile (rarely >5)
- Measurement system constants defined in `config.rs`:
  ```
  const HEX_CIRCUMRADIUS_CM: f32 = 5.0;  // Real world
  const SOIL_DEPTH_CM: f32 = 30.0;
  const GAME_UNIT_TO_MM: f32 = 10.0;     // 1 game unit = 10mm
  ```

**Estimated Duration**: 2-3 days

**Subtasks**:
- 01_03_a: Define Tile struct with water and soil profiles
- 01_03_b: Implement World resource and TileGrid wrapper
- 01_03_c: Implement measurement system conversions
- 01_03_d: Add reflection and serialization support

---

### 01_04 — Isometric Tile Rendering Pipeline

**Description**:
Implement the hexagonal tile rendering system using Bevy's rendering pipeline. Render a perspective isometric view of the hexagonal grid with sprite-based hexagon tiles. Each tile should display its current state (water level, soil fertility, vegetation density) through visual feedback. Establish camera controls and viewport setup.

**Prerequisites**:
- 01_02: Bevy Project Setup (render plugins initialized)
- 01_03: Core Data Structures (tile data available)

**Related Systems**:
- Grid system (01_01)
- UI system (01_06)
- Input system (02_04)

**Success Criteria**:
1. ✓ 50×50 hex grid renders without visual artifacts or overlaps
2. ✓ Each hex tile displays correct isometric projection
3. ✓ Tile coloring reflects basic state (water level, soil type)
4. ✓ Camera provides clear isometric view with correct aspect ratios
5. ✓ Frame rate stable (>30 FPS on standard hardware for a single 50×50 grid)
6. ✓ Sprites load and scale correctly for 5cm hexagons at standard zoom
7. ✓ Z-ordering correct (no flickering, proper depth sorting)

**Implementation Details**:
- Use Bevy's 2D rendering with isometric projection matrix
- Create hex sprite sheet asset (provide 6 basic tile variants: bare, grass, water, etc.)
- Implement coordinate conversion from hex grid → world position → screen position
- Use `Transform` component to position all hexagon sprites
- Camera setup:
  ```
  - Orthographic projection for isometric (no perspective distortion)
  - Viewport centered on grid
  - Zoom level: adjustable (Phase 2: add mouse wheel zoom)
  ```
- Color system for tiles (terrain type → base color, water level → brightness)
- Optional: Color banding for soil fertility (richer colors = better soil)

**Technical Notes**:
- Isometric projection: x → screen_x + y/2, y → screen_y (or similar standard formula)
- Sprite scale: `HEX_CIRCUMRADIUS_PX = 100` pixels at standard zoom
- Consider using `bevy_sprite` for simple sprite rendering
- Pre-generate sprite UV coordinates for efficient batch rendering
- Use z-ordering: y-position determines draw order (painter's algorithm)

**Estimated Duration**: 3-4 days

**Subtasks**:
- 01_04_a: Set up camera and orthographic projection
- 01_04_b: Implement hex → screen coordinate conversion
- 01_04_c: Create sprite rendering system with color based on tile state
- 01_04_d: Add sprite asset loading and z-ordering
- 01_04_e: Test rendering performance and optimize if needed

---

### 01_05 — Water Flow Simulation (Foundation)

**Description**:
Implement the foundational water flow system that simulates infiltration, surface runoff, and groundwater movement across the hexagonal grid. This is the core simulation system that affects all downstream mechanics (soil saturation, plant growth, character pathfinding). Establish the framework for water state updates and neighbor-based flow calculations.

**Prerequisites**:
- 01_01: Hexagonal Grid Generation (neighbor calculations available)
- 01_03: Core Data Structures (water profile structure defined)
- 01_02: Bevy Project Setup (simulation plugin framework)

**Related Systems**:
- All soil systems (Phase 2)
- All plant systems (Phase 2)
- Labor system (Phase 3)

**Success Criteria**:
1. ✓ Water flows from uphill to downhill hexes based on elevation
2. ✓ Infiltration: water enters soil, saturation increases
3. ✓ Evapotranspiration: water loss each simulation cycle
4. ✓ Water level tracked per tile (surface water + subsurface saturation)
5. ✓ Simulation produces plausible results (water pools in low areas, drains from slopes)
6. ✓ System runs at reasonable speed for 50×50 grid (update < 5ms per frame)
7. ✓ Water conservation: no water appears/disappears unexpectedly

**Implementation Details**:
- Define `WaterProfile` struct:
  ```
  - surface_water: f32,      // mm of surface water
  - soil_saturation: [f32; 6],  // per-layer saturation (0-100%)
  - infiltration_rate: f32,   // soil-dependent
  - evapotranspiration: f32,  // daily rate
  ```
- Implement water flow algorithm:
  1. Calculate water pressure at each tile (based on surface water + saturation)
  2. Distribute flow to lower neighbors (proportional to elevation difference)
  3. Apply infiltration (surface water → soil layers)
  4. Apply evapotranspiration (reduce saturation)
  5. Update tile state
- Add rainfall input: accept rainfall events (configurable mm/day)
- Config parameters:
  ```
  - BASE_INFILTRATION_RATE: 5.0 mm/hour
  - EVAPOTRANSPIRATION_RATE: 0.5 mm/day (configurable by vegetation)
  - MAX_SURFACE_WATER: 50.0 mm (after this, runoff occurs)
  ```

**Technical Notes**:
- Use iterative approach (all flow calcs happen in parallel, then update state)
- Consider using a `FlowStep` schedule or system ordering to ensure correct sequence
- Elevation can be hardcoded as noise or procedural for Phase 1 (placeholder)
- Saturation should affect plant growth and soil properties
- Performance: avoid per-pixel water simulation; use per-tile flow only

**Estimated Duration**: 3-4 days

**Subtasks**:
- 01_05_a: Define water profile and flow data structure
- 01_05_b: Implement elevation-based flow algorithm
- 01_05_c: Implement infiltration and evapotranspiration
- 01_05_d: Add rainfall input and test water conservation
- 01_05_e: Benchmark and optimize for real-time performance

---

### 01_06 — Basic UI Framework (Pause/Play, Status Panel)

**Description**:
Implement a minimal UI system for game state control and status display. Create a pause/play button (keyboard shortcut initially), a status panel showing current time, tile count, and water statistics, and establish the foundation for future UI expansions. Use Bevy's UI system with a simple layout.

**Prerequisites**:
- 01_02: Bevy Project Setup (UI plugin initialized)
- 01_03: Core Data Structures (world state available)
- 01_05: Water Flow Simulation (water stats to display)

**Related Systems**:
- Input system (02_04)
- Rendering system (01_04)
- All simulation systems (state displayed in status panel)

**Success Criteria**:
1. ✓ Pause/play toggle works (keyboard: spacebar or P)
2. ✓ Status panel displays in a readable location (top-left or corner)
3. ✓ Panel shows: simulation time, game speed multiplier, paused status
4. ✓ Panel shows: total tiles, water statistics (avg saturation, surface water)
5. ✓ Text rendering is legible at standard resolution (1920×1080)
6. ✓ UI updates every frame without lag
7. ✓ UI toggles visibility (press H for help/hide)

**Implementation Details**:
- Use Bevy's `bevy::ui` with `NodeBundle` for layout
- Create simple status panel layout:
  ```
  ┌─────────────────────────┐
  │ REGROWTH - Paused       │
  │ Time: Day 5, 14:32      │
  │ Speed: 1.0x             │
  │ Tiles: 2500             │
  │ Avg Water: 45%          │
  │ Surface Water: 12 mm    │
  └─────────────────────────┘
  ```
- Create `UIState` resource holding panel visibility and layout prefs
- Implement pause system: when paused, all simulation systems skip updates
- Use `TextBundle` for text rendering with `Val::Px` for fixed positioning
- Colors: Follow simple palette (white text, dark background, blue water stats)

**Technical Notes**:
- Pause mechanics: use `run_if()` condition on pause state for all simulation systems
- Time formatting: convert seconds to "Day X, HH:MM" format
- Statistics: compute averages from world tiles in a separate system each frame
- Consider using `bevy_egui` or simple text rendering for MVP
- Font: Use default Bevy font for now; can improve in Phase 3

**Estimated Duration**: 1-2 days

**Subtasks**:
- 01_06_a: Create UIState resource and pause system
- 01_06_b: Implement status panel layout and text rendering
- 01_06_c: Add toggle visibility shortcut (H key)
- 01_06_d: Compute and display statistics from world state

---

## Dependencies & Interfaces Between Phase 1 Tasks

### Dependency Order

```
01_01: Hexagonal Grid Generation
  ↓
01_02: Bevy Project Setup & Components
  ↓
01_03: Core Data Structures (Tile, Grid, World)
  ↓
  ├── 01_04: Isometric Rendering (consumes Tile data)
  ├── 01_05: Water Flow Simulation (modifies Tile water profiles)
  └── 01_06: UI Framework (displays statistics from Tiles)
```

### Interface Points

| From Task | To Task | Interface | Data Flow |
|-----------|---------|-----------|-----------|
| 01_01 | 01_03 | Grid structure | Grid provides coordinate systems and neighbor queries |
| 01_03 | 01_04 | Tile state | Rendering reads tile properties (elevation, water, terrain) |
| 01_03 | 01_05 | Water profile | Water system mutates WaterProfile; affects plant/soil components |
| 01_05 | 01_06 | Statistics | Status panel queries water stats from WaterProfile |
| 01_02 | All | Plugin framework | TimePlugin, scheduling system used by all simulation systems |

### Coordination Notes

- **01_04 & 01_05 Parallelization**: Rendering and water simulation are independent; can develop in parallel after 01_03 is complete
- **Clock Dependencies**: 01_06 requires 01_02's TimePlugin; 01_05 also depends on TimePlugin for delta time
- **Testing Strategy**: Each task should include unit tests; integration tests combine water + rendering in mid-Phase 1

---

## Phase 1 Success Criteria (Overall)

The Phase 1 milestone is complete when:

1. ✓ Bevy app runs and displays a 50×50 hexagon grid in isometric view
2. ✓ Water flows across the grid, infiltrates, and evaporates realistically
3. ✓ Pause/play mechanism works; UI displays current state
4. ✓ All systems run without crashes or memory leaks (validated with profiler)
5. ✓ Code is well-organized into plugins and systems (ready for external developers)
6. ✓ Performance baseline: 60 FPS minimum on standard hardware (Intel i5 / 8GB RAM)
7. ✓ Documentation: Each task includes comments explaining architectural choices
