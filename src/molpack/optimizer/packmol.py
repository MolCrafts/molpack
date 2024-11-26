import shutil
import molpy as mp
import subprocess
import tempfile
import numpy as np
import pandas as pd

class Optimizer:

    def __init__(self):
        ...

    def optimize(self, targets, max_steps, **kwargs):
        raise NotImplementedError
    
class Packmol(Optimizer):

    def __init__(self):
        super().__init__()
        # check if packmol is installed
        self.cmd = "packmol"
        self.optimizer = shutil.which(self.cmd)
        if self.optimizer is None:
            raise FileNotFoundError("Packmol not found in PATH")

    def generate_input(self, targets, max_steps, seed: int):
        
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
                lines.append(f"  {target.direction} cube {region.origin[0]} {region.origin[1]} {region.origin[2]} {region.side}")
            lines.append(f"end structure")

        with open("packmol.inp", "w") as f:
            f.write("\n".join(lines))

    def optimize(self, targets, max_steps, seed)->mp.System:
        self.generate_input(targets, max_steps, seed)
        subprocess.run(f"{self.cmd} < packmol.inp > packmol.out", shell=True, capture_output=True, text=True)
        self.targets = targets
        optimized_system = mp.io.read_pdb("_optimized.pdb")
        atoms = []
        bonds = []
        angles = []
        dihedrals = []
        system = mp.System(frame=mp.Frame("optimized"))
        n_atoms = np.cumsum([len(target.frame["atoms"]) for target in self.targets for _ in range(target.number)])
        n_atoms = np.concatenate([[0], n_atoms])
        n_struct_added = 0
        for target in self.targets:
            for i in range(target.number):
                a = target.frame["atoms"].copy()
                a['id'] = a['id'] + n_atoms[n_struct_added+i]
                a['molid'] = a['residue'] + n_struct_added+i
                atoms.append(a)
            if "bonds" in target.frame:
                for i in range(target.number):
                    b = target.frame["bonds"].copy()
                    b['id'] = b['id'] + n_atoms[n_struct_added+i]
                    b[['i', 'j']] = b[['i', 'j']] + n_atoms[n_struct_added+i]
                    bonds.append(b)
            if "angles" in target.frame:
                for i in range(target.number):
                    a = target.frame["angles"].copy()
                    a['id'] = a['id'] + n_atoms[n_struct_added+i]
                    a[['i', 'j', 'k']] = a[['i', 'j', 'k']] + n_atoms[n_struct_added+i]
                    angles.append(a)
            if "dihedrals" in target.frame:
                for i in range(target.number):
                    d = target.frame["dihedrals"].copy()
                    d['id'] = d['id'] + n_atoms[n_struct_added+i]
                    d[['i', 'j', 'k', 'l']] = d[['i', 'j', 'k', 'l']] + n_atoms[n_struct_added+i]
                    dihedrals.append(d)

            n_struct_added += target.number
        _atoms = pd.concat(atoms, ignore_index=True)
        _atoms[['x', 'y', 'z']] = optimized_system.frame["atoms"][['x', 'y', 'z']]
        system.frame["atoms"] = _atoms
        system.frame["bonds"] = pd.concat(bonds, ignore_index=True)
        system.frame["angles"] = pd.concat(angles, ignore_index=True)
        system.frame["dihedrals"] = pd.concat(dihedrals, ignore_index=True)
        return system