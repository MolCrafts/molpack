from .base import Packer
import nlopt
import numpy as np


class NloptPacker(Packer):

    def __init__(self, algorithm: str="LN_COBYLA"):
        super().__init__()
        self.algorithm = getattr(nlopt, algorithm)
        
    def pack(self):

        inner_opt = nlopt.opt(self.algorithm, self.n_points*3)
        print(f"Using {inner_opt.get_algorithm_name()} algorithm")
        print(f"Number of to be optimized parameters: {self.n_points*3}")

        # !! x is to be optimized parameters
        # !! not the coordinates of the points
        # !! x.shape = (n_points * (3 + 4), )
        # !! x[:, :3] = translation
        # !! x[:, 3:] = rotation quaternion
        x = np.zeros(self.n_points*3)
        all_xyz = []
        for target in self.targets:
            all_xyz.append(np.tile(target.points, (target.number, 1)))
        xyz = np.concatenate(all_xyz, axis=0)

        def constraints(result: np.ndarray, x: np.ndarray, grad: np.ndarray):
            if grad.size > 0:
                raise NotImplementedError("Gradient is not implemented")
            for i, target in enumerate(self.targets):
                result[i] = target.region.constrain(xyz+x.reshape(-1, 3))
            print(result[0])

        def objective(x, grad):
            if grad.size > 0:
                raise NotImplementedError("Gradient is not implemented")
            
            new_xyz = xyz + x.reshape(-1, 3)
            # naive nblist
            dist = np.linalg.norm(new_xyz[:, None, :] - new_xyz[None, :, :], axis=-1)
            return -np.triu(dist).sum()
            
        inner_opt.add_inequality_mconstraint(constraints, np.zeros(len(self.targets)))
        inner_opt.set_min_objective(objective)
        inner_opt.set_xtol_rel(1e-4)
        x = inner_opt.optimize(x)
        minf = inner_opt.last_optimum_value()
        print("optimum at ", x)
        print("minimum value = ", minf)
        print("result code = ", inner_opt.last_optimize_result())
        # opt = nlopt.opt(nlopt.AUGLAG, self.n_points*3)
        # opt.set_local_optimizer(inner_opt)
        # opt.set_xtol_rel(1e-4)
        # result = opt.optimize(x)
        # optimal_value = opt.last_optimum_value()

        print(f"Optimal solution: {result}")
        print(f"Optimal value: {optimal_value}")
        return xyz + x.reshape(-1, 3)

