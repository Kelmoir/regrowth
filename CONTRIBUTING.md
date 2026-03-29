# Contributing to Regrowth

Thank you for your interest in contributing to Regrowth! This document outlines how to get involved, what we're looking for, and how to submit your work.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Areas of Need](#areas-of-need)
5. [Code Standards](#code-standards)
6. [Submission Process](#submission-process)
7. [Communication](#communication)

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please review our [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) before participating.

**TL;DR**: Be respectful, constructive, and supportive. Harassment or discrimination of any kind is not tolerated.

---

## Getting Started

### Prerequisites

- **Rust**: 1.70+ (install via [rustup.rs](https://rustup.rs/))
- **Cargo**: Comes with Rust
- **Git**: For version control
- **Basic Rust knowledge**: Familiarity with Cargo, crates, and Rust's ownership model is helpful

### Clone the Repository

```bash
git clone git@github.com:Kelmoir/regrowth.git
cd regrowth
```

### Build & Run Locally

```bash
# Build in development mode (faster compilation)
cargo build

# Run the game
cargo run

# Build optimized release
cargo build --release

# Run tests
cargo test

# Run integration tests specifically
cargo test --test '*'

# Run benchmarks (performance)
cargo bench

# Check code without building
cargo check

# Format code (auto-fix)
cargo fmt

# Lint code
cargo clippy
```

### First-Time Setup

1. **Familiarize yourself with the codebase**: Read [ARCHITECTURE.md](./ARCHITECTURE.md) for an overview of core systems.
2. **Review open issues**: Check [GitHub Issues](https://github.com/regrowth/regrowth/issues) for tasks labeled `good first issue`.
3. **Read the game design**: Review [README.md](./README.md) to understand the vision and mechanics.
4. **Ask questions**: Join our discussions or open a GitHub Discussion if something is unclear.

---

## Development Workflow

### Coordinated Development with Agents

Regrowth uses specialized development agents to maintain code quality:

- **Planner**: Breaks down features into clear, implementable tasks
- **Coder**: Implements features with idiomatic Rust and full documentation
- **Reviewer**: Evaluates code for SOLID principles, performance, and architecture
- **Tester**: Writes unit tests (90%+ coverage minimum)
- **Integration Tester**: Tests multi-module interactions and game scenarios
- **Benchmarker**: Profiles performance and detects regressions

**For contributors**: You follow this same workflow:

### Step-by-Step Contribution Process

#### 1. Pick a Task or Propose a Feature

**Option A: Work on an existing issue**
- Find an issue in the [backlog](https://github.com/Kelmoir/regrowth/issues)
- Comment that you're interested in working on it
- Maintainers will assign it to you

**Option B: Propose a new feature**
- Open a GitHub Discussion describing the feature
- Discuss design and feasibility with the team
- If approved, an issue will be created for you

#### 2. Plan Your Implementation

- Break down the task into small, testable units
- Identify which simulation systems are affected (soil, water, organisms, labor, etc.)
- Design interfaces between modules before coding
- Document your plan in the issue

#### 3. Create a Feature Branch

```bash
git checkout -b feature/your-descriptive-name
# Example: feature/soil-nutrient-cycling
# Example: bugfix/water-flow-direction
```

#### 4. Implement with Code Standards

Follow the [Code Standards](#code-standards) section below. Key points:

- Write **idiomatic Rust** (Rust 2021 edition, Bevy 0.18+)
- Follow **SOLID principles**
- Document all public modules and functions
- Use **Python-style naming** (snake_case functions, PascalCase types)
- Include **unit tests** alongside your code
- Handle errors with `Result<T, E>` and `Option<T>` (use `thiserror` crate)

#### 5. Run Quality Checks

```bash
# Format your code
cargo fmt

# Check for lint/style issues
cargo clippy -- -D warnings

# Run tests
cargo test

# Run integration tests
cargo test --test '*'

# Check coverage (requires `cargo tarpaulin` or `cargo llvm-cov`)
cargo tarpaulin --out Html --output-dir coverage/
```

#### 6. Commit with Clear Messages

```bash
# Commit with descriptive messages
git commit -m "Feature: implement soil nutrient cycling

- Add Nutrient struct to track N, P, K
- Implement decomposition rates based on moisture
- Add microbial activity modifier
- Include tests for nutrient balance"
```

#### 7. Push & Open a Pull Request

```bash
git push origin feature/your-descriptive-name
```

Then open a PR on GitHub with:
- **Title**: Clear, descriptive (e.g., "Add soil nutrient cycling simulation")
- **Description**: 
  - What does this PR do?
  - Which game mechanics does it affect?
  - Any new dependencies added?
  - Links to related issues/discussions

#### 8. Code Review

- A reviewer will evaluate your code against SOLID principles, performance, and documentation
- Address feedback constructively
- Updates trigger automatic test runs

#### 9. Testing & Integration

- Once review passes, tests are run automatically
- Integration tests validate multi-module behavior
- Performance benchmarks check for regressions

#### 10. Merge & Deploy

- Maintainers merge when all checks pass
- Your contribution is included in the next release!

---

## Areas of Need

### 🌱 Simulation Systems

**What we need**: Help refine and implement core ecological mechanics

- **Water hydrology**: Improve flow algorithms, infiltration, groundwater dynamics
- **Soil simulation**: Nutrient cycling, decomposition, organic matter accumulation
- **Organism models**: Expand plant growth, pollinator behavior, microbial simulation
- **Emergence**: Design unexpected interactions that create engaging gameplay

**Good issues**: Search for `simulation`, `water`, `soil`, `ecosystem`

### 🎨 UI & Visualization

**What we need**: Make the game intuitive and visually clear

- **Hexagon rendering**: Sprite optimization, isometric perspective
- **Terrain visualization**: Color/texture to convey soil quality, moisture, vegetation
- **UI panels**: Status readouts, character assignment UI, goal tracking
- **Feedback systems**: Visual indicators when swales work, organisms appear, etc.

**Good issues**: Search for `ui`, `rendering`, `graphics`

### 📖 Scenario Design

**What we need**: Engaging, educationally-valuable restoration scenarios

- **Write scenarios**: Create realistic restoration challenges with compelling stories
- **Balance them**: Ensure win conditions are achievable but require strategy
- **Co-design with players**: Gather feedback and iterate

**Good issues**: Search for `scenario`, `design`

### 🧪 Testing & QA

**What we need**: Comprehensive test coverage and performance validation

- **Write unit tests**: Additional coverage for simulation systems
- **Integration tests**: Multi-module scenarios (soil + water interactions, etc.)
- **Performance benchmarks**: Scaling tests (100s, 1000s of tiles/entities)
- **Play-test**: Try scenarios, report bugs and UX issues

**Good issues**: Search for `testing`, `bug`, `performance`

### 📚 Documentation

**What we need**: Clear guides for players, developers, and contributors

- **Game guide**: How to play, strategy tips, tutorial
- **Architecture docs**: Expand ARCHITECTURE.md with examples
- **API docs**: Expand rustdoc comments in source code
- **Tutorials**: Getting started guides for specific systems

**Good issues**: Search for `documentation`, `docs`

### 🎓 Permaculture & Ecology

**What we need**: Domain expertise to ensure game accuracy

- **Permaculture techniques**: Feedback on swale design, guilds, polycultures
- **Ecological accuracy**: Validate organism interactions, seasonal behavior
- **Research integration**: Connect game to peer-reviewed restoration science
- **Educational content**: Help make the game a learning tool

**Good issues**: Search for `ecology`, `education`

---

## Code Standards

### Rust Edition & Version

- **Edition**: 2021
- **Minimum Supported Rust Version (MSRV)**: 1.94
- **Channel**: Stable (not nightly)

### Naming Conventions

- **Functions & variables**: `snake_case` (e.g., `calculate_soil_moisture()`)
- **Types & traits**: `PascalCase` (e.g., `SoilComposition`, `Organism`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_INFILTRATION_RATE`)
- **Private items**: Prefix with context if unclear (e.g., `_internal_buffer`, `decompose_internal()`)

### Documentation

**All public items must be documented**:

```rust
/// Calculates the infiltration rate of water into soil.
///
/// # Arguments
/// * `soil_type` - Soil classification (Sand, Loam, Clay)
/// * `moisture_saturation` - Current moisture as fraction (0.0 to 1.0)
///
/// # Returns
/// Infiltration rate in mm/hour
///
/// # Example
/// ```
/// let rate = calculate_infiltration_rate(SoilType::Loam, 0.5);
/// assert!(rate > 0.0);
/// ```
pub fn calculate_infiltration_rate(soil_type: SoilType, moisture_saturation: f32) -> f32 {
    // Implementation
}
```

**Document intent for complex private logic**:

```rust
// Decompose organic matter based on temperature and moisture.
// Decomposition rate increases exponentially with moisture up to saturation,
// then decreases (anaerobic conditions). Temperature follows Q10 model.
fn _decompose_organic_matter(organic_matter: f32, temp: f32, moisture: f32) -> f32 {
    // Implementation
}
```

### Error Handling

Use `Result<T, E>` for fallible operations; use `thiserror` for custom error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SoilError {
    #[error("Invalid soil depth: {0}")]
    InvalidDepth(f32),
    
    #[error("Nutrient deficiency prevents plant growth")]
    NutrientDeficiency,
}

pub fn validate_soil(soil: &Soil) -> Result<(), SoilError> {
    if soil.depth < 0.0 {
        return Err(SoilError::InvalidDepth(soil.depth));
    }
    Ok(())
}
```

**Panic**: Only panic if the caller made an error (invalid state):

```rust
// OK: Caller made an error (passed invalid index)
pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
    &self.tiles[y * self.width + x]  // Panics if OOB; caller's fault
}

// Not OK: Simulation error; return Result instead
pub fn update_soil(&mut self) -> Result<(), SoilError> {
    // Validate and return errors, don't panic
}
```

### Testing

**Unit test expected locations**: `tests/` folder or alongside code with `#[cfg(test)]` modules

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infiltration_sandy_soil_high_moisture() {
        let rate = calculate_infiltration_rate(SoilType::Sand, 0.9);
        assert!(rate > 0.0);
        assert!(rate < 100.0);
    }
    
    #[test]
    fn test_nutrient_cycle_balance() {
        let mut soil = Soil::default();
        soil.add_nitrogen(10.0);
        soil.decompose(1.0);  // Partial decomposition
        assert!(soil.nitrogen > 0.0);
    }
}
```

**Coverage target**: 90% minimum (100% aspirational)

### Performance Considerations

- **Function-first approach**: Prioritize correctness and clarity
- **Optimize late**: Profile before optimizing; leave `// TODO: optimize` comments
- **Zero-cost abstractions**: Use Rust's type system to your advantage
- **Allocation concerns**: Pre-allocate buffers where simulations are cpu-heavy

### SOLID Principles

- **Single Responsibility**: Each module/struct handles one job
- **Open/Closed**: Use traits for extensibility
- **Liskov Substitution**: Trait implementations must honor contracts
- **Interface Segregation**: Fine-grained traits preferred over monolithic ones
- **Dependency Inversion**: Depend on abstractions, not concrete types

Example:

```rust
// Good SOLID design
trait SoilSimulator: Send + Sync {
    fn update(&mut self, delta_time: f32) -> Result<(), SimulationError>;
}

struct NutrientCycleSimulator {
    decomposition_rate: f32,
}

impl SoilSimulator for NutrientCycleSimulator {
    fn update(&mut self, delta_time: f32) -> Result<(), SimulationError> {
        // Implementation
        Ok(())
    }
}

// Usage: depends on trait, not concrete type
fn run_simulation(simulator: &mut dyn SoilSimulator) {
    simulator.update(0.016)?;  // 16ms frame
}
```

---

## Submission Process

### Pull Request Checklist

Before submitting, ensure:

- [ ] Code follows style guidelines and passes `cargo clippy`
- [ ] Code is formatted with `cargo fmt`
- [ ] Changes are tested with appropriate unit and integration tests
- [ ] Coverage remains ≥90% (or improved)
- [ ] Documentation is complete (public APIs documented)
- [ ] Commit messages are clear and descriptive
- [ ] No merge conflicts with `main` branch
- [ ] Related issues are linked in PR description

### Review Process

1. **Automated checks run**: Linting, formatting, tests, coverage
2. **Human review**: Maintainers evaluate design, SOLID compliance, performance
3. **Feedback & iteration**: Requested changes are addressed by contributor
4. **Approval**: Once all checks pass and review approves, PR is merged

### Timeline Expectations

- Small fixes: ~1-3 days for review
- Features: ~3-7 days for review and iteration
- Complex systems: ~1-2 weeks for thorough review

We appreciate your patience!

---

## Communication

### Questions & Discussion

- **GitHub Discussions**: General questions, feature brainstorming, design feedback
- **GitHub Issues**: Bug reports, task assignments, specific problems
- **README**: Overview and quick reference
- **ARCHITECTURE.md**: Technical design details

### Reporting Bugs

If you find a bug:

1. Check [existing issues](https://github.com/Kelmoir/regrowth/issues) to avoid duplicates
2. Open a new issue with:
   - **Title**: Clear bug description
   - **Steps to reproduce**: Exact steps to trigger the bug
   - **Expected behavior**: What should happen
   - **Actual behavior**: What actually happened
   - **Environment**: OS, Rust version, etc.

### Feedback on Design

We welcome feedback on game design, mechanics, and permaculture accuracy:

- Start a GitHub Discussion
- Include examples or data supporting your feedback
- Be constructive and collaborative

---

## License

By contributing to Regrowth, you agree that your contributions will be licensed under the [MIT License](./LICENSE). See the license file for details.

---

## Acknowledgments

Thank you for contributing to Regrowth! Your efforts help build a game that explores ecological restoration and permaculture in a meaningful, engaging way.

Happy coding! 🌱
