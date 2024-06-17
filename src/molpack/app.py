import molpy as mp
from molpack.optimizer import get_optimizer
from pathlib import Path

class Target:

    def __init__(self, struct, number, region, direction: str = "inside"):
        self.struct = struct
        self.number = number
        self.region = region
        self.direction = direction

class Molpack:

    def __init__(self):
        
        self.targets = []

    def add_struct(self, struct:mp.Struct|Path, number:int, region:mp.Region, direction: str = "inside"):
        self.targets.append(Target(struct, number, region, direction))

    def set_optimizer(self, optimizer: str = "packmol"):
        self.optimizer = get_optimizer(optimizer)

    def optimize(self):
        self.optimizer.optimize(self.targets)