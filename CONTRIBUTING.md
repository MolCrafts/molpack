# Contributing to molcrafts-molpack

Thanks for your interest in contributing. This document covers how to set up a
development environment, run tests, and get a PR merged.

## Development environment

**Prerequisites:**
- Rust 1.91+ (`rustup update stable`)
- Python 3.12+ with `maturin` and `pytest` for the Python bindings (the wheel's `requires-python` is `>=3.12`)
- The [molrs](https://github.com/MolCrafts/molrs) repo checked out as a sibling:

```
workspace/
├── molrs/      ← git clone https://github.com/MolCrafts/molrs
└── molpack/    ← this repo
```

The root `Cargo.toml` uses a path dependency on `../molrs/molrs-core`. With the
sibling layout above everything resolves automatically.

**Version pins:** path molrs / PyPI `molcrafts-molrs` / `molcrafts-molpy` are
fixed at **0.9.3** (see `Cargo.toml`, `python/pyproject.toml`, and
`MOLRS_GIT_REF` in `.github/workflows/ci.yml`). Keep the sibling molrs clone
on that version line (`git checkout v0.9.3` or the matching release branch).

**First-time setup:**

```bash
# Fetch test data used by the regression suite
bash ../molrs/scripts/fetch-test-data.sh

# Lint + pre-push CI parity hooks (rust + isolated python tests)
pre-commit install

# Verify the Rust build
cargo build -p molcrafts-molpack
```

## Running tests

CI and pre-push share the same scripts under `scripts/`:

```bash
# Same as the CI "rust tests" job (not the long packmol regression)
bash scripts/run-rust-tests.sh

# Isolated throwaway venv: non-editable molrs (sibling) + molpack wheel + pytest
bash scripts/run-python-tests.sh

# Packmol regression suite (requires test data, slow; CI master only)
cargo test --release --features io --test examples_batch -- --ignored
```

For a quick local edit loop you may still `maturin develop` in a personal
venv; **do not** rely on that for the gate — pre-push always uses the
isolated script above.

## Code style

- `cargo fmt` / `clippy` / `ruff` / `ty` — commit-stage pre-commit hooks
- pre-push runs `scripts/run-{rust,python}-tests.sh` (same as CI)
- Follow the immutability rule: return new values, never mutate in place
- Follow the immutability rule: return new values, never mutate in place
- Keep files under ~400 lines; split at ~200 if the module grows beyond one concern
- New public types must implement `Debug` and, where appropriate, `Clone`

## Adding a new restraint type

1. Add a struct under `src/restraint/` with semantically-named fields — alongside the existing geometric restraints in `src/restraint/geometric/`, or as a new submodule (see `src/restraint/profile/` for a composed example)
2. Implement `Restraint` (both `f` and `fg`; `fg` must match the gradient of `f`)
3. Re-export it from `src/restraint/mod.rs`, then from the crate root in `src/lib.rs`
4. Add a unit test in `tests/restraint.rs`
5. Document it in `docs/concepts.md` under the restraint table

See the `extending` rustdoc chapter (`cargo doc --open`) for detailed tutorials.

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <short description>

<optional body>
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`, `ci`

## Pull request checklist

All items in the PR template must be checked before requesting review. CI must
be green. For restraint or objective changes, the Packmol regression suite
(`examples_batch`) must also pass.

## Licensing

By contributing you agree that your contributions will be licensed under
the BSD-3-Clause license that covers this project.
