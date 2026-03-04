# Agent Memory

## High-Level Goal
Build a clone of "Beyond All Reason", a large-scale real-time strategy (RTS) game.

## Key Constraints & Definitions
- **Engine**: Bevy (Rust)
- **Architecture**: Strict strict separation of simulation and rendering. Deterministic Lockstep networking model.
- **Focus**: Large-scale battles (10,000+ units), economy management, and deterministic gameplay.
- **Communication Style:** When making code changes, always add comments structured as if explaining the concepts to a student.

## Pointers
- **Architecture**: See `docs/architecture.md` for details on game simulation, ECS, networking, etc.
- **Tech Stack**: See `docs/tech_stack.md` for language and library choices.
- **Workflows**: See `.agent/workflows` for custom agent behaviours.
