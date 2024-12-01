from .base import Optimizer

import nlopt
import numpy as np
import molpy as mp

class Nlopt(Optimizer):

    def __init__(self, algorithm):
        super().__init__()
        self.algorithm = algorithm

    def optimize(self, targets, max_steps, seed)->mp.System:

        # create the optimization problem
        opt = nlopt.opt(self.algorithm, )
