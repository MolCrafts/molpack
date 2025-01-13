import shutil
import molpy as mp
import subprocess
import tempfile
import numpy as np
import pandas as pd
from .base import Packer


class Packmol(Packer):

    def __init__(self, work_dir):
        super().__init__()
        # check if packmol is installed
        self.cmd = "packmol"
        self.work_dir = work_dir
        self.optimizer = shutil.which(self.cmd)
        if self.optimizer is None:
            raise FileNotFoundError("Packmol not found in PATH")

    def generate_input(self, targets, max_steps, seed: int):

        self.intermediate_files = [
            self.work_dir / "_optimized.pdb",
            self.work_dir / "packmol.inp",
            self.work_dir / "packmol.out",
        ]

        lines = []
        lines.append("tolerance 2.0")
        lines.append("filetype pdb")
        lines.append("output _optimized.pdb")
        lines.append(f"seed {seed}")
        for target in targets:
            frame = target.frame
            number = target.number
            region = target.region

            tmpfile = tempfile.NamedTemporaryFile(delete=False, suffix=".pdb")

            mp.io.write_pdb(mp.System(frame=frame), tmpfile.name)

            lines.append(f"structure {tmpfile.name}")
            lines.append(f"  connect no")  # when we need it?
            lines.append(f"  number {number}")
            if isinstance(region, mp.region.Cube):
                lines.append(
                    f"  inside cube {region.origin[0]} {region.origin[1]} {region.origin[2]} {region.side}"
                )
            lines.append(f"end structure")

        with open(self.work_dir / "packmol.inp", "w") as f:
            f.write("\n".join(lines))

    def remove_input(self):
        for file in self.intermediate_files:
            file.unlink()

    def pack(self, targets, max_steps, seed) -> mp.System:
        self.generate_input(targets, max_steps, seed)
        subprocess.run(
            f"{self.cmd} < packmol.inp > packmol.out",
            shell=True,
            capture_output=True,
            text=True,
            cwd=self.work_dir,
        )
        self.targets = targets
        optimized_system = mp.io.read_pdb(self.work_dir / "_optimized.pdb")
        # self.remove_input()
        atoms = []
        bonds = []
        angles = []
        dihedrals = []
        system = mp.System(frame=mp.Frame())
        n_atoms = np.cumsum(
            [
                len(target.frame["atoms"])
                for target in self.targets
                for _ in range(target.number)
            ]
        )
        n_atoms = np.concatenate([[0], n_atoms])
        n_struct_added = 0
        n_atom_types = 0
        for target in self.targets:
            for i in range(target.number):
                a = target.frame["atoms"].copy()
                a["id"] = a["id"] + n_atoms[n_struct_added + i]
                a["molid"] = a["molid"] + n_struct_added + i
                a["type"] = a["type"] + n_atom_types
                atoms.append(a)
            if "bonds" in target.frame:
                for i in range(target.number):
                    b = target.frame["bonds"].copy()
                    b["id"] = b["id"] + n_atoms[n_struct_added + i]
                    b[["i", "j"]] = b[["i", "j"]] + n_atoms[n_struct_added + i]
                    bonds.append(b)
            if "angles" in target.frame:
                for i in range(target.number):
                    an = target.frame["angles"].copy()
                    an["id"] = an["id"] + n_atoms[n_struct_added + i]
                    an[["i", "j", "k"]] = (
                        an[["i", "j", "k"]] + n_atoms[n_struct_added + i]
                    )
                    angles.append(an)
            if "dihedrals" in target.frame:
                for i in range(target.number):
                    d = target.frame["dihedrals"].copy()
                    d["id"] = d["id"] + n_atoms[n_struct_added + i]
                    d[["i", "j", "k", "l"]] = (
                        d[["i", "j", "k", "l"]] + n_atoms[n_struct_added + i]
                    )
                    dihedrals.append(d)

            n_atom_types += a["type"].max()  # assume start with 1 always

            n_struct_added += target.number
        _atoms = pd.concat(atoms, ignore_index=True)
        _atoms[["x", "y", "z"]] = optimized_system.frame["atoms"][["x", "y", "z"]]
        system.frame["atoms"] = _atoms
        system.frame["bonds"] = pd.concat(bonds, ignore_index=True)
        system.frame["angles"] = pd.concat(angles, ignore_index=True)
        system.frame["dihedrals"] = pd.concat(dihedrals, ignore_index=True)
        return system
