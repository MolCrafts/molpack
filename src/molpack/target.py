import molpy as mp

class Target:

    def __init__(self, frame:mp.Frame, number: int, region: mp.Region, is_fixed:bool=False, optimizer=None, name: str=""):
        self.frame = frame
        self.number = number
        self.region = region
        self.is_fixed = is_fixed
        self.optimizer = optimizer
        self.name = name

    def __repr__(self):
        return f"<Target {self.name}: density={self.density:.2f}>"

    @property
    def n_points(self):
        return len(self.frame['atoms']) * self.number
    
    @property
    def density(self):
        return self.n_points / self.region.volumn()
    
    @property
    def points(self):
        return self.frame['atoms'][['x', 'y', 'z']].to_numpy()