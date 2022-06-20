# pylint: disable = C0114, C0103, C0116, C0301, E0401

# Imports
import os
import sys
import random
from opensimplex import OpenSimplex
import numpy as np

# Function that takes in certain parameters and creates an csv file with altitude data
def create_random_terrain(size, freq, water):
    # Fixing parameters
    size = int(size)
    freq = float(freq)
    water = float(water)

    # Creating temp directory
    if not os.path.exists("temp"):
        os.mkdir("temp")
    num = random.randrange(0, 10000)
    path = "temp/data" + str(num) + ".csv"

    # Initializing altitude data and creating noise generators
    rnd = random.randrange(0, 1000000)
    gens = [OpenSimplex(seed=i) for i in range(10)]
    alt = np.zeros((size, size))

    # Creating noise in altitude data
    for x in range(size):
        for y in range(size):
            for i, gen in enumerate(gens):
                alt[x][y] += (0.5 ** i) * (gen.noise2(freq * (x / size - 0.5) - rnd, freq * (y / size - 0.5) - rnd) / 2 + (0.5 - water / 100))

    # Reshaping altitude data
    alt = alt.flatten()
    np.round(alt, decimals=6)

    # Creating data.csv
    np.savetxt(path, alt, delimiter = ',', fmt="%s")

    # Printing absolute path to stdout
    print("Python: Generating succeeded")
    print(os.path.abspath(path))

# Running stuff from this file
if __name__ == "__main__":
    args = sys.argv
    # args[0] = current file
    # args[1] = function name
    # args[2:] = function args : (*unpacked)
    globals()[args[1]](*args[2:])
