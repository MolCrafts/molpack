#!/usr/bin/env bash
# Rust test gate — shared by CI and pre-commit/pre-push.
# Mirrors the "rust tests" job in .github/workflows/ci.yml (not the long
# packmol regression suite, which stays CI-only on master).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"

if [[ ! -d "../molrs/molrs" ]]; then
  echo "error: expected sibling molrs at $(cd .. && pwd)/molrs" >&2
  exit 1
fi

echo "==> cargo test --features io --lib --tests --examples"
cargo test --features io --lib --tests --examples

echo "==> cargo test --test cli --features cli"
cargo test --test cli --features cli

echo "==> feature-gate cargo check"
cargo check --no-default-features
cargo check --features rayon
cargo check --features ff

echo "==> cargo build --benches"
cargo build --benches

echo "==> rust tests OK"
