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
  directory named `molrs`:

```
workspace/
├── molrs/      ← git clone https://github.com/MolCrafts/molpy
└── molpack/    ← this repo
```

The root `Cargo.toml` uses a path dependency on `../molrs/crates/molrs`. With the
sibling layout above everything resolves automatically.

**Version pins:** a local build uses the sibling checkout `../molrs`. The
version fields name the **0.20.*** minor line (see `Cargo.toml` and
`python/pyproject.toml`). CI checks out `MolCrafts/molpy` at ref `dev` into
`../molrs` (`setup-partners` in `.github/workflows/ci.yml`).
`.github/partners.env` names the same repository and ref. `Cargo.lock`,
`python/Cargo.lock` and `python/uv.lock` are committed and every cargo / uv
call in the workflow is `--locked`; they record molpy's package metadata, not a
commit, so relock only when molpy's `dev` changes its version or dependencies
(the recipe is in `.github/partners.env`).
The Python wheel checks this on ``import molpack`` — a molpy minor mismatch is an
``ImportError``, not a later FFI segfault.

**First-time setup:**

```bash
# Fetch test data used by the regression suite
bash ../molrs/scripts/fetch-test-data.sh

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

CI checks `../molrs` out at that ref. A local build uses the sibling as it is.

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

On `dev`, the partner is tracked, not pinned. `setup-partners` in
`.github/workflows/ci.yml` and `release.yml` names it: repository
`MolCrafts/molpy`, ref `dev`, path `../molrs`. It does not look up a branch
of the same name. `.github/partners.env` must name the same repository and
ref (`MOLRS_REPOSITORY`, `MOLRS_REF`). `scripts/partners.py check` fails when
a path dependency (`../molrs`, `../../molrs`, ...) lands outside that
checkout, or when a workflow spells a partner commit of its own. The relock
recipe in `partners.env` still uses `scripts/partners.py run`.

A local build does not move `../molrs`.

A change that needs a molrs change lands by updating the workflow `ref` and
`MOLRS_REF` to the molrs commit you built against, in a commit of its own,
after molrs's change is on the ref you name. Do not expect CI to follow a
same-named branch.

**Releasing.** A release builds against a fixed molrs: the release commit on
`master` sets the workflow `ref` and `MOLRS_REF` to the molrs release tag
of the line `Cargo.toml` names (`vX.Y.Z`), relocks if that changes molrs's
metadata, and is tagged. When `master` is merged back into `dev`, both name
`dev` again.

## CI

`.github/workflows/ci.yml` is the workflow. Its jobs call
`MolCrafts/molcrafts-ci` `setup-rust`, `setup-python` and `setup-partners`.
Every push, pull request and manual run executes every job: `python` on
Linux, macOS and Windows, and the other checks on Linux. The job `result`
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
