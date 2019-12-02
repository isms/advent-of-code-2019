#!/usr/bin/env python
import sys
import numpy as np


def main(text):
    lines = text.split()
    arr = np.array(lines).astype(int)
    fuel = arr // 3 - 2
    return fuel.sum()


if __name__ == "__main__":
    text = sys.stdin.read()
    result = main(text)
    print(result)
