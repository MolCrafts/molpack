# molpack — project instructions

molpack builds initial molecular configurations through a Rust library,
Packmol-compatible `.inp` CLI and PyO3 Python wheel. Solvers construct geometry;
users supply chemical information, priors and downstream force fields.

This file is the sole project instruction source. Keep it current with the code;
do not recreate separate agent rulebooks, debt ledgers or historical spec trees.
User instructions take precedence.

## Architecture and ownership

- `src/pipeline/`: run lifecycle, stage composition and callback identity.
- `src/stage.rs`: geometry-only stage contract; `src/pipeline/engine.rs`: engine settings
  and factories. Solvers are peers and never invoke another solver's driver.
- `src/pack/`: rigid packing, initial placement, bad-move heuristics and GENCAN.
- `src/grow/`: continuous and lattice growth. Shared lifecycle stays in pipeline.
- `src/system/`: mutable run state, rigid views and neighbor-grid context.
  `src/state.rs`: frozen public outcome; do not add another live-state facade.
- `src/target.rs`: per-target data; `src/template.rs`: template coordinate and
  rotatable-bond policy. Graph, frame, region and geometry primitives belong to
  `molrs`; use its APIs instead of creating parallel types.
- `src/restraint/`: lift molrs regions and evaluate restraints.
  `src/objective.rs`: the shared objective; `src/invariant.rs`: post-stage checks.
- `src/optimizer/`: in-loop optimizer seam, available without a separate `ff`
  feature. User-supplied potentials own chemistry and energy.
- `src/script/`: `.inp` parsing, lowering and optional format IO.
  `src/bin/molpack/`: CLI; `src/assemble.rs`: output frame assembly.
- `python/src/`: PyO3 bindings; `python/python/molpack/`: Python package;
  `python/tests/`: binding tests; `docs/`: Zensical documentation.

## Engineering rules

- One concept, one authoritative implementation. Search local code, molrs and
  dependencies before adding helpers, types, public APIs or dependencies.
- Keep policy, state ownership, ordering and errors explicit. Do not hide failed
  work behind empty/default results, silent fallback or compatibility shims.
- Keep changes local and dependency direction acyclic. Add abstraction only for
  demonstrated consumers; do not add a parallel API or speculative extension.
- Solvers consume topology, coordinates, radii and caller-supplied priors. The
  user chooses the method; unsupported combinations return named errors.
- All-atom and coarse-grain targets are equally supported. Never invent atomic
  numbers for coarse-grain beads. Exclusions and rotatable bonds are target data.
- Regions belong to molrs. Molpack lifts them into restraints without defining
  another region language or primitive family.
- Public identifiers use `molpack`, never `packmol`; prose may mention Packmol.
- `.inp` is the only script configuration language. Do not add TOML/YAML/JSON
  configuration surfaces or per-structure solver selection keywords.
- Public outcome is `State`; mutable run state is `PackState`. Stage verdicts use
  the shared objective. Report exhausted budgets honestly as unconverged.
- Rust edition 2024. Follow `rust-toolchain.toml` for the actual compiler.
  New public types implement `Debug` and, where meaningful, `Clone`.
- Prefer immutable configuration and explicit mutable runtime ownership.
  Aim for 200–400 lines per file, at most 800; split by responsibility.

## Dependencies and Python interop

- Required sibling: `../molpy`, repository `MolCrafts/molpy`.
  Cargo uses `../molpy/crates/molrs`; Python also uses `molrs-ffi`.
- `.github/partners.env` records immutable molpy and MolCrafts CI revisions.
  Keep Cargo/uv metadata and `rust-toolchain.toml` consistent with that source.
  Verification rejects a mismatched or dirty sibling; never change a user's
  checkout automatically to satisfy a check.
- Publish shared infrastructure before pinning it in a consumer. Use
  `molci partners pin-ci <sha>` to update Actions, hooks and the CI dependency
  together, including `python/uv.lock`.
- `.cargo/config.toml` shares `../molpy/target` across Rust and Python builds.
  Use an explicit `CARGO_TARGET_DIR` when an isolated worktree needs that cache.
- Features: no library defaults; `io` enables format IO, `cli` adds clap + IO,
  `rayon` enables parallel evaluation. There is no molpack `ff` feature.
  The wheel enables `rayon` and loads structures through molpy, without `io`.
- Python objects come from `molpy.core`. Frame and region handles cross the
  extension boundary through `molrs_ffi` capsules. Always obtain names from
  `molrs_ffi::abi`; never hard-code a capsule name or add a version handshake.
  Names fingerprint the source, compiler and handle layout, so both wheels
  must be built from matching molpy sources and toolchains.
- Reuse `FrameRef` accessors for shared ownership and `Frame::deep_copy` for
  independent frames. Keep unsafe pointer handling at the existing capsule seam.

## Verification and Git workflow

- The pinned MolCrafts CI `molpack` profile owns gate commands. `test.yml` and
  the two remote hooks (`molci-commit`, `molci-push`) invoke the same runner.
  `release.yml` owns guarded publishing. Do not add tox, local shell launchers
  or duplicate workflow check definitions.
- Run `molci doctor --install-hooks` to configure local hooks, and `molci verify`
  before pushing. Reuse only receipts/artifacts matching the submitted source.
  Never bypass commit/push hooks or push a different/uncommitted source snapshot.
- Tests belong to the behavior's owner: Rust in-module unit tests, Rust doctests
  and Python binding tests. Use RED → GREEN for defects. Preserve meaningful
  assertions; lifecycle tests use existing stage doubles when no solver result
  is under test. Do not add ad hoc e2e/regression/benchmark infrastructure.
- Focused Rust tests: `cargo test --locked --lib --features cli <filter>`.
  Full verification includes formatting, Clippy, Rust features/standalone CLI,
  locked dependencies, matching non-editable wheels, pytest, types and strict docs.
- `origin` is the Roy-Kid fork; `upstream` is MolCrafts. Work on a feature branch
  and open a PR. Never push directly to upstream master, force-push or rebase an
  upstream branch. Do not merge or publish without the user's instruction.

## Scope of future work

Per-chain recoverable growth softening and redesigned packing-quality
measurements require their own design and acceptance criteria. They are future
features, not promises of the current solver. Do not revive obsolete tox/pin
failures or old ABI minor-line assumptions as active debt.
