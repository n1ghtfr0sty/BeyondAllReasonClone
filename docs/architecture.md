# Game Architecture

*This document will track the high-level architecture of our Beyond All Reason clone.*

## Core Simulation
- **ECS (Entity Component System)**: Bevy ECS. The simulation must run on a fixed timestep independent of the rendering frame rate to maintain determinism.
- **Networking**: Deterministic Lockstep. Only player commands are sent over the network. The simulation must use fixed-point math (or strictly controlled floating-point behavior if possible in Rust, though fixed-point is safer) to ensure all clients simulate the exact same state.
- **Pathfinding**: Flow fields / Vector fields for massive unit counts, combined with local avoidance (e.g., Boids or deterministic RVO).

## Graphics/Rendering
- **Engine**: Bevy's default wgpu-based renderer.
- **State Interpolation**: Visual transforms will interpolate between the previous and current fixed simulation ticks to provide smooth rendering despite a fixed timestep simulation.

## Audio
- **Engine**: Bevy's built-in audio system (or `bevy_kira_audio` if more advanced features are needed later).
