# Oxide
CDTCMMS2 Rust Project

Step 1: Learn Rust * http://rustbyexample.com/
                   * https://doc.rust-lang.org/book/

Step 2: Make a sick molecular visualiser.

## High-level overview

Currently we have something that displays a specific molecule, with broken rotations.

The molecule (the thing being viewed) is created using the molecule module.
Atoms are then added to build up the structure.

The camera is then set up (with anti-aliasing), and the main loop entered in which the viewer continously checks for input.

###What we want (not fixed, not prioritised):

* The ability to load in a given structure from a file
* Proper handling of rotations and perspective using quaternions
* Ability to manipulate the bonds and the representation of the molecule
* Supercell/Projection Analysis/Polyhedra for analysing what the structure looks like
* A GUI for doing all the above
* Output of images of the structure


### Personal to-dos

#### wpg

* Floating utility player

#### mj

* Write 3d OpenGL... I've done it before, it's just a case of Rustifying it (and remembering it, since I'm on a different laptop...)

#### mle

* Write interface with some crystal file formats to plot shapes at the right coordinates
