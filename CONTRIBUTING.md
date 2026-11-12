# Contributing to molcrafts-molpack

Thanks for your interest in contributing. This document covers how to set up a
development environment, run tests, and get a PR merged.

## Development environment

**Prerequisites:**
- Rust 1.91+ (`rustup update stable`)
- Python 3.12+ with `maturin` and `pytest` for the Python bindings (the wheel's `requires-python` is `>=3.12`)
- [prek](https://github.com/j178/prek) for git hooks (drop-in for pre-commit)
- [uv](https://docs.astral.sh/uv/) (hooks run `uv run --group dev tox …`; tox is in
  `python/` dependency-group `dev`, not a separate global install)
- The [molpy](https://github.com/MolCrafts/molpy) repo checked out as a sibling
  directory named `molpy`:

```
workspace/
├── molpy/      ← git clone https://github.com/MolCrafts/molpy
└── molpack/    ← this repo
```

The root `Cargo.toml` uses a path dependency on `../molpy/crates/molrs`. With the
sibling layout above everything resolves automatically.

**Version pins:** a local build uses the sibling checkout `../molpy`.
The native API requires molpy **0.20.1** or later on the **0.20** line.
`.github/partners.env` pins the exact molpy commit used by every
`setup-partners` action, including releases. Check out that revision locally.
`Cargo.lock`, `python/Cargo.lock` and `python/uv.lock` record package metadata;
regenerate them when that revision changes package versions or dependencies.

Build both Python wheels from one molpy checkout with the same Rust toolchain.
The capsule name checks the compiler, source and handle layout when a frame
or region crosses between extensions; a mismatch raises `ValueError`.

**First-time setup:**

```bash
# Fetch test data used by the regression suite
bash ../molpy/scripts/fetch-test-data.sh

# Git hooks via prek (reads .pre-commit-config.yaml)
prek install

# Verify the Rust build
cargo build -p molcrafts-molpack
```

## Running tests

`.github/workflows/ci.yml` is the only place these commands are written:

```bash
# Rust — the `rust` check. Behaviour lives in `#[cfg(test)]` modules next
# to the code; there is no tests/ directory.
cargo test --locked --lib --features cli
cargo test --locked --doc --features cli
cargo check --locked --all-targets --features cli      # bin + examples must compile
cargo check --locked --no-default-features && cargo check --locked --features rayon

# Python — tox isolated env (tox itself from python/ dependency-group dev)
uv run --locked --python 3.12 --directory python --group dev tox -e py
```

CI checks `../molpy` out at that ref. A local build uses the sibling as it is.

For a quick local edit loop you may still `maturin develop` in a personal
venv; **do not** rely on that for the gate — CI uses
`uv run --locked --python 3.12 --directory python --group dev tox -e py`.

## Hooks

`prek install` installs both hook types from `.pre-commit-config.yaml`.
The hook is actionlint on `.github/workflows`, on commit and on push. It
does not run the test or lint commands; those exist only as jobs in
`.github/workflows/ci.yml`. **Never `git commit --no-verify` or
`git push --no-verify`, and never merge a red pull request.**

`scripts/hook-run.sh` stays so the shared git hooks on this cluster keep
prek on the login node. They look for `MOLCRAFTS_HOOK_RUNNER` in that file;
if the string is missing they send the whole hook to Slurm.

## Partners

`.github/partners.env` names `MolCrafts/molpy` and the exact commit to check
out as `../molpy`. Every workflow calls `setup-partners` without overriding
that file. `scripts/partners.py check` verifies that the revision exists and
that path dependencies resolve inside the partner checkout.

A local build does not move `../molpy`. To update the dependency, check out
the intended revision there, change `MOLPY_REF`, and regenerate the locks
with the commands in `.github/partners.env`. Build and test both wheels
against that source before opening a PR. A release tag is created only after
the changes reach `master`; preparing an API update does not publish it.

## CI

`.github/workflows/ci.yml` is the workflow. Its jobs call
`MolCrafts/molcrafts-ci` `setup-rust`, `setup-python` and `setup-partners`.
A pull request, a push to `dev` or `master`, and a manual run execute every
job. `python` is CPython 3.12 on Linux, macOS and Windows, and the test wheel
is a dev build. The other checks run on Linux. The job `result`
is the check `ci / result`, and it passes when every other job did.
`release.yml` calls this same file.

`release.yml` is the tag (and the manual dry run). It runs that CI, then
publishes. A dispatch builds and runs `cargo publish --dry-run` and uploads
nothing. A `v*` tag on MolCrafts publishes the crate, the wheels, the sdist
and the GitHub Release. The tag must be `v` plus the `Cargo.toml` version and
an ancestor of `master`. Trusted publishing names `release.yml` and the
`crates-io` and `pypi` environments. Cloudflare Pages deploys the site from
MolCrafts; CI only builds it (`docs`, `zensical build --clean --strict`).

## Code style

- `cargo fmt` / `ruff` / `clippy` / `ty` — the `hygiene`, `clippy` and `ty` jobs in `.github/workflows/ci.yml`
- CI: clippy, ty, rust tests + `uv run … tox -e py` (see [Running tests](#running-tests))
- Follow the immutability rule: return new values, never mutate in place
- Keep files under ~400 lines; split at ~200 if the module grows beyond one concern
- New public types must implement `Debug` and, where appropriate, `Clone`

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
