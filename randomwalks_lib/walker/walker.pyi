from enum import Enum

class WalkerError(Enum):
    RequiresSingleDynamicProgram = 1
    RequiresMultipleDynamicPrograms = 2,
    NoPathExists = 3,
    InconsistentPath = 4,
    RandomDistributionError = 5

class StandardWalker:
    def __new__(cls, kernel: 'Kernel') -> 'StandardWalker': ...
    def generate_path(self, dp: 'DynamicProgram', to_x: int, to_y: int,
                      time_steps: int) -> 'Walk': ...
    def generate_paths(self, dp: 'DynamicProgram', qty: int,
                       to_x: int, to_y: int, time_steps: int) -> list['Walk']: ...
    def name(self, short: bool) -> str: ...

class CorrelatedWalker:
    def __new__(cls) -> 'CorrelatedWalker': ...
    def generate_path(self, dp: list['DynamicProgram'], to_x: int, to_y: int,
                      time_steps: int) -> 'Walk': ...
    def generate_paths(self, dp: list['DynamicProgram'], qty: int,
                       to_x: int, to_y: int, time_steps: int) -> list['Walk']: ...
    def name(self, short: bool) -> str: ...

class MultiStepWalker:
    def __new__(cls, max_step_size: int) -> 'MultiStepWalker': ...
    def generate_path(self, dp: 'DynamicProgram', to_x: int, to_y: int,
                      time_steps: int) -> 'Walk': ...
    def generate_paths(self, dp: 'DynamicProgram', qty: int,
                       to_x: int, to_y: int, time_steps: int) -> list['Walk']: ...
    def name(self, short: bool) -> str: ...

class LandCoverWalker:
    def __new__(cls, max_step_sizes: dict[int, int], land_cover: list[list[int]]) -> 'LandCoverWalker': ...
    def generate_path(self, dp: 'DynamicProgram', to_x: int, to_y: int,
                      time_steps: int) -> 'Walk': ...
    def generate_paths(self, dp: 'DynamicProgram', qty: int,
                       to_x: int, to_y: int, time_steps: int) -> list['Walk']: ...
    def name(self, short: bool) -> str: ...

class LevyWalker:
    def __new__(cls, jump_probability: float, jump_distance: int, kernel: 'Kernel') -> 'LevyWalker': ...
    def generate_path(self, dp: 'DynamicProgram', to_x: int, to_y: int,
                      time_steps: int) -> 'Walk': ...
    def generate_paths(self, dp: 'DynamicProgram', qty: int,
                       to_x: int, to_y: int, time_steps: int) -> list['Walk']: ...
    def name(self, short: bool) -> str: ...
