"""FFI capsule-name gate.

molpack exchanges ``molrs_ffi`` handle capsules with the installed
``molcrafts-molpy`` wheel. The capsule names carry the molrs handle-layout
fingerprint, so a capsule whose name is not the one this wheel accepts must be
rejected at resolve time, cleanly, before any dereference.
"""

from __future__ import annotations

import ctypes

import molpy
import pytest

import molpack


class _ForeignFrame:
    """Quacks like a molrs Frame but exports a capsule under a foreign name."""

    def _ffi_frameref_capsule(self):  # noqa: ANN202 — mirrors the duck-typed contract
        new_capsule = ctypes.pythonapi.PyCapsule_New
        new_capsule.restype = ctypes.py_object
        new_capsule.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_void_p]
        # Bogus payload on purpose: the name check must reject the capsule
        # before any dereference happens.
        return new_capsule(ctypes.c_void_p(0xDEAD), b"molpy.FrameRef", None)


class TestFrameCapsuleGate:
    def test_foreign_capsule_is_rejected_at_resolve(self) -> None:
        with pytest.raises(ValueError, match="capsule name"):
            molpack.Target(_ForeignFrame(), 1)

    def test_molpy_frame_resolves(self) -> None:
        import numpy as np

        frame = molpy.core.Frame()
        block = molpy.core.Block()
        block.insert("x", np.array([0.0, 1.0]))
        block.insert("y", np.array([0.0, 0.0]))
        block.insert("z", np.array([0.0, 0.0]))
        block.insert("id", np.array([1, 2], dtype=np.uint64))
        frame["atoms"] = block
        assert molpack.Target(frame, 1) is not None


class _ForeignRegion:
    """Quacks like a molrs region but exports a capsule under a foreign name."""

    def _ffi_regionref_capsule(self):  # noqa: ANN202 — mirrors the duck-typed contract
        new_capsule = ctypes.pythonapi.PyCapsule_New
        new_capsule.restype = ctypes.py_object
        new_capsule.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_void_p]
        return new_capsule(ctypes.c_void_p(0xDEAD), b"molpy.RegionRef", None)


class _NotACapsule:
    def _ffi_regionref_capsule(self):  # noqa: ANN202
        return object()


class TestRegionCapsuleGate:
    def _target(self):
        import numpy as np

        frame = molpy.core.Frame()
        block = molpy.core.Block()
        block.insert("x", np.array([0.0]))
        block.insert("y", np.array([0.0]))
        block.insert("z", np.array([0.0]))
        block.insert("id", np.array([1], dtype=np.uint64))
        frame["atoms"] = block
        return molpack.Target(frame, 1)

    def test_foreign_region_is_rejected_at_resolve(self) -> None:
        with pytest.raises(ValueError, match="capsule name"):
            self._target().with_restraint(_ForeignRegion())

    def test_capsule_returning_non_capsule_is_typeerror(self) -> None:
        with pytest.raises(TypeError, match="PyCapsule"):
            self._target().with_restraint(_NotACapsule())

    def test_molpy_region_resolves(self) -> None:
        import numpy as np

        region = molpy.core.Sphere(np.zeros(3), 5.0)
        assert self._target().with_restraint(region) is not None
