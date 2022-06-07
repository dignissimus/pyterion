<h1 align="center">Pyterion</h1>
<div align="center">Statistics-driven Microbenchmarking in Python</div>

<div align="center">
<a href="https://github.com/dignissimus/pyterion"><img alt="Build status" src="https://github.com/dignissimus/pyterion/actions/workflows/CI.yml/badge.svg"></a>
<a href="https://github.com/psf/black/blob/main/LICENSE"><img alt="License: MIT" src="https://black.readthedocs.io/en/stable/_static/license.svg"></a>
<a href="https://github.com/psf/black"><img alt="Code style: black" src="https://img.shields.io/badge/code%20style-black-000000.svg"></a>
</div>

-----

Pyterion is a Python library that provides utilities to benchmark and analyse performance.

------

# Example
To benchmark a function, you can use the `benchmark` decorator.

```python
import time
from pyterion import benchmark

@benchmark
def function():
    time.sleep(0.001)
```
Or alternatively
```python
import time
from pyterion import benchmark

def function():
    time.sleep(0.001)

benchmark(function)
```

The above code below will produce the following output among other statistics and visualisations.

![Example Benchmark](docs/images/example-benchmark.png)
![Example Benchmark](docs/images/example-benchmark-pdf.svg)

# Related projects
* [criterion.rs](https://github.com/bheisler/criterion.rs) - The Rust microbenchmarking library that pyterion is based on
* [criterion](http://www.serpentine.com/criterion/) - The Haskell microbenchmarking library that inspired criterion.rs
