# Contributing to molcrafts-molpack

Use the Rust toolchain in `rust-toolchain.toml`, Python 3.12 and uv. Clone
`MolCrafts/molpy` next to this repository as `../molpy`, then check out the
exact `MOLPY_REF` in `.github/partners.env`. The native API requires molpy
0.20.1 on the 0.20 line. Both wheels must embed that same source and compiler;
foreign frame or region capsules raise `ValueError` when resolved.

```text
workspace/
├── molpy/
└── molpack/
```

## Setup and local checks

Run these commands from the repository root:

```bash
uv sync --project python --locked --group dev --no-install-project --no-install-package molcrafts-molpy
uv run --project python molci doctor --install-hooks
uv run --project python molci verify
```

`molci verify` is the pre-push gate. It reuses successful checks only while
source, dependencies, tools and built artifacts still match. `--full` reruns
every applicable check. For an edit loop, select one gate:

```bash
uv run --project python molci run rust
uv run --project python molci run python
uv run --project python molci run types
uv run --project python molci run docs
```

The shared `molpack` profile in MolCrafts CI owns the commands. The Rust gate
runs unit tests and doctests, checks examples and feature combinations, and
compiles the CLI without dev dependencies. The Python gates build one
non-editable molpy wheel and one molpack wheel from the pinned checkout;
pytest, typing and documentation use those same wheels. The sibling must be
clean and at the pinned revision; local checks never move it.

## Hooks and CI

`.pre-commit-config.yaml` installs only the shared `molci-commit` and
`molci-push` hooks from the immutable `CI_REF`. Pre-commit runs format, Ruff,
hygiene and workflow checks. Pre-push runs the complete native verification.
Never bypass hooks with `--no-verify`, and never merge a red pull request.
Push each committed snapshot from its own clean worktree.

`test.yml` calls the shared context, runs native verification and the platform
lanes, and ends in the required `test / result` check. Linux verifies all gates;
macOS and Windows verify the built Python wheels. A fork push does not repeat
the upstream PR matrix. `release.yml` requires successful verification of the
exact upstream master commit before publishing. Preparing a change creates
no release tag and publishes no package.

`MOLCRAFTS_HOOK_RUNNER`, when configured, must synchronously verify the exact
source snapshot on the execution node. No repository-local hook launcher or
shell test script is needed.

## Dependency pins

`.github/partners.env` pins molpy and the shared CI implementation. Cargo and
uv lock files record package metadata. To update molpy, use a matching sibling
worktree, change `MOLPY_REF`, and regenerate the locks:

```bash
cargo update -w
cargo update --manifest-path python/Cargo.toml -w
uv lock --directory python
```

Publish a shared CI revision before updating consumers. The existing command
updates Actions, hooks, the CI pin and the locked Python dependency together:

```bash
uv run --project python molci partners pin-ci <full-published-commit-sha>
```

## Code style

Rust behavior is tested next to its implementation. New public types implement
`Debug` and, where appropriate, `Clone`. Run the shared format, Clippy, Ruff and
type gates before pushing. Keep new units focused and reuse existing APIs.

## Adding a new restraint type

1. Add a struct under `src/restraint/` with semantically-named fields — alongside the existing geometric restraints in `src/restraint/geometric/`, or as a new submodule (see `src/restraint/profile/` for a composed example)
2. Implement `Restraint` (both `f` and `fg`; `fg` must match the gradient of `f`)
3. Re-export it from `src/restraint/mod.rs`, then from the crate root in `src/lib.rs`
4. Add a `#[cfg(test)]` unit test next to the struct — the restraint owns its own tests
5. Document it in `docs/concepts.md` under the restraint table

See the `extending` rustdoc chapter (`cargo doc --open`) for detailed tutorials.

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <short description>

<optional body>
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`, `ci`
