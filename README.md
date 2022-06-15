# Graphics Final Project

## Name: Mohammad Khan

## Class Period: 4

## Requirements

Rust and Cargo which can be downloaded from here https://www.rust-lang.org/tools/install

Make which can be downloaded from http://ftp.gnu.org/gnu/make/

Python and Pip which can be downloaded from here https://pip.pypa.io/en/stable/installation/
## Compilation Instructions

Cloning the repo: `git clone git@github.com:Sayeem2004/Graphics.git`

Changing directory: `cd Graphics`

Compiling and running: `make`

Converting all ppm images to png: `make convert`

Running previous assignments: `make test`

Cleaning up repository: `make clean`

Removing all images: `make remove`

Opening up documentation: `make doc`

Installing necessary python requirements: `make install`

## Features

- [x] `alterlight` mdl command
- [x] `ambient` mdl command
- [x] `light` mdl command
- [x] `movelight` mdl command
- [x] `savecs` mdl command
- [x] `saveknobs` mdl command
- [x] `set` mdl command
- [x] `setknobs` mdl command
- [x] `terrain` mdl command
- [x] `tween` mdl command

## Details

- `alterlight` name dr dg db knob
  - Takes in the above arguments and shifts the rgb values of the light specified by name (if it exists) by the values dr, dg, db, respectively. A knob can be attached to the command for animation purposes as well. This command works similarly to other transformation commands but for lights instead of coordinate systems.

- `ambient` r g b
  - Takes in the above arguments and sets the ambient light of the image to the rgb value of r, g, b respectively. Implemented as described by the instructions in MDL.spec

- `light` name r g b x y z
  - Takes in the above arguments and adds a light specified by name to the symbol table and another vector in the program which is then iterated through when performing lighting calculations. Follows the instructions described in the assignment page and MDL.spec

- `movelight` name dx dy dz knob
  - Takes in the above arguments and shifts the xyz coordinates of the light specified by name (if it exists) by the values dx, dy, dz respectively. A knob can be attached to the command for animation purposes as well. This command works almost exactly as the move command but for lights instead of coordinate systems

- `savecs` name
  - Takes in the above arguments and saves the current coordinate system under the symbol specified by name. This symbol can be given as an optional argument to the shape and line commands as well. Implemented as described by the instructions in MDL.spec and the assignment page.

- `saveknobs` name
  - Takes in the above arguments and saves all the current knobs into a knoblist specified by name, and this knoblist is then inserted into the symbol table for future use. Implemented as described by the instructions in MDL.spec and the assignment page.

- `set` name val
  - Takes in the above arguments and saves all

- `tween` will be implemented as described in the assignment page.

- `shading` will include flat, gouraud, and phong shading.

- `terrain` will generate an image in the form of a topographic map (with the appropriate gradient shading to make it look like a topographic map) from a randomly generated terrain using methods like perlin noise terrain generation, and possiblly other ones, if found. Will take in numeric inputs related to the terrain generation method.

- `gmt` will generate an image in the form of a topographic map (with the appropriate gradient shading) from data taken from the pygmt global elevation dataset. Note that I will not be using the library specific drawing functions, and instead will be implementing my own drawing function. I will only be using the library for getting and parsing elevation data.
