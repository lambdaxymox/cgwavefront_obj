# Wavefront OBJ Parser
### Introduction
This document contains the grammar extracted from the Wavefront OBJ file format specification used to implement the parser and lexer.

### Grammar
```
ObjectSet ::= [Object]*
Object ::=

Comment ::= '#' String '\n'
Whitespace ::= [[' '] | Comment]+
Letter ::= <Ascii Letters>
String ::= [Letter]+
Digit ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
Digits ::= [Digit]+
Float ::= Digits '.' Digits
Vertex ::= 'v' Float Float Float [Float]
TextureVertex ::= 'vt' Float Float Float
NormalVertex ::= 'vn' Float Float Float
ParameterVertex ::= 'vp' Float [Float] [Float]
V ::= Digits
VN ::= Digits '//' Digits
VT ::= Digits '/' Digits
VTN ::= Digits '/' Digits '/' Digits
Point ::= 'p' Digits [Digits]*
Line ::= 'l' (VT VT [VT]+ | V V [V]+)
Face ::= 'f' V V V [V]*
       | 'f' VT VT VT [VT]*
       | 'f' VN VN VN [VN]*
       | 'f' VTN VTN VTN [VTN]*
Group ::= 'g' [String]*
SmoothingGroup ::= 's' ('off' | Digits)
ObjectName = 'o' String
```