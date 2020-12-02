# Wavefront OBJ Parser Grammar

## Introduction
This document contains the grammar for the Wavefront OBJ and MTL file formats. 
The grammar was extracted from the Wavefront OBJ file format specification. 
The parser and lexer implement the grammar contained here. See the docs in the 
source tree for more specific details on the file format. The OBJ grammar 
specifies polygonal geometry only. It does not recognize free-form geometry. 
The MTL grammar only specifies which file goes with each texture map type, and 
does not implement any of the option flags.

## Grammar For OBJ Files
The grammar is written in Backus-Naur form. An `ObjectSet` is a collection of 
meshes that are extracted from a `*.obj` file. A mesh is composed of vertices, 
texture vertices, normal vertices, and geometry elements. Typically there is 
only one object in an object file, but there can be more than one. A wavefront 
obj file declares additional object in the same file by using an `o` statement 
for each object beyond the first one. The `o` statements are optional: it does 
not need to be stated in a file with only one element. The grammar is written 
bottom up; one infers the recursive descent structure of the parser by 
reading from the bottom to the top.

```
Empty           ::= ''
Digit           ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
Letter          ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' 
                  | 'k' | 'l' | 'm' | 'n' | 'o '| 'p' | 'q' | 'r' | 's' | 't' 
                  | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' 
                  | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' 
                  | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T'
                  | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z'
String          ::= [Letter]+ [Digit | Letter]*
Digits          ::= [Digit]+
Comment         ::= '#' String '\n'
Whitespace      ::= [' ' | '\t' | Comment]+
Number          ::= ['-'] Digits
Float           ::= Number '.' Digits
Vertex          ::= 'v' Float Float Float [Float]
TextureVertex   ::= 'vt' Float [Float] [Float]
NormalVertex    ::= 'vn' Float Float Float
ParameterVertex ::= 'vp' Float [Float] [Float]
AnyVertex       ::= Vertex | TextureVertex | NormalVertex | ParameterVertex
V               ::= Number
VN              ::= Number '//' Number
VT              ::= Number '/' Number
VTN             ::= Number '/' Number '/' Number
Point           ::= 'p' Number [Number]*
Line            ::= 'l' (VT VT [VT]+ | V V [V]+)
Face            ::= 'f' V V V [V]*
                  | 'f' VT VT VT [VT]*
                  | 'f' VN VN VN [VN]*
                  | 'f' VTN VTN VTN [VTN]*
Element         ::= Point | Line | Face
GroupName       ::= 'g' [String]*
SmoothingGroup  ::= 's' ('off' | 0 | Digits)
AnyElement      ::= [Element]* | SmoothingGroup [Element]*
Group           ::= GroupName [AnyVertex | AnyElement]*
ObjectName      ::= 'o' String
ObjectBody      ::= [AnyVertex | AnyElement]* [Group]*
Object          ::= ObjectName ObjectBody
ObjectSet       ::= [Object | ObjectBody] [Object]*
```

## Grammar For MTL Files
The grammar is written in Backus-Naur form. A `MtlSet` is a collection of 
materials that are extracted from a `*.mtl` file. Each material is composed of 
multiple illumination model parameters and texture maps.

```
Empty              ::= ''
Digit              ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
Letter             ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' 
                     | 'k' | 'l' | 'm' | 'n' | 'o '| 'p' | 'q' | 'r' | 's' | 't' 
                     | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' 
                     | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' 
                     | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T'
                     | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z'
String             ::= [Letter]+ [Digit | Letter]*
Digits             ::= [Digit]+
Whitespace         ::= [' ' | '\t' ]+
Number             ::= Digits
Float              ::= Number '.' Digits
NewMtl             ::= "newmtl"
Ka                 ::= "Ka"
Kd                 ::= "Kd"
Ks                 ::= "Ks"
Ke                 ::= "Ke"
Ns                 ::= "Ns"
Ni                 ::= "Ni"
Tf                 ::= "Tf"
Illum              ::= "illum"
Dissolve           ::= "d"
Bump               ::= "bump"
Disp               ::= "disp"
MapKa              ::= "map_Ka"
MapKd              ::= "map_Kd"
MapKs              ::= "map_Ks"
MapKe              ::= "map_Ke"
MapNs              ::= "map_Ns"
MapDissolve        ::= "map_d"
MapBump            ::= "map_Bump"

AmbientComponent   ::= Ka Float Float Float
DiffuseComponent   ::= Kd Float Float Float
SpecularComponent  ::= Ks Float Float Float
EmissiveComponent  ::= Ke Float Float Float
TransmissionFilter ::= Tf Float Float Float
IlluminationModel  ::= Illum Number
DissolveComponent  ::= Dissolve Float
SpecularExponent   ::= Ns Float
OpticalDensity     ::= Ni Float

AmbientMap         ::= MapKa String
DiffuseMap         ::= MapKd String
SpecularMap        ::= MapKs String
EmissiveMap        ::= MapKe String
BumpMap            ::= MapBump String | Bump String
DisplacementMap    ::= Disp String
DissolveMap        ::= MapDissolve String

MtlName            ::= NewMtl String
MaterialProperties ::= 
                     { AmbientComponent 
                     | DiffuseComponent 
                     | SpecularComponent 
                     | EmissiveComponent 
                     | TransmissionFilter 
                     | DissolveComponent 
                     | IlluminationModel
                     | SpecularExponent 
                     | OpticalDensity 
                     | AmbientMap
                     | DiffuseMap
                     | SpecularMap
                     | EmissiveMap
                     | BumpMap
                     | DisplacementMap
                     | DissolveMap
                     }*
Material           ::= MtlName MaterialProperties
MtlSet             ::= [Material]*
```

## Notation
There are several extragrammatical symbols used in the grammar.
* A `[...]`  without a `*` or `+` suffix denotes an optional field (zero or 
             one instances).
* A `[...]*` means zero or more symbols of that form.
* A `[...]+` means one or more symbols of that form.
* A `(...)`  exists to disambiguate how to group a set of symbols. It lies 
             outside the grammar itself.
* A `[...]`  has no other suffixes, and a `(...)` has no suffixes.
* A `{...}`  indicates that the enclosed terms may appear at most once in any order.
