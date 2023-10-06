import typing as t
from enum import Enum

class GCSPoint:
    def __new__(cls, x: float, y: float) -> 'GCSPoint': ...
    def __repr__(self) -> str: ...
    def __add__(self, other) -> 'GCSPoint': ...
    def __sub__(self, other) -> 'GCSPoint': ...
    def __str__(self) -> str: ...

class XYPoint:
    def __new__(cls, x: float, y: float) -> 'XYPoint': ...
    def __repr__(self) -> str: ...
    def __add__(self, other) -> 'XYPoint': ...
    def __sub__(self, other) -> 'XYPoint': ...
    def __str__(self) -> str: ...

class Dataset:
    def __new__(cls, coordinate_type: 'CoordinateType') -> 'Dataset': ...
    def __len__(self) -> int: ...
    def is_empty(self) -> bool: ...
    def coordinate_type(self) -> 'CoordinateType': ...
    def push(self, datapoint: 'Datapoint'): ...
    def get(self, index: int) -> t.Optional['Datapoint']: ...
    def __iter__(self) -> t.Iterable['Datapoint']: ...
    def keep(self, from_idx: t.Optional[int] = None, to_idx: t.Optional[int] = None): ...
    def filter(self, filter: 'DatasetFilter') -> int: ...
    def min_max(self, from_idx: t.Optional[int] = None,
                to_idx: t.Optional[int] = None) -> t.Optional[tuple['Point', 'Point']]: ...
    def convert_gcs_to_xy(self, scale: float): ...
    def convert_xy_to_gcs(self, scale: float): ...
    def rw_between(self, dp: 'SimpleDynamicProgram' | 'MultiDynamicProgram',
                   walker: 'StandardWalker' | 'CorrelatedWalker' | 'MultiStepWalker' | 'LevyWalker',
                   from_idx: int, to_idx: int, time_steps: int, auto_scale: bool,
                   extra_steps: int) -> 'Walk': ...
    def generate_walks(self, dp: 'SimpleDynamicProgram' | 'MultiDynamicProgram',
                       walker: 'StandardWalker' | 'CorrelatedWalker' | 'MultiStepWalker' | 'LevyWalker',
                       count: int = 1, time_steps: t.Optional[int] = None,
                       by_time_diff: t.Optional[tuple[float, str]] = None,
                       by_dist: t.Optional[float] = None,
                       auto_scale: t.Optional[bool] = False,
                       extra_steps: t.Optional[int] = 0) -> list['Walk']: ...
    def direct_between(self, from_idx: int, to_idx: int) -> 'Walk': ...
    def print(self, from_idx: t.Optional[int] = None, to_idx: t.Optional[int] = None): ...
    def plot(self, path: str, from_idx: t.Optional[int] = None, to_idx: t.Optional[int] = None,
             color_by: t.Optional[str] = None): ...

class DatasetFilter:
    @staticmethod
    def by_metadata(key: str, value: str) -> 'DatasetFilter': ...
    @staticmethod
    def by_coordinates(from_point: 'GCSPoint' | 'XYPoint', to_point: 'GCSPoint' | 'XYPoint') -> 'DatasetFilter': ...

class Datapoint:
    def __new__(cls, point: any, metadata: Dict[str, str]) -> 'Datapoint': ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

class DatasetLoaderError(Enum):
    NoXColumnSpecified = 1
    NoYColumnSpecified = 2
    MoreColumnsThanActions = 3

class CoordinateType(Enum):
    GCS = 1
    XY = 2

    def __repr__(self) -> str: ...

class CSVLoader:
    def __new__(cls, path: str, delimiter: str = ',', header: bool = False,
                coordinate_type: 'CoordinateType' = 'CoordinateType.GCS',
                columns: list[str] = []) -> 'CSVLoader': ...
    def load(self) -> 'Dataset': ...
    def coordinate_type(self) -> 'CoordinateType': ...
