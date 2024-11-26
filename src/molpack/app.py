import molpy as mp
from molpack.optimizer import get_optimizer
from typing import Literal
import random

class Target:

    def __init__(self, frame, number, region, direction: str = "inside"):
        self.frame = frame
        self.number = number
        self.region = region
        self.direction = direction

class Molpack:

    def __init__(self, optimizer: Literal["packmol"] = "packmol"):
        
        self.targets = []
        self.optimizer = get_optimizer(optimizer)

    def add_target(self, frame:mp.Frame, number:int, region:mp.Region, direction: str = "inside"):
        self.targets.append(Target(frame, number, region, direction))

    def optimize(self, max_steps: int = 1000, seed: int|None = None):
        if seed is None:
            seed = random.randint(1, 10000)
        return self.optimizer.optimize(self.targets, max_steps=max_steps, seed=seed)
