<h1 align="center">Pyterion</h1>
<div align="center">Statistics-driven Microbenchmarking in Python</div>

<div align="center">
<a href="https://github.com/psf/black/blob/main/LICENSE"><img alt="License: MIT" src="https://black.readthedocs.io/en/stable/_static/license.svg"></a>
<a href="https://github.com/psf/black"><img alt="Code style: black" src="https://img.shields.io/badge/code%20style-black-000000.svg"></a>
</div>

-----

Pyterion is a Python library that provides utilities to benchmark and analyse performance.

------

# Usage
```python
import time
from pyterion import benchmark

@benchmark
def function():
    time.sleep(0.001)
```
Alternatively
```python
from pyterion import benchmark

def function():
    time.sleep(0.001)

benchmark(function)
```
