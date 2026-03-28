# Regrowth

**Restore degraded ecosystems through permaculture, community labor, and ecological simulation.**

![Status Badge](https://img.shields.io/badge/status-Early%20Development-yellow)
![License Badge](https://img.shields.io/badge/license-MIT-green)

---

## Quick Pitch

Regrowth is a gamified ecological simulation where you restore degraded land using permaculture techniques, water harvesting, and community labor. Plan interventions, manage workers, observe how soil, water, and life systems naturally regenerate, and accomplish meaningful restoration goals. Every decision impacts the ecosystem—watch bare earth transform into thriving, biodiverse landscapes.

---

## Vision & Overview

### What is Regrowth?

Regrowth simulates the ecological restoration process at a level of detail rarely seen in games. Rather than abstract mechanics, the game models the actual systems that drive ecosystem recovery:

- **Soil composition and nutrient cycles**: Watch organic matter accumulate, microbes colonize, and soil health rebuild over time.
- **Water hydrology**: Rain infiltrates soil, establishes groundwater, flows downslope—all with physically plausible behavior.
- **Plant and organism interactions**: Vegetation grows according to soil/water conditions, attracts pollinators and beneficial insects, improves microbial diversity.
- **Human labor and intervention**: Characters perform targeted work (swale construction, terracing, planting) to accelerate restoration.

The game is **single-player, scenario-based**, with each scenario presenting a different restoration challenge and storyline (e.g., joining the Great Green Wall project, rescuing a family farm, rewilding degraded pasture).

### Design Vision

Regrowth bridges two worlds:

1. **For strategy/simulation enthusiasts**: Emergent gameplay where ecological systems interact in surprising ways. Decisions have ripple effects (e.g., planting trees affects water retention, which supports more vegetation, which attracts fauna).
2. **For permaculture/sustainability advocates**: A thoughtful exploration of real restoration techniques translated into engaging gameplay. Learn *why* swales work, how polycultures succeed, and what "building soil" truly means.

### Why Permaculture?

Permaculture provides both the game's core mechanics and its philosophical foundation:

- **Observation & design cycles**: Players assess land, design interventions, execute labor, observe results.
- **Working with natural systems**: Rather than fighting against hydrology or soil biology, permaculture techniques *harmonize* with them.
- **Layered productivity**: Vertical tile profiles (subsurface, surface, canopy) mirror permaculture's stacking of functions and yields.
- **Practical hope**: Degraded land *can* recover with the right approach—a powerful narrative for a game.

---

## Gameplay Highlights

### Core Gameplay Loop

Each scenario follows this cycle:

1. **Assess**: Examine current land state (soil quality, moisture, vegetation, organisms).
2. **Plan**: Design interventions (swales, plantings, structures) and assign labor tasks.
3. **Execute**: Characters perform work over simulated time. Progress is visible (swala dug, seeds germinate, water infiltrates).
4. **Observe**: Ecosystem responds—soil improves, vegetation grows, organisms colonize, water becomes available.
5. **Adapt**: Adjust strategy based on ecological outcomes and scenario goals.

Repeat until scenario objectives are met or you decide to restart with a different approach.

### Aesthetic & Presentation

- **Perspective**: 2D isometric (Dwarf Fortress / Rimworld aesthetic), though with a "game of life" simulation depth beneath the surface
- **Map Structure**: Hexagonal grid where each tile contains vertical "layers":
  - **Surface**: Terrain height, features (trees, water bodies, built structures)
  - **Subsurface**: Soil type, nutrient content, water saturation, organic matter
  - **Below**: Groundwater levels, root zones, microbial activity
- **Visual Feedback**: Color, texture, and icons convey soil quality, moisture, vegetation health, and biodiversity presence

### Game Pace & Timeflow

- **In-game time**: Days → Seasons → Years (configurable timescale)
- **Player control**: Pause, slow, or accelerate simulation to observe ecological changes in real-time
- **Growth cycles**: Plant germination, growth, flowering, and decomposition follow realistic timelines
- **Seasonal effects**: Temperature, precipitation, and dormancy cycles drive ecosystem behavior

---

## Core Systems

### Character & Labor Management

**Characters** are the player's intervention tools. Each character:

- **Is assigned labor tasks**: Dig swales, build terraces, plant trees, apply mulch
- **Performs work over time**: Tasks consume stamina and take realistic duration
- **May develop skills**: Experience in digging, planting, or irrigation improves work efficiency
- **Has basic AI**: Can prioritize tasks or wander when idle
- **Requires sustenance**: Will require food, water, sleep, places to dispose waste.

**Labor is limited**: Players must prioritize which interventions happen first, creating strategic choice.

### Terrain & Land Systems

The map is a **hexagonal grid** where terrain and subsurface matter equally:

- **Elevation & topology**: Determines water flow direction and erosion patterns
- **Soil types**: Sand, loam, clay—each has different infiltration, retention, and nutrient characteristics
- **Soil depth**: Tracks layers from surface to bedrock/clay pan
- **Vertical mixing**: Tillage, burrowing organisms, or water movement can mix layers
- **Elevation modification**: Terraforming (cutting, filling, building terraces) is labor-intensive but radically changes hydrology

**Resource conservation**: Soil doesn't vanish from the map—dug earth becomes fill elsewhere; eroded soil deposits downslope.

### Water Systems & Hydrology

Water is the lifeblood of restoration. The game models:

- **Precipitation**: Rainfall events simulate seasonal/random variability
- **Infiltration**: Water soaks into soil at rates determined by soil type and slope
- **Groundwater**: Infiltrated water accumulates below surface, flowing slowly downslope
- **Surface flow**: Excess rainfall runs downslope, potentially eroding unprotected soil
- **Evapotranspiration**: Water leaves soil and plants, affected by temperature and vegetation
- **Water transport**: Fast-flowing water can carry sediment and organic matter downslope

**Player interventions** (swales, mulch) slow surface flow and boost infiltration, raising groundwater and reducing erosion.

### Ecosystem Health & Biodiversity

The game tracks ecosystem "richness":

- **Vegetation diversity**: Different plant species establish based on soil and water conditions
- **Fauna presence**: Pollinators, beneficial insects, and soil fauna indicated by icons/indicators
- **Microbial diversity**: Not directly visible, but affects nutrient availability and soil structure
- **Biodiversity index**: A score reflecting species diversity and ecosystem health (affects resilience to stress)

**Complex interactions**:
- Plants with high-quality soil and water encourage fauna
- Fauna (especially insects and microbes) improve soil structure and fertility
- Diversity buffers ecosystem against disturbances (drought, pests, erosion)

### Building & Infrastructure

Players construct structures to modify terrain hydrology and create growing conditions:

- **Swales**: Curved ditches that capture surface runoff and maximize infiltration (foundational technique)
- **Terraces**: Level benches on slopes that reduce erosion and create stable planting zones
- **Water tanks/ponds**: Capture and store runoff for dry periods
- **Mulch patches**: Increase water retention and organic matter accumulation
- **Guilds/polycultures**: Defined plant groupings with complementary functions (nitrogen fixation, pest suppression, etc.)
- **Trees & tree canopies**: Provide shade, reduce evaporation, drop organic matter
- **Free terraform**: The player can also manually create terraforming operations, based on the terrain and various requirements.

Each structure has **labor costs** and **takes time to construct**, but provides long-term hydrological or fertility effects.

---

## Advanced Mechanics

### Soil & Nutrient Cycles

Restoring soil is regrowth's core challenge:

- **Organic matter**: Accumulates as vegetation dies and decomposes; improves water retention and microbial activity
- **Nutrient cycling**: Nitrogen, phosphorus, potassium circulate through plants, soil, and microbes
- **Decomposition**: Dead matter breaks down over time (dependent on moisture, temperature, microbial activity)
- **Mineral weathering**: Rocks slowly release minerals; accelerated by organic acids in soil
- **Nutrient deficiency**: Plants cannot grow on nutrient-poor soil; must wait for cycles to rebuild or manually amend

### Organism Simulation & Life Systems

Beyond biodiversity tracking, individual organism types matter:

- **Soil microbes**: Facilitate decomposition, nutrient cycling, and soil structure; colonize based on organic matter and moisture
- **Pollinators**: Required for productive vegetation; attracted by diverse flowering plants
- **Soil fauna** (earthworms, arthropods): Improve soil structure, aeration, nutrient cycling
- **Pest dynamics**: Various pests thrive in monocultures; suppressed by diversity and natural predators

### Growth Cycles & Seasons

Vegetation follows realistic timelines:

- **Germination**: Seeds require moisture and appropriate temperature; take days to weeks
- **Growth phases**: Seedling → juvenile → mature; each phase has different moisture/nutrient needs
- **Seasonal dormancy**: Trees shed leaves; cold kills annuals; patterns affect water and nutrient cycling
- **Reproduction**: Mature plants flower and set seed, spreading diversity

### Complex Interactions & Emergence

The simulation models surprising ecological interactions:

- **Swales + mulch + polycultures**: Together they create a self-reinforcing cycle of improved soil and water retention
- **Canopy effects**: Established trees shade understory, reduce evaporation, drop organic matter → improved soil → more vegetation growth
- **Nitrogen fixation**: Legume-family plants enriching soil; nitrogen-hungry plants then thrive; diversity improves
- **Tipping points**: After reaching a soil-fertility or biodiversity threshold, the landscape "tips" into rapid regeneration

---

## Scenarios, Goals & Failure Modes

### Scenario Framework

Each scenario presents:

- **Location**: A specific degraded landscape with particular soil, climate, and baseline vegetation
- **Storyline**: A narrative framing (e.g., "A family joins the Great Green Wall project to restore their ancestral land")
- **Constraints**: Starting resources (labor budget, tools available, initial plant supply)
- **Victory conditions**: Specific restoration targets (e.g., "Establish 500 trees," "Raise groundwater table by 2m," "Achieve biodiversity index > 50")
- **Time limit** (optional): Some scenarios have deadline pressure
- **Politics fallout** (optimal): The deciusions by the goverment may affect you in various ways, may it be availability, bans on invasive, but helpfull plants, familiy members getting drafted, etc. 

### Goals & Objectives

Possible victory conditions include:

- **Ecological restoration**: Reach biodiversity, soil health, or groundwater targets
- **Productive landscape**: Establish functional food production or forage systems
- **Resilience**: Withstand simulated drought or extreme weather without degradation
- **Resource conservation**: Complete restoration within labor or time budgets

### Example Scenarios

1. **The Great Green Wall**: Join a community project to combat desertification. Restore degraded savanna through strategic planting and swale networks. Goal: Establish 1000 trees and raise groundwater in 50 in-game years.

2. **Family Farm Revival**: Your family's farm has eroded and lost fertility. Limited budget, small team of characters. Restore soil health and establish productive guilds. Goal: Reach soil quality threshold and support 20 productive plants.

3. **Rewilding the Commons**: A communal degraded pasture. You must restore it to support diverse wildlife while providing community resources. Goal: Achieve high biodiversity, establish 100+ tree species, support visible fauna.

---

## Features Status & Roadmap

### Development Status Overview

| Phase | Status | Features |
|-------|--------|----------|
| **Planning** | ✅ Complete | Design docs, mechanics proposals, prototype data structures |
| **Prototyping** | ⏳ In Progress | Hexagonal grid system, basic water flow, simple soil model |
| **Alpha** | 📋 Planned | All core systems playable, 1-2 scenarios, basic UI |
| **Beta** | 📋 Planned | 5+ scenarios, polish, balance, player feedback iteration |
| **Release** | 🎯 Planned | Full feature set, optimized, documented |

### Core Features (Foundation)

- [ ] Hexagonal grid map with vertical tile profiles
- [ ] Soil composition and nutrient tracking
- [ ] Water flow physics and infiltration
- [ ] Character labor system
- [ ] Terrain modification (swales, terraces)
- [ ] Basic vegetation and growth
- [ ] Scenario framework and objectives
- [ ] Pause/play time control

### Nice-to-Have Features (Polish & Depth)

- [ ] Character skill progression
- [ ] Advanced organism simulation (microbes, pollinators, pests)
- [ ] Building construction UI and feedback
- [ ] Seasonal weather variation
- [ ] Save/load games
- [ ] Detailed ecosystem graph visualization
- [ ] Audio (ambient soundscape, satisfying labor sounds)

### Future/Stretch Goals (Vision Extensions)

- [ ] Multiplayer cooperative restoration
- [ ] Workshop/mod support for custom scenarios
- [ ] Integration with permaculture research datasets
- [ ] Educational mode with explicit learning objectives
- [ ] Landscape generation based on real ecological data
- [ ] Advanced AI for NPC characters with relationships/stories

---

## Getting Started

### For Players

**System Requirements**: [TBD upon implementation]

**How to Play**:
1. Launch the game and select a scenario
2. Review starting conditions and objectives
3. Pause the game; use tools to plan swales, plantings, structures
4. Assign labor tasks to your characters
5. Unpause and watch the simulation progress
6. Observe ecosystem changes and adapt your strategy

**Tips**:
- Start with **swales**: The foundational permaculture technique; dramatically improve water infiltration
- **Mulch early**: Protects soil and speeds organic matter accumulation
- **Plant diverse**: Monocultures are fragile; polycultures are resilient and self-supporting
- **Observe seasonality**: Some growth happens only during specific seasons

### For Contributors

We welcome contributions from ecologists, game developers, permaculture practitioners, and artists!

**How to Get Involved**:
1. Check [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines
2. Review [open issues](https://github.com/Kelmoir/regrowth/issues) for tasks
3. Propose new features or scenarios via GitHub discussions

**Areas of Need**:
- **Simulation accuracy**: Help refine water flow, nutrient cycling, and ecosystem models
- **Scenario design**: Create compelling restoration scenarios with ecological depth
- **UI/UX**: Improve player feedback and usability
- **Art & aesthetics**: Hexagon sprites, terrain textures, UI design
- **Documentation**: Expand game design docs and technical architecture

### Development Setup

**Prerequisites**:
- Rust 1.70+ (https://rustup.rs/)
- Cargo

**Clone & Build**:
```bash
git clone git@github.com:Kelmoir/regrowth.git
cd regrowth
cargo build --release
cargo run
```

**Project Structure**:
```
regrowth/
├── src/
│   ├── main.rs              # Entry point
│   ├── simulation/          # Core systems (soil, water, organisms)
│   ├── ui/                  # User interface and rendering
│   ├── scenarios/           # Scenario definitions
│   └── util/                # Helpers and utilities
├── tests/                   # Integration tests
├── benches/                 # Performance benchmarks
├── Cargo.toml               # Rust dependencies
└── README.md                # This file
```

**Running Tests**:
```bash
cargo test                   # Unit tests
cargo test --test '*'        # Integration tests
cargo bench                  # Performance benchmarks
```

---

## Additional Resources

### FAQ

**Q: Is this a real-time strategy game?**
A: It's closer to a pausable turn-based strategy game with real-time simulation underneath. You plan interventions, assign labor, then watch the ecosystem respond over simulated time. You can pause, adjust, and resume.

**Q: How complex are the simulation mechanics?**
A: Complex enough to be scientifically interesting but simple enough to remain playable. We model soil-, water-, and organism-level dynamics without aiming for PhD-level accuracy.

**Q: Can I play multiple scenarios or is it story-driven?**
A: Both! Each scenario has a narrative frame, but you're free to tackle them in any order. Multiple playthroughs encourage different strategies.

**Q: Will there be multiplayer?**
A: Not in initial release, but it's a planned stretch goal for cooperative restoration.

**Q: How long are scenarios?**
A: Roughly 30 minutes to 2 hours depending on difficulty and player style.

### Architecture Overview

See [ARCHITECTURE.md](./ARCHITECTURE.md) for details on:
- Hexagonal grid implementation
- Vertical tile profiles and subsurface storage
- Water flow and soil transport algorithms
- ECS (Entity-Component-System) patterns for organisms and structures
- Rendering pipeline and UI framework

### License

This project is licensed under the [MIT License](./LICENSE).

### Credits & Team

**Core Team**: Sebastian Hacker

**Special Thanks**:
- Permaculture practitioners and educators who provided design input
- Indie game communities for inspiration and feedback
- Open-source Rust ecosystem contributors

**GitHub**: [github.com/Kelmoir/regrowth](https://github.com/Kelmoir/regrowth)

---

**Last Updated**: 28. März 2026
**Next Update**: prototyping