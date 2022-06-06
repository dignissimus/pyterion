from .pyterion import benchmark as _benchmark


def benchmark(function=None, **kwargs):
    if kwargs:

        def _inner_decorator(function):
            _benchmark(function)

        return _inner_decorator
    if function is not None:
        _benchmark(function)
    else:
        raise TypeError("benchmark requires a function to benchmark")
