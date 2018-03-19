# Wavefront OBJ Parser
### Introduction
This document contains the grammar extracted from the Wavefront OBJ file format specification used to implement the parser and lexer. See the docs subdirectory of the source tree for details.

### Grammar
The grammar is written in Backus-Naur form.
```
Comment            ::= '#' String '\n'
Whitespace         ::= [' ' | '\t' | Comment]+
Letter             ::= <Ascii Letters>
String             ::= [Letter]+
Digit              ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
Digits             ::= [Digit]+
Float              ::= ['-'] Digits '.' Digits
Vertex             ::= 'v' Float Float Float [Float]
TextureVertex      ::= 'vt' Float Float Float
NormalVertex       ::= 'vn' Float Float Float
ParameterVertex    ::= 'vp' Float [Float] [Float]
Number             ::= ['-'] Digits
V                  ::= Number
VN                 ::= Number '//' Number
VT                 ::= Number '/' Number
VTN                ::= Number '/' Number '/' Number
Point              ::= 'p' Number [Number]*
Line               ::= 'l' (VT VT [VT]+ | V V [V]+)
Face               ::= 'f' V V V [V]*
                     | 'f' VT VT VT [VT]*
                     | 'f' VN VN VN [VN]*
                     | 'f' VTN VTN VTN [VTN]*
Element            ::= Point | Line | Face
GroupName          ::= 'g' [String]*
SmoothingGroupName ::= 's' ('off' | 0 | Digits)
Group              ::= GroupName 

ObjectName         ::= 'o' String
ObjectSet          ::= [Object]*
```