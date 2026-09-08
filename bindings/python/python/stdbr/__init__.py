from . import stdbr as _native
from .stdbr import *

__doc__ = _native.__doc__
if hasattr(_native, "__all__"):
    __all__ = _native.__all__
