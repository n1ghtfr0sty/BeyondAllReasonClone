# Agent Memory

## High-Level Goal
Build a multiplayer "Beyond All Reason" / "Earth 2150" inspired large-scale real-time strategy (RTS) game.
The initial gameplay iteration will focus on a PvE wave-defense style against alien creatures, where players feel very powerful fighting hordes of enemies.

## Key Constraints & Definitions
- **Engine**: Bevy (Rust)
- **Architecture**: Strict strict separation of simulation and rendering. Deterministic Lockstep networking model.
- **Focus**: Large-scale battles (10,000+ units), economy management, and deterministic gameplay.
- **Unit Design**: Units must be highly modular. Players should be able to design their units by combining frames and weapons.
- **Communication Style:** When making code changes, always add comments structured as if explaining the concepts to a student.

## Pointers
- **Architecture**: See `docs/architecture.md` for details on game simulation, ECS, networking, etc.
- **Tech Stack**: See `docs/tech_stack.md` for language and library choices.
- **Workflows**: See `.agent/workflows` for custom agent behaviours.
