# Wavefront OBJ Loader Library
### Introduction
The `wavefront-obj` library is a library for working with Wavefront OBJ files. Wavefront OBJ is a file format 
that represents three-dimensional meshes. In particular, the library parses and represents Wavefront OBJ files. 
It presently supports polygonal geometry only. Any file containing free-form geometry elements will be rejected. 
For specific details on the grammar, see the `GRAMMAR.md` file. For information on the file format, the `docs` 
directory contains a copy of the format specification.

### What Is The Wavefront OBJ Format For?
The Wavefront OBJ format's purpose is to load and store geometry assets for computer graphics applications such 
as computer aided design, 3D modeling software, games, and scientific visualization. It is a text based format 
that can be read or written directly in a text editor. It is the lingua franca of 3D geometry formats because 
of its age and simplicity. You can integrate the format into your asset pipeline by producing your assets in a 
program such as Blender, serializing your assets use in the OBJ format, and then loading them into the end user 
application. The library supports this workflow.

### Usage
Add the following line to your `Cargo.toml` file
```toml
[dependencies]
# ...
wavefront_obj = "0.4"
# ...
```
to import the library. Include the line
```rust
extern crate wavefront;
```
in your main file. This is all you need to use the it. The library loads all the geometry data into memory 
which you can then use in rendering.

### License
The `wavefront-obj` source code has two licenses you can choose from.
* Apache License (Version 2.0) ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Notes
* The Wavefront OBJ format does not contain information about how polygons of vertex count larger than three 
should be be tesselated. It is up to the modeler to communicate this information to the end users.
* All quadrilateral and higher vertex count faces must be either coplanar and convex, or coplanar and concave; either 
condition is necessary to get a correct rendering. Otherwise, the results of rendering will be unpredictable.