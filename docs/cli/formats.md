# Formats

The CLI reads molecule templates through the `io` feature and writes the final
packed structure to the path named by the script's `output` keyword.

The output format is inferred from the output file extension. Input formats can
be inferred from structure-file extensions or set globally with `filetype`.

| Format | Read | Write | Extension or `filetype` |
|---|---:|---:|---|
| PDB | Yes | Yes | `.pdb`, `pdb` |
| XYZ | Yes | Yes | `.xyz`, `xyz` |
| SDF / MOL | Yes | No | `.sdf`, `.mol`, `sdf` |
| LAMMPS dump | Yes | Yes | `.lammpstrj`, `lammps_dump` |
| LAMMPS data | Yes | No | `.data`, `lammps_data` |

## Example

```text
filetype pdb
output packed.xyz

structure water.pdb
  number 100
  inside box 0. 0. 0. 30. 30. 30.
end structure
```

The input template is read as PDB because of `filetype pdb`; the output is
written as XYZ because the output path ends with `.xyz`.
