#!/usr/bin/env bash
# Isolated Python binding tests — shared by CI and pre-commit/pre-push.
#
# Why a throwaway venv + wheel install:
#   - Avoids picking up a developer editable install / leftover .so from
#     `maturin develop`, which can hide packaging and import-path bugs.
#   - Installs *wheels* (or non-editable path builds), never editable trees.
#   - Tears the venv down so the workspace stays clean.
#
# molrs is installed from the sibling source tree (same tree Cargo path-deps),
# not from PyPI, so the extension's FFI layout matches the Python package.
# molpy is installed from PyPI at the pinned version.
#
# Prerequisites:
#   - Sibling molrs at ../molrs (path dep for the Rust extension).
#   - uv, a working Rust toolchain, Python 3.12+.
#
# Pins (must match Cargo.toml / pyproject.toml / CI MOLRS_GIT_REF):
#   molcrafts-molrs==0.9.3, molcrafts-molpy==0.9.3

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PYTHON_DIR="${ROOT}/python"
MOLRS_SRC="${ROOT}/../molrs"
MOLRS_PIN="${MOLRS_PIN:-0.9.3}"
MOLPY_PIN="${MOLPY_PIN:-0.9.3}"

if [[ ! -d "${MOLRS_SRC}/molrs" || ! -d "${MOLRS_SRC}/molrs-python" ]]; then
  echo "error: expected sibling molrs at ${MOLRS_SRC} (path dep for maturin)" >&2
  exit 1
fi

# Fail fast if the sibling tree is not the pinned version line.
SIBLING_VER="$(
  python3 -c "
import tomllib
from pathlib import Path
root = Path(r'''${MOLRS_SRC}''')
ws = tomllib.loads((root / 'Cargo.toml').read_text())
print(ws['workspace']['package']['version'])
"
)"
if [[ "${SIBLING_VER}" != "${MOLRS_PIN}" ]]; then
  echo "error: sibling molrs version is ${SIBLING_VER}, expected pin ${MOLRS_PIN}" >&2
  echo "       (CI checks out MolCrafts/molrs@v${MOLRS_PIN}; align the local clone)" >&2
  exit 1
fi

TMP="$(mktemp -d "${TMPDIR:-/tmp}/molpack-pytest.XXXXXX")"
cleanup() {
  rm -rf "${TMP}"
}
trap cleanup EXIT

echo "==> throwaway venv at ${TMP}/venv"
uv venv "${TMP}/venv" --python 3.12
# shellcheck disable=SC1091
source "${TMP}/venv/bin/activate"

echo "==> install build + runtime deps"
uv pip install \
  "maturin>=1.0,<2.0" \
  "pytest" \
  "numpy>=2.0" \
  "typer>=0.12" \
  "molcrafts-molpy==${MOLPY_PIN}"

echo "==> build+install molrs from sibling (non-editable, pin ${MOLRS_PIN})"
# Path install without -e: uv builds a wheel and installs it (not editable).
uv pip install "${MOLRS_SRC}/molrs-python"
INSTALLED_MOLRS="$(python -c 'import importlib.metadata as m; print(m.version("molcrafts-molrs"))')"
if [[ "${INSTALLED_MOLRS}" != "${MOLRS_PIN}" ]]; then
  echo "error: installed molcrafts-molrs==${INSTALLED_MOLRS}, expected ${MOLRS_PIN}" >&2
  exit 1
fi

echo "==> maturin build molpack (wheel, non-editable)"
mkdir -p "${TMP}/wheels"
(
  cd "${PYTHON_DIR}"
  maturin build --release -o "${TMP}/wheels"
)

WHEEL="$(ls -1 "${TMP}/wheels"/molcrafts_molpack-*.whl 2>/dev/null | head -1 || true)"
if [[ -z "${WHEEL}" ]]; then
  WHEEL="$(ls -1 "${TMP}/wheels"/*.whl | head -1)"
fi
echo "==> pip install molpack wheel (no editable): ${WHEEL}"
uv pip install --force-reinstall --no-deps "${WHEEL}"

# Sanity: not an editable install
python - <<'PY'
import molpack, pathlib
root = pathlib.Path(molpack.__file__).resolve()
print(f"molpack loaded from: {root}")
if "site-packages" not in str(root):
    raise SystemExit(f"expected site-packages install, got {root}")
PY

echo "==> pytest"
(
  cd "${PYTHON_DIR}"
  python -m pytest -v
)

echo "==> python tests OK (venv will be deleted)"
