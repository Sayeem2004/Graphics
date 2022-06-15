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
- [x] `bezier` mdl command
- [x] `circle` mdl command
- [x] `hermite` mdl command
- [x] `light` mdl command
- [x] `movelight` mdl command
- [x] `savecs` mdl command
- [x] `saveknobs` mdl command
- [x] `screen` mdl command
- [x] `set` mdl command
- [x] `setknobs` mdl command
- [x] `terrain` mdl command
- [x] `tween` mdl command

## Details

- `alterlight` name dr dg db knob
  - Takes in the above arguments and shifts the rgb values of the light specified by name (if it exists) by the values dr, dg, db, respectively. A knob can be attached to the command for animation purposes as well. This command works similarly to other transformation commands but for lights instead of coordinate systems.

- `ambient` r g b
  - Takes in the above arguments and sets the ambient light of the image to the rgb value of r, g, b respectively. Implemented as described by the instructions in MDL.spec.

- `light` name r g b x y z
  - Takes in the above arguments and adds a light specified by name to the symbol table and another vector in the program which is then iterated through when performing lighting calculations. Follows the instructions described in the assignment page and MDL.spec.

- `movelight` name dx dy dz knob
  - Takes in the above arguments and shifts the xyz coordinates of the light specified by name (if it exists) by the values dx, dy, dz respectively. A knob can be attached to the command for animation purposes as well. This command works almost exactly as the move command but for lights instead of coordinate systems.

- `savecs` name
  - Takes in the above arguments and saves the current coordinate system under the symbol specified by name. This symbol can be given as an optional argument to the shape and line commands as well. Implemented as described by the instructions in MDL.spec and the assignment page.

- `saveknobs` name
  - Takes in the above arguments and saves all the current knobs into a knoblist specified by name, and this knoblist is then inserted into the symbol table for future use. Implemented as described by the instructions in MDL.spec and the assignment page.

- `screen` width height
  - Takes in the above arguments and sets the width and height of the image the script is creating. Implemented as described by the instructions in MDL.spec.

- `set` name val
  - Takes in the above arguments and sets the value of the knob specified by name (if it exists) to val.Implemented as described by the instructions in MDL.spec and the assignment page.

- `setknobs` val
  - Takes in the above argument and sets the value of all knobs currently in the symbol table to val. Implemented as described by the instructions in MDL.spec and the assignment page.

- `terrain` freq water
  - Takes in the above arguments which represent the mountainousness of the data generated (recommended 1 <= freq <= 25, but all freq >= 0 is allowed) and the percentage of data generated covered by water (recommended around 50, but all 100 >= water >= 0 is allowed). These arguments are then imported to a python file which uses perlin noise generation to create a vector of altitudes. These altitudes are then used in combination with a gradient function to color the image and make it look (hopefully somewhat) like a topographic map. Note that each pixel in the image is assigned an altitude, so to prevent too much wait time please limit the screen size to less than 500x500. Note that this command requires some python dependencies, which are automically installed when running the makefile using the make install command.

- `tween` name1 name2
  - Takes in the above arguments and searches the symbol table for two knoblists specified by name1 and name2 respectively. It then iterates through these knoblists and sees if they contain any common knobs, if they do these knobs are then interpolated and inserted into a frames vector for future use in animation. Implemented as described by the instructions in MDL.spec and the assignment page.
