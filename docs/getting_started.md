# Quickstart

Pack **100 water molecules** into a **40 Å** cube. This walkthrough uses the
Python package — the shortest path from a loaded frame to a packed result. The
same model is available as a [CLI script](cli/) or the [Rust builder](rust/).

## 1. Install

```bash
pip install molcrafts-molpack
```

This installs the packing engine and pulls in `molcrafts-molrs` for the frame
type plus PDB/XYZ I/O.

## 2. Load or build a template

=== "From a PDB file"

    ```python
    import molrs

    frame = molrs.read_pdb("water.pdb")
    ```

=== "In-memory (no file)"

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

!!! warning "Missing restraints"
    Without a spatial restraint (or a global PBC box), initial placement has to
    invent a huge free-space region and the run can become impractical.

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

| Field | Meaning |
|---|---|
| `converged` | Both objectives fell below the packer precision threshold |
| `fdist` | Pair-distance (overlap) violations |
| `frest` | Restraint violations |
| `frame` | Topology-complete packed `molrs.Frame` |

For a frame-only return, use `Molpack().pack([water], max_loops=200)`.

## 5. Save

```python
import molrs

molrs.write_pdb(packed, "water_box.pdb")
# or: molrs.write_xyz(packed, "water_box.xyz")
```

## Where next

<div class="molpack-next-cards" markdown>

- **[Concepts](concepts/)** — targets, restraints, phases of the packer
- **[CLI](cli/)** — same job as a Packmol-style `.inp`
- **[Python guide](python/)** — fixed solutes, PBC, collective restraints
- **[Rust API](rust/)** — embed the engine in a native crate
- **[Packmol parity](packmol_parity/)** — what matches Packmol, and what does not

</div>
