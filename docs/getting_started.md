# Quickstart

This tutorial packs 100 water molecules into a 40 Angstrom cube. It uses the
Python package because that is the shortest path from a loaded molecular frame
to a packed result; the same model can be expressed as a [CLI script](cli/) or
with the [Rust builder API](rust/).

## 1. Install

```bash
pip install molcrafts-molpack
```

`molcrafts-molpack` provides the packing engine and installs the `molrs`
dependency that provides the frame type plus PDB/XYZ readers and writers.

## 2. Load or build a molecule template

If you have a PDB file:

```python
import molrs

frame = molrs.read_pdb("water.pdb")
```

For a minimal in-memory example:

```python
import molrs
import numpy as np

frame = molrs.Frame.from_dict({
    "blocks": {
        "atoms": {
            "x": np.array([0.00, 0.96, -0.24]),
            "y": np.array([0.00, 0.00, 0.93]),
            "z": np.zeros(3),
            "element": ["O", "H", "H"],
        }
    }
})
```

## 3. Define the target

A `Target` is one molecule species plus the number of copies to place. Every
mobile target needs a spatial restraint:

```python
from molpack import InsideBoxRestraint, Target

water = (
    Target(frame, count=100)
    .with_name("water")
    .with_restraint(InsideBoxRestraint([0.0, 0.0, 0.0], [40.0, 40.0, 40.0]))
)
```

## 4. Pack

```python
from molpack import Molpack

result = (
    Molpack()
    .with_tolerance(2.0)
    .with_seed(42)
    .pack_with_report([water], max_loops=200)
)

print(result.converged, result.natoms, result.fdist, result.frest)
packed = result.frame
```

`fdist` measures pair-distance violations. `frest` measures restraint
violations. A successful run has both below the packer's precision threshold.

## 5. Save

molpack returns a `molrs.Frame` and delegates file writing to molrs or another
frame-compatible writer:

```python
molrs.write_xyz("packed.xyz", packed)
```

## Full script

```python
import molrs
import numpy as np
from molpack import InsideBoxRestraint, Molpack, Target

frame = molrs.Frame.from_dict({
    "blocks": {
        "atoms": {
            "x": np.array([0.00, 0.96, -0.24]),
            "y": np.array([0.00, 0.00, 0.93]),
            "z": np.zeros(3),
            "element": ["O", "H", "H"],
        }
    }
})

water = (
    Target(frame, count=100)
    .with_name("water")
    .with_restraint(InsideBoxRestraint([0.0, 0.0, 0.0], [40.0, 40.0, 40.0]))
)

result = (
    Molpack()
    .with_tolerance(2.0)
    .with_seed(42)
    .pack_with_report([water], max_loops=200)
)

print(f"converged={result.converged} natoms={result.natoms}")
molrs.write_xyz("packed.xyz", result.frame)
```

## Where to go next

- [Concepts](concepts.md) explains targets, restraints, periodic boxes,
  handlers, relaxers, and the optimizer model.
- [CLI](cli/) documents Packmol-compatible `.inp` scripts.
- [Python](python/) expands the binding API used in this tutorial.
- [Rust](rust/) covers the native builder API.
- [Examples](examples.md) collects the five canonical workloads.
