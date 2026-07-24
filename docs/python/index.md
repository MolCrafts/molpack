# Python

Packmol-grade molecular packing in Rust, with a first-class Python binding.

`molpack` arranges $N$ molecule types under geometric restraints without
overlaps. The engine is a faithful port of Packmol's GENCAN-driven algorithm
(Martínez *et al.* 2009); five canonical workloads stay pinned against Packmol
reference output.

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

<div class="molpack-next-cards" markdown>

- **[Installation](installation.md)** — pip install and verification
- **[Quickstart](getting-started.md)** — first pack end-to-end
- **[Targets](guide/targets.md)** — templates, counts, fixed placement
- **[Restraints](guide/restraints.md)** — geometric and collective restraints
- **[Packer](guide/packer.md)** — builder options, diagnostics, handlers
- **[Periodic boundaries](guide/periodic-boundaries.md)** — PBC rules
- **[Examples](examples.md)** — Packmol-equivalent workloads
- **[API reference](api-reference.md)** — class-by-class summary

</div>

## See also

- Rust crate: [`molcrafts-molpack`](https://crates.io/crates/molcrafts-molpack) —
  the underlying engine
- [`molcrafts-molrs`](https://pypi.org/project/molcrafts-molrs/) — frame I/O and
  the `Frame` data model (installed as a dependency)
