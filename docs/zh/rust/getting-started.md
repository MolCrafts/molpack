# Quickstart

A Rust packing job has three parts:

1. Build one `Target` per molecule species.
2. Attach at least one spatial restraint to each mobile target, or use a
   packer-level global restraint.
3. Run `Molpack::pack(&targets, max_loops)`.

## One molecule type in a box

```rust
use molpack::{InsideBoxRestraint, Molpack, Target};

let water_positions = [
    [0.0, 0.0, 0.0],
    [0.96, 0.0, 0.0],
    [-0.24, 0.93, 0.0],
];
let water_radii = [1.52, 1.20, 1.20];

let water = Target::from_coords(&water_positions, &water_radii, 100)
    .with_name("water")
    .with_restraint(InsideBoxRestraint::new(
        [0.0, 0.0, 0.0],
        [40.0, 40.0, 40.0],
        [false, false, false],
    ));

let mut packer = Molpack::new()
    .with_tolerance(2.0)
    .with_seed(42);

let frame = packer.pack(&[water], 200)?;
let natoms = frame.get("atoms").and_then(|b| b.nrows()).unwrap_or(0);
println!("packed {natoms} atoms");
```

`pack()` returns a packed `molrs::Frame`. Use `pack_with_report()` if you also
need convergence fields:

```rust
let mut packer = Molpack::new().with_seed(42);
let result = packer.pack_with_report(&targets, 200)?;
println!("converged={} fdist={} frest={}", result.converged, result.fdist, result.frest);
```

## Builder defaults

Every tuning knob except `max_loops` has a Packmol-compatible default. Set a
builder value only when you need to change the default:

```rust
let packer = Molpack::new()
    .with_tolerance(2.0)
    .with_precision(0.01)
    .with_inner_iterations(20)
    .with_seed(42);
```

`max_loops` is positional because the right iteration budget depends on system
size and packing difficulty.

## Targets are snapshots

`Target` is a builder value. `pack()` snapshots the target slice at call time;
mutating or rebuilding a target after that does not affect an already running
pack.
