# Examples

The repository includes five canonical Packmol-equivalent workloads. Each can
be run directly through the CLI:

```bash
molpack examples/pack_mixture/mixture.inp
molpack examples/pack_bilayer/bilayer-comment.inp
molpack examples/pack_interface/interface.inp
molpack examples/pack_spherical/spherical-comment.inp
molpack examples/pack_solvprotein/solvprotein.inp
```

## Workloads

| Workload | Script | Demonstrates |
|---|---|---|
| Mixture | `examples/pack_mixture/mixture.inp` | Two species co-packed in a cube. |
| Bilayer | `examples/pack_bilayer/bilayer-comment.inp` | Atom-subset plane restraints for layer orientation. |
| Interface | `examples/pack_interface/interface.inp` | Liquid-liquid interface around a fixed solute. |
| Spherical | `examples/pack_spherical/spherical-comment.inp` | Nested radial shells. |
| Solvated protein | `examples/pack_solvprotein/solvprotein.inp` | Fixed protein plus water and ions. |

## Compare with programmatic APIs

The same workloads also exist as Rust examples under `examples/pack_*` and as
Python scripts under `python/examples/`. Use them when you want to compare a
scripted `.inp` job with the builder APIs.

```bash
cargo run --release --features io --example pack_mixture
cd python
python examples/pack_mixture.py
```
