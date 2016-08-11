#stones

An esoteric programming language

##To do

* [X] Math
* [ ] Arrays
* [ ] Stack operations
* [ ] More I/O
* [ ] Control flow

##Specification

Instructions in `stones` are carried out by moving colored stones around in a
hypothetical `field`. Each color of stone has a separate purpose, and each
movement a stone can carry out has a separate effect. For example, the purple
stone is for control flow. If you move the purple stone up by one unit, it
takes the top value off of the stack, and if it has a truthy value, it executes
instructions until it finds that the purple stone has moved down. If you move
the red stone down two units, it pushes a 5 on to the stack.

| Color      | Purpose                               | Up    | Down  | Left   | Right |
|------------|---------------------------------------|-------|------ |--------|-------|
| Red x 1    | Numbers/booleans                      | 0     | 1     | 2      | 3     |
| Red x 2    |                                       | 4     | 5     | 6      | 7     |
| Red x 3    |                                       | 8     | 9     | true   | false |
| Orange     | Array operations                      | [     | ]     | ,      | nth   |
| Yellow     | Math operators                        | \*    | +     | -      | /     |
| Green      | Stack operations                      | roll  | dup   | drop   | not   |
| Blue       | Input/output                          | print | input | printc | ?     |
| Purple     | Control flow                          | if    | else  | while  | end   |

As the program starts, the field is arranged as such:

```
b.....o....
...........
..r.....g..
...........
....y.....p
...........
```

The stones, represented by letters, move around on the field. When a stone hits
the edge of the field, it simply wraps around the edge of the field. For
example, moving the orange stone up from the default program produces this
result:

```
b..........
...........
..r.....g..
...........
....y..o..p
...........
```

Each stone has a different weight. The table with the stone colors above is
sorted by weight. As you can see, the lighter stones can move much further than
the heavier stones. This introduces an interesting property: When a stone runs
into another, it is possible that the stone can accidentally push it out of the
way. This can lead to interesting errors. Consider the following:

```
b.....o....     start
...........
..r.....g..
...........
....y.....p
...........

b.....o....
...........
....r...g..     red right 2: push 7
...........
....y.....p
...........

b.....o....
...........
......r.g..     red right 1: push 3
...........
....y.....p
...........

b.....o....
...........
......r.g..
....y......     yellow up 1: *
..........p
...........

b.....o....
...........
....r...g..     red left 2: push 6
....y......
..........p
...........
```

This example pushes a 7 and a 3 on to the stack, then multiplies them. If we
were then to try to push a 1 on to the stack, the operation would not occur
because the yellow stone is heavier than the red stone. If we tried to
multiply again, however, the red stone would be pushed by the yellow stone,
causing a 0 to be pushed on to the stack, then be multiplied by the 21
that is already there.

When a heavier stone collides with a lighter stone, the lighter stone moves
first, causing its action to occur, then the heavier stone moves, causing its
action to occur.

