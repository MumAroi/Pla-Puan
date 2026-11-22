---
name: pla-puan-bevy
description: Develop and review the Pla-Puan Rust and Bevy 2D top-down pixel art game using feature plugins, ECS, and practical clean code. Applies to this project only.
---

# Pla-Puan architecture

## Project direction

- Build a 2D top-down pixel art game in Rust with Bevy.
- Online co-op with friends is a product requirement. Teach and implement it incrementally; do not assume the game will remain single-player.
- Read Cargo.toml and the affected code before changing implementation. Match the project's Bevy version; check official documentation for uncertain APIs. Do not upgrade dependencies as part of unrelated work.
- Explain choices in Thai using plain language. Introduce ECS terms when needed; the user is learning Bevy.

## Feature-based organization

Start with a single Cargo package and one module per implemented feature:

```text
src/
  main.rs       # Application setup and plugin composition
  player.rs     # PlayerPlugin, player data and behavior
  camera.rs     # CameraPlugin, camera setup and following
  world.rs      # WorldPlugin, map and world objects
assets/
  sprites/
  tilesets/
  audio/
  fonts/
```

This is a growth guide, not a requirement to create unused files or directories. Add UI, audio, loading, or other modules when a feature needs them.

- Keep main.rs focused on application configuration and registering plugins.
- A feature plugin registers its systems, resources, state hooks, and communication types. Keep gameplay work in systems rather than the plugin's build method.
- Keep a feature's components and systems together initially. When responsibilities become difficult to navigate, turn player.rs into player/mod.rs with focused files such as movement.rs and animation.rs. Do not keep both module entry forms.
- Keep implementation details private; expose only the types other features actually use.
- Avoid a project-wide components/ or systems/ dumping ground. Extract shared code only when multiple features need the same responsibility.
- Do not introduce a workspace, generic engine layer, dependency injection framework, or repository/service layers without a concrete requirement.

## ECS responsibilities

- Entities identify game objects. Components hold per-entity data; resources hold genuinely shared data. Systems implement behavior through explicit queries and parameters.
- Use Bevy's existing components, such as Transform, where they represent the authoritative value. Avoid duplicate position or state fields unless synchronization has an explicit purpose.
- Query only the data a system needs. Keep dependencies between features visible through small shared components, resources, or the Bevy version's appropriate message/event mechanism.
- Add explicit system ordering when one system depends on another's results. Do not rely on registration order or chain unrelated systems.
- Use state conditions when menus, loading, pausing, or gameplay require them. Scope spawning and cleanup to the correct lifecycle so re-entering a state does not duplicate entities.
- Use frame delta time for continuous movement. Use a fixed schedule when simulation requires a fixed step, and keep input, simulation, and rendering timing deliberate.

## Online co-op boundaries

- Defer Host/Client implementation until the create-room/join-room learning milestone. Keep the already installed Lightyear dependency, but do not restore startup networking modes or room scaffolding before that step.
- Player marks any player character, not exclusively the local one. Filter local keyboard input and camera targeting by explicit local control; do not drive all Player entities from the same keyboard.
- Keep local input collection separate from shared movement/gameplay simulation when introducing multiplayer behavior. Remote characters must be driven by the chosen networking authority, not local keyboard input.
- Distinguish player identity, ownership, and simulation authority. Do not use raw local Entity IDs as stable cross-machine identity; use the networking layer's entity mapping or explicit network identifiers.
- Host-authoritative co-op is a candidate, not a finalized decision. Select topology and networking dependencies when implementing a concrete networking milestone, checking Bevy version compatibility first.
- Work one learning step at a time. Do not add networking dependencies, prediction, rollback, lobbies, or matchmaking merely to prepare for later. Clearly label local-only prototypes as such.

## Practical clean code

- Prefer the smallest complete implementation of the requested behavior. Keep unrelated refactors out of the change.
- Give functions and types names that describe gameplay intent: move_player, MovementSpeed, Facing. Avoid vague managers, helpers, and utils modules.
- Give each system a coherent responsibility. Split it when responsibilities change for different reasons, not to satisfy a line limit.
- Prefer direct Rust code and composition. Add traits, generic abstractions, and configuration only when concrete callers need them.
- Keep values local until they are truly shared. Name gameplay tuning values and make units clear; use a configuration resource only when runtime or shared configuration is needed.
- Remove demonstrated duplication while keeping domain differences explicit. Do not merge similar-looking behavior with different rules merely to reduce lines.
- Handle expected failures explicitly. Avoid unwrap/expect in runtime paths unless the invariant is established and the failure policy is intentional; do not silently ignore errors.
- Comment on reasons, invariants, or constraints that the code cannot express. Avoid narrating obvious code.
- Avoid unnecessary clones, allocations in frequent systems, and broad mutable access. Optimize further when measurement or a concrete bottleneck justifies it.

## Pixel art behavior

- Use nearest-neighbor texture sampling for pixel art. Choose consistent asset scale and camera behavior; nearest sampling alone does not guarantee pixel-perfect rendering.
- Establish logical resolution and scaling policy when implementing rendering. Do not treat the previously suggested 16-pixel tiles or character sizes as approved requirements.
- For top-down overlap, define a consistent depth convention based on each object's ground/feet position. Reserve intentional layers for terrain and overlays, and avoid conflicting depth systems.
- Preserve smooth simulation data when implementing visual pixel snapping; do not round authoritative positions every frame.

## Verification and completion

- For Rust changes, run cargo fmt --check and cargo check; run cargo clippy and targeted tests when relevant and available. Resolve warnings introduced by the change without sweeping unrelated fixes.
- Add small behavioral tests for nontrivial rules, calculations, and regressions. Do not test framework boilerplate or mirror implementation details.
- When visual behavior changes and runtime access is available, check the actual game: movement, camera, pixel scaling, overlap, or state transitions as affected.
- For instruction-only changes, validate the skill rather than building the game.
- Report what changed, what was verified, and any unverified behavior. Do not claim runtime or visual verification from a successful compilation.
