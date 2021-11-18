# import the contents of the Rust library into the Python extension
from .librust import *


class PythonClass:
    def __init__(self, value: int) -> None:
        self.value = value