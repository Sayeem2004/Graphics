# Imports
from opensimplex import OpenSimplex
import numpy as np
import os
import sys
# import pygmt
import random
# import xarray as xr

# # Function that extracts data from pygmt and creates a data file storing it
# def get_etopo_data(type, minlon, maxlon, minlat, maxlat):
#     # Creating temp directory and generating file path
#     if not os.path.exists("temp"): os.mkdir("temp")
#     num = random.randrange(0, 10000)
#     path = "temp/data" + str(num) + ".nc"

#     # Making sure coordinates are within bounds
#     minlon = max(-180, minlon)
#     maxlon = min(180, maxlon)
#     minlat = max(-89, minlat)
#     maxlat = min(89, maxlat)

#     # Determining which etopo data file to use
#     if (type == "03arc"): topodata = '@earth_relief_03s'
#     elif (type == "15arc"): topodata = '@earth_relief_15s'
#     else: topodata = '@earth_relief_30s'

#     # Extracting subregion and creating data.nc file
#     pygmt.grdcut(
#         grid = topodata,
#         outgrid = path,
#         projection = 'M4i',
#         region = [minlon, maxlon, minlat, maxlat],
#     )

#     # Converting data to csv
#     convert_to_csv(path)

# # Function that converts a nc file to a csv file
# def convert_to_csv(path):
#     # Creating temp directory
#     if not os.path.exists("temp"): os.mkdir("temp")

#     # Reading in data from nc file
#     nc = xr.open_dataset(path)
#     lon = nc.variables['x'][:]
#     length = np.size(lon)
#     lat = nc.variables['y'][:]
#     width = np.size(lat)
#     alt = nc.variables['z'][:]

#     # Reshaping and flattening data
#     lon = np.tile(lon,width)
#     lat = np.tile(lat,length)
#     lat = np.reshape(lat,(width,length))
#     lat = lat.flatten('F')
#     alt = np.array(alt)
#     alt = alt.flatten()

#     # Concatenating data together
#     data = np.column_stack((lon,lat,alt))

#     # Creating data.csv
#     np.savetxt(path[:-2] + "csv", data, delimiter=',')

def create_random_terrain(size, freq, water):
    # Fixing parameters
    size = int(size)
    freq = float(freq)
    water = float(water)

    # Creating temp directory
    if not os.path.exists("temp"): os.mkdir("temp")
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
    np.savetxt(path, alt, delimiter = ',', fmt="%s");

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
