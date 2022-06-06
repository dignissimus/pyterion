<h1 align="center">Pyterion</h1>
<div align="center">Statistics-driven Microbenchmarking in Python</div>

Pyterion is a Python library that provides utilities to benchmark and analyse performance.

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
