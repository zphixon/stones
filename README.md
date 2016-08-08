#stones

An esoteric programming language

##Specification

Instrunctions in `stones` are carried out by moving colored stones around in a
hypothetical `field`. Each color of stone has a separate purpose, and each
movement a stone can carry out has a separate effect. For example, the purple
stone is for control flow. If you move the purple stone up by one unit, it
takes the top value off of the stack, and if it has a truthy value, it executes
instructions until it finds that the purple stone has moved down. If you move
the red stone down two units, it pushes a 5 on to the stack.

| Color  | Purpose      | Up | Down | Left | Right |
|--------|--------------|----|------|------|-------|
| Red    | Numbers      | 0  | 1    | 2    | 3     |
| Red\*2 |              | 4  | 5    | 6    | 7     |
| Red\*3 |              | 8  | 9    |      |       |
| Orange | Operations   | \* | +    | -    | /     |
| Yellow | Order of op. | (  | )    | (    | )     |
| Green  |              |    |      |      |       |
| Blue   |              |    |      |      |       |
| Purple | Control flow | if | end  | else | while |

