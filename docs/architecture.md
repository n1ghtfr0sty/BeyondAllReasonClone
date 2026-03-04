# Game Architecture

*This document will track the high-level architecture of our Beyond All Reason clone.*

## Core Simulation
- **ECS (Entity Component System)**: Bevy ECS. The simulation must run on a fixed timestep independent of the rendering frame rate to maintain determinism.
- **Unit Design**: High modularity. Units aren't monolithic blocks. They consist of composed parts (e.g., a "Frame" and one or more "Weapons"). In ECS, this likely maps to a core Unit Entity with parent-child relationships linking to weapon/module Entities, or composing all stats iteratively on a single base entity.
- **Networking**: Deterministic Lockstep. Only player commands are sent over the network. The simulation must use fixed-point math (or strictly controlled floating-point behavior if possible in Rust, though fixed-point is safer) to ensure all clients simulate the exact same state.
- **Pathfinding**: Flow fields / Vector fields for massive unit counts, combined with local avoidance (e.g., Boids or deterministic RVO).

## Graphics/Rendering
- **Engine**: Bevy's default wgpu-based renderer.
- **State Interpolation**: Visual transforms will interpolate between the previous and current fixed simulation ticks to provide smooth rendering despite a fixed timestep simulation.
- **Camera**: Top-down RTS camera (`PanOrbitCamera`) driven by a decoupling of standard inputs (WASD + Scroll Wheel).

## Audio
- **Engine**: Bevy's built-in audio system (or `bevy_kira_audio` if more advanced features are needed later).
