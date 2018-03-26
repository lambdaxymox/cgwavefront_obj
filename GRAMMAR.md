# Wavefront OBJ Parser Grammar

### Introduction
This document contains the grammar extracted from the Wavefront OBJ file format specification used to implement the parser and lexer. See the docs subdirectory of the source tree for details. At this time the grammar centers mainly on parsing polygonal geometry, not free-form geometry. The grammar will not parse free-form geometry data.

### Grammar
The grammar is written in Backus-Naur form. An `ObjectSet` is the collection of meshes that are extracted from a *.obj file. Typically there is only one object in an object set, but there can be more than one if the `o` tag is used to split them up.
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

### Notation
The following notation describes the extragrammatical symbols used in the grammar.
* A `[...]` without a `*` or `+` suffix denotes an optional field (zero or one instances).
* A `[...]*` means zero or more symbols of that form.
* A `[...]+` means one or more symbols of that form.
* A `(...)` exists to disambiguate how to group a set of symbols. It lies outside the grammar per se.
* A `[...]` has no other suffixes, and a `(...)` has no suffixes.