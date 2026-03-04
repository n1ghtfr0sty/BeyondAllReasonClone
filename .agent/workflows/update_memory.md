---
description: Automatically update MEMORY.md and project documentation.
---
# Workflow: Update Memory

When the user asks you to "update memory", run this workflow to ensure that the project's state documentation is accurate.

1. **Review Recent Changes**: Look at the work completed in the recent task/conversation summary.
2. **Update MEMORY.md**: If the high-level goal, major milestones, or constraints have changed, update `MEMORY.md`.
3. **Update Architecture/Tech Stack**: E.g., if a new library is added, ensure `docs/tech_stack.md` is updated. If a new major system (like ECS or Pathfinding) is implemented, update `docs/architecture.md`.
4. **Pruning**: Remove outdated information from these docs to keep them readable and accurate.
