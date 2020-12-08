# Wavefront OBJ Loader Library

## Introduction
The **wavefront-obj** library is a library for working with Wavefront OBJ files. 
Wavefront OBJ is a file format that represents three-dimensional meshes and 
material data. In  particular, the library parses and represents Wavefront OBJ 
and MTL files. It presently supports polygonal geometry only. Any file containing 
free-form geometry elements will be rejected by the parser. For specific details 
on the grammar, see the `GRAMMAR.md` file. For information on the file format, the 
`docs` directory contains a copy of the format specification.

### What Is The Wavefront OBJ Format For?
The Wavefront OBJ format's purpose is to load and store geometry assets for 
computer graphics applications such as computer aided design, 3D modeling 
software, games, and scientific visualization. It is a text based format 
that can be read or written directly in a text editor. It is the lingua franca 
of 3D geometry formats because of its age and simplicity. You can integrate the 
format into your asset pipeline by producing your assets in a program such as 
Blender, serializing your assets use in the OBJ format, and then loading them 
into the end user application. The library supports this workflow.

## Getting Started
Add the following line to your `Cargo.toml` file

```toml
[dependencies]
wavefront_obj = "1.0.2"
```

to import the library. Include the line

```rust
extern crate wavefront_obj;
```

in your `main.rs` or `lib.rs` file. This is all you need to use the it.

## Usage
The library parses both **mtl** and **obj** files. In order to load a wavefront obj
file along with its modules, you need to import both the `mtl` and `obj` modules
separately.

```rust
use wavefront_obj::obj;
use wavefront_obj::mtl;
```

Each module has a top-level parsing function to parse a each file type. There are concrete
examples of explicit use of the module to parse OBJ and MTL files in the module documentation
as well as the `examples` directory in the source tree.

## Notes
* The Wavefront OBJ format does not contain information about how polygons of 
  vertex count larger than three should be be tessellated. It is up to the 
  modeler to communicate this information to the end users.
* All quadrilateral and higher vertex count faces must be either coplanar and 
  convex, or coplanar and concave; either condition is necessary to get a correct 
  rendering. Otherwise, the results of rendering will be unpredictable.

