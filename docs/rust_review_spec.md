# Rust Codebase Review & Improvement Spec

Status: proposal (no code changes attached — this document is the deliverable)
Scope: the Rust portion of `molpack` (`src/`, `python/src/`), ~11.7k LOC
Goal: improve **architecture**, **performance**, and **maintainability**
(more human-readable, easier to extend) without regressing Packmol parity.

This is a planning document. Each item lists location, the problem, the
proposed change, and risk. Items are grouped into three priority tiers at
the end. Nothing here changes behavior unless a tracking PR implements it.

---

## 0. Overall assessment

The crate is a high-quality Fortran→Rust port. Strengths worth preserving:

- CI enforces `cargo clippy --all-targets --features cli -- -D warnings` and
  `cargo fmt --check`, so the tree is already clippy-clean; routine idiom
  fixes are not needed.
- Deliberate performance design: `AtomProps` AoS layout with a compile-time
  40-byte size assertion (`src/context/pack_context.rs:69`), precomputed PBC
  reciprocals, `any_short_radius` / `any_fixed_atoms` summary flags that skip
  cold branches, precomputed cell-list neighbors, and an opt-in rayon path
  gated by a 4096-atom merge threshold.
- Strong documentation: four rustdoc chapters (concepts / architecture /
  extending / getting_started), `docs/packmol_parity.md`, and cache-line-level
  comments on hot structs.
- Essentially **no `unsafe`**; all indexing is bounds-checked.
  `PackContext::debug_assert_atom_props_sync` is a professional mirror-state
  drift check.

Consequently the recommendations below are **refinements** — reducing
cognitive load and closing robustness gaps — not bug-driven rewrites. The
issues cluster into: God-object coupling, hot-kernel duplication, residual
Fortran naming, a few robustness gaps in the parser / FFI layer, and a
cross-cutting frontend capability mismatch.

---

## 1. Architecture

### 1.1 `PackContext` is a God-object

`src/context/pack_context.rs` — `PackContext` carries ~50+ fields spanning
atom geometry, linked-cell state, constraint machinery, gradient buffers,
scaling/penalty state, PBC params, counters/flags, and the output frame.

Consequences:
- Any mutation of per-atom props requires manual `sync_atom_props()`, guarded
  by a debug assertion (the guard's existence signals the coupling risk).
- The `Objective` trait abstracts the eval entry point but does not decouple
  state — kernels still read/write the whole context.
- Signature explosion: `run_iteration` takes **19 parameters**
  (`src/packer.rs:746`), about half `&mut` borrows threaded through.

Proposed change:
1. Split `PackContext` by lifetime/role into sub-structs held as fields, e.g.
   `Topology` (immutable: `nmols`/`natoms`/`idfirst`/`coor`/`dmax`),
   `CellGrid` (`latom*`/`lcell*`/`neighbor_cells_*`),
   `EvalState` (`xcart`/`radius`/`fscale`/`fdist`/`frest`),
   `PbcParams`. Callers can then borrow `&ctx.cells` / `&mut ctx.eval`,
   shrinking borrow conflicts and reader cognitive load.
2. Collect `run_iteration` / `run_phase` scalar `&mut` args (`flast`,
   `fimp_prev`, `radscale`) into an `IterationState` struct (19 → ~4 params).

Risk: medium. Field moves are mechanical but touch many call sites; do
incrementally with tests green.

### 1.2 `Constraints` is a zero-sized indirection shell

`src/constraints/container.rs` — `Constraints` is a ZST whose `evaluate()`
only dispatches by `EvalMode` to `compute_f`/`compute_fg`/`compute_g`. It
holds no state, adding one conceptual hop (`ctx.evaluate → Constraints::evaluate
→ compute_*`).

Proposed change: either inline the `EvalMode` match into
`PackContext::evaluate` (drop the `Constraints` type) or give it real state.

Risk: low.

### 1.3 Phase selection uses a bare `phase == ntype`

`src/packer.rs:945` — `is_all = phase == ntype` distinguishes the per-type
phases from the final all-type phase; the outer loop is `for phase in 0..=ntype`.
No `assert!(phase <= ntype)` guard, no named constant.

Proposed change: introduce `enum Phase { PerType(usize), AllTypes }` to make
phase semantics type-level and remove the magic comparison.

Risk: low-medium.

---

## 2. Performance

Hot-path design is already good; remaining items are minor and should be
benchmarked first (7 criterion benches exist).

| Location | Problem | Impact | Proposed change |
|---|---|---|---|
| `src/objective.rs:179` | `compute_fg` unconditionally `gxcar.fill([0.0;3])` over 3N, even when the geometry cache hits and `accumulate_pair_fg` fully overwrites | small | fill only when `!matches_cached_geometry` |
| `src/packer.rs:788` | relaxer MC loop does `sys.coor[base..].to_vec()` per runner per iteration | small–medium (relaxer is opt-in) | hoist `saved` buffer out of the loop and reuse |
| `src/packer.rs:837` | `relaxer_acceptance` Vec is built every iteration even with no handlers | small | move into `if !handlers.is_empty()` |
| `src/packer.rs:701` (`evaluate_unscaled`) | two O(ntotat) radius swap/restore passes per iteration | medium (large systems) | wrap in a RAII guard or cache |
| `src/gencan/mod.rs` | `vec![0.0;n]` and `lastgpns = vec![INFABS;1000]` allocated per `pgencan` call | small | preallocate in `GencanWorkspace` |

Parallelism assessment: the rayon paths (`accumulate_pair_f_parallel` /
`_fg_parallel`) use thread-local reductions with no false sharing and a sane
threshold guard — correct, no action. CG's finite-difference Hessian-vector
product is serial by Fortran-port design, not a Rust-specific defect.

---

## 3. Maintainability (primary focus: readable & extensible)

### 3.1 ⭐ Five near-duplicate hot kernels (highest priority)

`src/objective.rs` has five pair kernels:
`fparc` (L218), `gparc` (L719), `fgparc` (L788), `fgparc_stats` (L887),
`fparc_stats` (L974). Their distance math, overlap penalty, and short-radius
penalty are byte-identical except for variable names and which of f/g and
which buffer they write. The short-radius block is copied five times — a
single formula change must be applied in five places.

Proposed change: factor the penalty value/gradient math into one
`#[inline(always)]` helper that all five kernels call; or parameterize a
single kernel over (accumulate-f, accumulate-g, mutable-vs-readonly) via
generics/marker types. Guard with the `pair_kernel` bench to hold performance.

Risk: medium (hot path) — bench-gated.

### 3.2 ⭐ Residual Fortran naming

Many Fortran abbreviations survive, forcing readers to consult the Fortran
source. Suggested renames (keep a `// Packmol: <old>` comment for traceability
with `docs/packmol_parity.md`):

| Current | Meaning | Suggested |
|---|---|---|
| `fparc`/`gparc`/`fgparc` | pair kernel ("parc" unexplained) | `pair_f_atom`/`pair_g_atom`/`pair_fg_atom` |
| `*_stats` | actually parallel/read-only variants | `*_parallel` |
| `ilubar`/`ilugan` | COM / Euler segment offsets into `x` | `x_com_offset`/`x_euler_offset` |
| `beta`/`gama`/`teta` | Euler angles β/γ/θ (`gama` reads like a typo) | `euler_beta/gamma/theta` |
| `ibtype`/`ibmol` | atom's type / molecule index | `atom_type_idx`/`atom_mol_idx` |
| `comptype` | whether a type is computed this phase | `is_type_active` |
| `fdist`/`frest` | max distance / restraint violation | document once, or `max_dist_violation` |
| `move_flag` | movebad per-atom tracking switch | `selective_repack_mode` + doc |

Risk: low (rename + comment). Highest readability ROI.

### 3.3 Long functions & unexplained magic numbers

- `pack()` ~354 lines (`src/packer.rs:276`): split into `validate_inputs` /
  `prepare_system` / `run_main_loop`.
- `tn_linesearch` ~328 lines (`src/gencan/mod.rs:657`): split into
  `tn_extrapolation` / `tn_interpolation`.
- Name magic constants and cite the Packmol source line: kernel `4.0`
  (apparent derivative of `penalty²`), movebad trigger `10.0`
  (`src/packer.rs:770`), the `[-99.99, 99.99]` `fimp` clamp, CG `GAMMA=1e-4`
  / `THETA=1e-6`.
- CG convergence test packs four conditions on one line
  (`src/gencan/cg.rs:173`): break into named booleans.

Risk: low–medium.

### 3.4 `restraint.rs` inside/outside mirror duplication (~57%)

~535 of 945 lines are paired inside/outside mirrors differing only by
`.max`↔`.min`, `+`↔`*`, and whether `scale2` is applied. The cylinder's
~40-line gradient (`src/restraint.rs:709`) has zero comments and is written
twice. This is intentional Packmol-per-kind parity, acceptable, but hurts
extensibility.

Lightweight, API-preserving improvements:
- Add an implementation-level docstring to each of the 14 restraints (penalty
  formula + parameter meanings), especially cylinder/gaussian.
- Add a complete minimal "write your own `Restraint`" example to the module
  docstring.
- Add a `debug_assert`-based aid in `fg` to catch implementers who use `=`
  instead of the required `+=` gradient accumulation.

Risk: low.

### 3.5 Deprecated aliases

`BBox→Aabb`, `FixedPlacement→Placement`, `FromRegion→RegionRestraint` are
still publicly exported; `FromRegion`'s deprecation note is vague. Provide an
explicit migration example and schedule removal for 0.2.

Risk: low.

---

## 4. ⭐ Cross-cutting: frontend capability mismatch

A finding invisible in any single file but important for extensibility:

| Capability | Rust API | `.inp` parser | Python bindings |
|---|---|---|---|
| Restraint kinds | **14** | **~5** (inside box/sphere, outside sphere, above/below plane) | **5** |

`src/script/parser.rs` (`parse_inside`/`parse_outside`) only accepts
box/sphere; cube/ellipsoid/cylinder/gaussian and the outside-*
variants are unreachable from `.inp`. Python exposes only 5 classes. Adding a
restraint today means editing **three** places, easily missing two.

Proposed change:
- Build a single restraint registry/macro so kind → keyword → Rust
  constructor → Python class derive from one source of truth.
- Until then, explicitly mark in the README/docs keyword table which
  restraints are not yet wired into the parser/Python.

Risk: medium (touches parser + bindings; behavior-additive).

---

## 5. Robustness gaps (parser / CLI / FFI) — verified

The parser is a clean hand-rolled recursive-descent state machine with good,
context-rich error messages. The gaps below are higher priority than the
deeper layers because they sit on user input and the FFI boundary.

| Priority | Location | Problem | Proposed change |
|---|---|---|---|
| High | `python/src/packer.rs:39,55,57-59,62,89,92` | seven `.expect()` in Python-callable getters/methods; a malformed core result panics at the FFI boundary (pyo3 converts it to `PanicException` — not a clean `PyValueError`) | replace with `?` + `ok_or_else(PyValueError…)` |
| High | `src/script/build.rs:125` | 1-based→0-based via `saturating_sub(1)` silently clamps the invalid `atoms 0` to atom 0 | `checked_sub(1).ok_or(…)`, and validate `idx >= 1` in the parser |
| Medium | `src/script/parser.rs` (`parse_atom_indices`) | does not reject 0-based indices (`atoms 0` parses) | `if idx == 0 { return Err("indices must be ≥1") }` |
| Medium | `src/script/parser.rs` (`parse_inside`/`parse_outside`) | sphere radius not validated `> 0` (whereas `pbc` *is* validated — inconsistent) | reject `radius <= 0.0` |
| Medium | `python/src/target.rs` (`PyTarget::new`) | no `count > 0` check (length consistency is checked) | add `count > 0` validation |
| Low | `src/bin/molpack/main.rs:46` | `canonicalize().unwrap_or_else(...)` silently swallows symlink/permission errors | warn on failure |
| Low | `src/script/parser.rs:240-259` vs `297-328` | structure-level and atom-group-level restraint match arms duplicated | extract a helper |

Note: input validation is scattered and inconsistent (`pbc` validated,
radius not; atom indices validated in `python/src/target.rs`
`validate_atom_indices` but not on the script path). Centralize validation
into one reusable function shared by all three frontends.

Positives to preserve: the Python callback error-stashing pattern
(`Mutex<Option<PyErr>>` drained by `pack()`) and attach-time `f`/`fg` method
resolution for duck-typed restraints are both good designs.

---

## 6. Testing

- The orchestration layer (`pack` / `run_phase` / `run_iteration`) is only
  integration-tested. Add unit tests for phase transitions, the swap-state
  save/restore contract, and handler callback ordering.
- Handlers can panic without cleanup (`XYZHandler` holds a file handle; an
  `on_step` panic leaks it) and cannot surface I/O errors to `pack()`'s
  `Result`. Make handler methods return `Result` or wrap them in RAII.
- Consider property-based tests (`proptest`) for the parser.

---

## 7. Prioritized action plan

### Tier 1 — low risk, high value (do first)
1. Converge FFI panics: replace the 7 `.expect()` in `python/src/packer.rs`
   with `?`.
2. Index bug: `build.rs:125` `saturating_sub`→`checked_sub`; parser rejects
   `idx < 1`.
3. Fill validation gaps: sphere radius `> 0`, `count > 0`, non-empty index
   lists.
4. De-Fortran naming with `// Packmol:` traceability comments (§3.2).
5. Name magic constants (movebad `10.0`, kernel `4.0`, `fimp` clamp, CG
   constants) (§3.3).
6. Add implementation docstrings to the 14 restraints + a custom-restraint
   example in the module docstring (§3.4).

### Tier 2 — medium risk (bench-gated)
7. Merge the five pair kernels into one parameterized kernel (`pair_kernel`
   bench guards performance) (§3.1).
8. Split `pack()` (354 lines) and `tn_linesearch` (328 lines) (§3.3).
9. `run_iteration` 19 params → `IterationState` struct (§1.1).

### Tier 3 — architecture (medium-term)
10. Split `PackContext` into lifetime-scoped sub-structs (§1.1).
11. Single restraint registry unifying Rust API / `.inp` parser / Python (§4).
12. Drop the `Constraints` shell; type the phase with `enum Phase` (§1.2/§1.3).
13. Handler callbacks return `Result` / RAII to prevent file-handle leaks (§6).

---

## 8. Notes for implementers

- The local checkout lacks the `../molrs` path dependency, so
  `cargo build/clippy/test/bench` cannot run here. Tier 2/3 (behavior- or
  performance-affecting) changes should be done where the workspace builds,
  and bench-gated.
- Keep Packmol parity: any rename must preserve a comment pointer to the
  original Fortran symbol, and regression behavior must match
  `tests/examples_batch.rs` and `docs/packmol_parity.md`.
