import molpy as mp
from molpack.packer import get_packer
from typing import Literal
import random
from pathlib import Path
from .target import Target

class Molpack:

    def __init__(self, work_dir, packer: Literal["packmol"] = "packmol"):
        
        self.targets = []
        self.work_dir = Path(work_dir)
        self.packer = get_packer(packer, work_dir)

    def add_target(self, frame:mp.Frame, number:int, region:mp.Region):
        self.targets.append(Target(frame, number, region))

    def optimize(self, max_steps: int = 1000, seed: int|None = None):
        if seed is None:
            seed = random.randint(1, 10000)
        return self.packer.pack(self.targets, max_steps=max_steps, seed=seed)
