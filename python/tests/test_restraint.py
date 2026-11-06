"""Stacking molrs regions on a target — construction and attach only."""

from __future__ import annotations

import molpy as mp
import numpy as np

import molpack


def _make_frame():
    return mp.Frame(
        {
            "atoms": {
                "x": np.array([0.0]),
                "y": np.array([0.0]),
                "z": np.array([0.0]),
                "element": ["O"],
            }
        }
    )


class TestRegionConstruction:
    def test_box_and_sphere_reprs(self):
        box = mp.core.Cuboid([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])
        assert "Cuboid" in repr(box)
        assert "Sphere" in repr(mp.core.Sphere([0.0, 0.0, 0.0], 5.0))
        assert "composed" in repr(~mp.core.Sphere([0.0, 0.0, 0.0], 2.0))

    def test_half_spaces_replace_above_and_below(self):
        below = mp.core.HalfSpace([0.0, 0.0, 1.0], [0.0, 0.0, 10.0])
        above = ~mp.core.HalfSpace([0.0, 0.0, 1.0], [0.0, 0.0, 0.0])
        pts = np.array([[0.0, 0.0, 5.0], [0.0, 0.0, 12.0], [0.0, 0.0, -1.0]])
        assert list(below.contains(pts)) == [True, False, True]
        assert list(above.contains(pts)) == [True, True, False]


class TestConstraintStackingOnTarget:
    """Stacking multiple restraints via Target.with_restraint() calls."""

    def test_two_restraints(self):
        t = (
            molpack.Target(_make_frame(), count=1)
            .with_restraint(mp.core.Cuboid([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]))
            .with_restraint(mp.core.Sphere([5.0, 5.0, 5.0], 5.0))
        )
        assert t is not None

    def test_three_restraints(self):
        t = (
            molpack.Target(_make_frame(), count=1)
            .with_restraint(mp.core.Cuboid([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]))
            .with_restraint(mp.core.Sphere([5.0, 5.0, 5.0], 5.0))
            .with_restraint(~mp.core.HalfSpace([0.0, 0.0, 1.0], [0.0, 0.0, 0.0]))
        )
        assert t is not None

    def test_all_region_shapes(self):
        t = molpack.Target(_make_frame(), count=1)
        for r in [
            mp.core.Cuboid([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]),
            mp.core.Sphere([0.0, 0.0, 0.0], 5.0),
            ~mp.core.Sphere([0.0, 0.0, 0.0], 2.0),
            ~mp.core.HalfSpace([0.0, 0.0, 1.0], [0.0, 0.0, 0.0]),
            mp.core.HalfSpace([0.0, 0.0, 1.0], [0.0, 0.0, 10.0]),
            mp.core.Cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.0, 10.0),
            mp.core.Ellipsoid([0.0, 0.0, 0.0], [3.0, 4.0, 5.0]),
        ]:
            t = t.with_restraint(r)
        assert t is not None
