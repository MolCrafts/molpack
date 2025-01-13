from .nlopt import NloptPacker
from .packmol import Packmol

def get_packer(name, *args, **kwargs):

    if name == "packmol":
        return Packmol(*args, **kwargs)
    else:
        raise NotImplementedError(f"Optimizer {name} not implemented")