from molpack.optimizer.packmol import Packmol

def get_optimizer(name):
    if name == "packmol":
        return Packmol()
    else:
        raise NotImplementedError(f"Optimizer {name} not implemented")