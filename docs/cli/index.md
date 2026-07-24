# CLI

Use the `molpack` binary when you already have a Packmol-style `.inp`
script, or when you want a reproducible packing job that can be checked into a
simulation workflow.

```bash
molpack mixture.inp
```

The file form resolves paths inside the script relative to the script's
directory. Stdin mode is also supported and resolves relative paths against the
current working directory:

```bash
molpack < mixture.inp
```

## Install

```bash
cargo install molcrafts-molpack --features cli
molpack --help
```

For source builds:

```bash
cargo build --features cli --bin molpack
```

## Minimal input

```text
tolerance 2.0
seed 42
filetype pdb
output packed.pdb
nloop 400

structure water.pdb
  number 1000
  inside box 0. 0. 0. 40. 40. 40.
end structure
```

Every mobile structure should have a spatial restraint such as `inside box`,
`inside sphere`, or a global `pbc` declaration. Without one, initial placement
has to infer a huge free-space box and the run can become impractical.

## Next steps

- [Packmol Script Notes](script-format.md) links to the Packmol manual and
  lists molpack-specific keywords, formats, and parser behavior.
- [Formats](formats.md) documents readable and writable molecular file types.
- [Examples](examples.md) shows the five checked-in canonical workloads.
