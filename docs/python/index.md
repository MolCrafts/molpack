# molpack — Python

Packmol-grade molecular packing in Rust, with Python bindings.

`molpack` produces a non-overlapping arrangement of $N$ molecule types
with user-specified copy counts under geometric restraints. The engine
is a faithful port of Packmol's GENCAN-driven three-phase algorithm
(Martínez *et al.* 2009); correctness is pinned against Packmol's
reference output for five canonical workloads.

## At a glance

```python
import molrs
from molpack import InsideBoxRestraint, Molpack, Target

frame = molrs.read_pdb("water.pdb")

water = (
    Target(frame, count=100)
    .with_name("water")
    .with_restraint(InsideBoxRestraint([0.0, 0.0, 0.0], [40.0, 40.0, 40.0]))
)

packer = Molpack().with_tolerance(2.0).with_seed(42)
frame = packer.pack([water], max_loops=200)
print(f"packed {frame['atoms'].nrows} atoms")
```

## Next steps

- [Installation](installation.md) — pip install and verification.
- [Quickstart](getting-started.md) — first pack end-to-end.
- [Targets](guide/targets.md) — molecule templates, counts, fixed placement,
  and atom-subset restraints.
- [Restraints](guide/restraints.md) — geometric and collective restraints.
- [Packer](guide/packer.md) — builder options, diagnostics, and handlers.
- [Periodic boundaries](guide/periodic-boundaries.md) — PBC semantics and
  validation rules.
- [Examples](examples.md) — five Packmol-equivalent workloads.
- [API Reference](api-reference.md) — class-by-class summary.

## See also

- Rust crate: [`molcrafts-molpack`](https://crates.io/crates/molcrafts-molpack)
  — the underlying engine. All algorithmic details are documented there.
- [`molcrafts-molrs`](https://pypi.org/project/molcrafts-molrs/) —
  installed dependency that provides file I/O (PDB, XYZ, …) and the `Frame`
  data model. Pass a `molrs.Frame` directly to `Target`; no manual array
  extraction needed. `Molpack.pack()` returns a Frame-compatible structure
  for the writer of your choice.
