import mdl
import os
import sys
import json
import random

def main():
    # Getting filename from command line input
    filename = sys.argv[1]

    # Parsing file and error checking
    p = mdl.parseFile(filename)
    if (p): (operations, symbols) = p
    else: print("Python: Parsing failed"); return

    # Creating temp directory
    if not os.path.exists("temp"): os.mkdir("temp")

    # Converting data to json file
    num = random.randint(0, 1000)
    name1 = "temp/operation" + str(num).zfill(4)
    with open(name1, "w") as path: json.dump(operations, path, indent = 4)
    name2 = "temp/symbol" + str(num).zfill(4)
    with open(name2, "w") as path: json.dump(symbols, path, indent = 4)

    # Printing absolute paths to stdout
    print("Python: Parsing succeeded")
    print(os.path.abspath(name1))
    print(os.path.abspath(name2))

main()
