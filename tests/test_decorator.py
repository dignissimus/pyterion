import random

from pyterion import benchmark


@benchmark
def sort_list():
    random.seed(0)
    unsorted = list(range(1000))
    random.shuffle(unsorted)
    sorted(unsorted)


@benchmark(confidence_level=0.95)
def reverse_list():
    random.seed(0)
    ordered = list(range(1000))
    reversed(ordered)


@benchmark(significance_level=0.05)
def min_list():
    random.seed(0)
    unordered = list(range(1000))
    list(range(1000))
    min(unordered)
