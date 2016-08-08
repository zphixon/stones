#stones

An esoteric programming language

##Specification

Instrunctions in `stones` are carried out by moving colored stones around in a
hypothetical `field`. Each color of stone has a separate purpose, and each
movement a stone can carry out has a separate effect. For example, the purple
stone is for control flow. If you move the purple stone up, it
takes the top value off of the stack, and if it has a truthy value, it executes
instructions until it finds that the purple stone has moved down.

| Stone color | Purpose | Movements                |
|-------------|---------|----|------|------|-------|
|             |         | Up | Down | Left | Right |
|-------------|---------|----|------|------|-------|

