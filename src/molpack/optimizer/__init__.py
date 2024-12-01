from molpack.optimizer.packmol import Packmol
from molpack.optimizer.nlopt import Nlopt

def get_optimizer(name):
    if name == "packmol":
        return Packmol()
    else:
        raise NotImplementedError(f"Optimizer {name} not implemented")