import shutil
import molpy as mp
import subprocess
import tempfile
import os

class Optimizer:

    def __init__(self):
        ...

    def optimize(self, targets):
        raise NotImplementedError
    
class Packmol(Optimizer):

    def __init__(self):
        super().__init__()
        # check if packmol is installed
        self.cmd = "packmol"
        self.optimizer = shutil.which(self.cmd)
        if self.optimizer is None:
            raise FileNotFoundError("Packmol not found in PATH")

    def generate_input(self, targets):
        
        lines = []
        lines.append("tolerance 2.0")
        lines.append("filetype pdb")
        lines.append("output _optimized.pdb")
        for target in targets:
            struct = target.struct
            number = target.number
            region = target.region

            tmpfile = tempfile.NamedTemporaryFile(delete=False, suffix=".pdb")

            mp.io.save_struct(tmpfile.name, struct)

            # NOTE: chemfiles has abnormal output for pdb
            # need to remove "CONECT I " lines manually
            pdb_lines = []
            with open(tmpfile.name, "r") as f:
                pdb_lines = f.readlines()
                pdb_lines = filter(lambda line: not (line.startswith("CONECT") and len(line.split()) < 3), pdb_lines)
            with open(tmpfile.name, "w") as f:
                f.writelines(list(pdb_lines))

            lines.append(f"structure {tmpfile.name}")
            lines.append(f"  number {number}")
            if isinstance(region, mp.region.Cube):
                lines.append(f"  {target.direction} cube {region.origin[0]} {region.origin[1]} {region.origin[2]} {region.side}")
            lines.append(f"end structure")

        with open("packmol.inp", "w") as f:
            f.write("\n".join(lines))

    def optimize(self, targets):
        self.generate_input(targets)
        subprocess.run(f"{self.cmd} < packmol.inp > packmol.out", shell=True)