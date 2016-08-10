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

| Color      | Purpose                               | Up    | Down  | Left  | Right |
|------------|---------------------------------------|-------|------ |-------|-------|
| Red x 1    | Numbers                               | 0     | 1     | 2     | 3     |
| Red x 2    |                                       | 4     | 5     | 6     | 7     |
| Red x 3    |                                       | 8     | 9     | true  | false |
| Orange x 1 | Array operations, strings             | arr   | [     | ]     | ""    |
| Orange x 2 | Array operations, strings             | nth   | app   | head  | tail  |
| Yellow     | Math operators                        | \*    | +     | -     | /     |
| Green      | Stack operations, order of operations | roll  | dup   | (     | )     |
| Blue       | Input/output, two macros              | print | input | ?     | ?     |
| Purple     | Control flow                          | if    | else  | while | end   |

As the program starts, the field is arranged as such:

```
b.....o....
..r.....g..
....y.....p
```

The stones, represented by letters, move around on the field. When a stone hits
the edge of the field, it simply wraps around the edge of the field. For
example, moving the orange stone up from the default program produces this
result:

```
b..........
..r.....g..
....y..o..p
```

Each stone has a different weight. The table with the stone colors above is
sorted by weight. As you can see, the lighter stones can move much further than
the heavier stones. This introduces an interesting property: When a stone runs
into another, it is possible that the stone can accidentally push it out of the
way. This can lead to interesting errors. Consider the following:

```
b.....o....     start
..r.....g..
....y.....p

b.....o....
...r....g..     red right 1: 3
....y.....p

b.....o....
........g..
...ry.....p     right up 2: 4

b..........
........g..
...ry.o...p     orange up: *
```

This example pushes 3 on to the stack, pushes 4 on to the stack, and multiplies
the two, pushing 12 on to the stack. Now suppose we wanted to push a 7 on to
the stack. This would mean the red stone would collide with the yellow stone.

However, since the red stone is lighter than the yellow stone, the instruction
will not take place.

