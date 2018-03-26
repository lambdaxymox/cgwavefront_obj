# Wavefront OBJ Library
The wavefront-obj library is a library for working with Wavefront OBJ files. Wavefront OBJ files are a file format for representing three-dimensional meshes. In particular, the library parses and represents Wavefront OBJ meshes. It presently supports polygonal geometry. Any file containing free-form geometry elements will be rejected. The grammar as it currently stands is stored in the `GRAMMAR.md`. A copy of the file format specification is included in the `docs` directory.

### Dependencies
For release, the `wavefront-obj` library has no external dependencies. It requires `rustc` version 1.24 stable or greater to compile, along with a recent version of `cargo`. For testing and benchmarking, the library relies on the `criterion` library.

# Notes
* The Wavefront OBJ format does not preserve information about how the mesh should be tesselated. It is necessary that all quadrilateral and higher vertex count faces be coplanar and convex, or coplanar and concave, to get good results with rendering. Otherwise, there is no way to know how the model was intended to be tesslated, and unpredictable results may occur.