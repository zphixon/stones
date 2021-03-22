# stones

[![Freaking travis](https://travis-ci.org/zphixon/stones.svg?branch=master)](https://travis-ci.org/zphixon/stones) [![Freaking appveyor](https://ci.appveyor.com/api/projects/status/120smgk90ltqhopc?svg=true)](https://ci.appveyor.com/project/zphixon/stones)

An esoteric programming language

There are two interpreters of the stones programming language.
Go [here](https://github.com/zphixon/stones-rewrite) for the scripted
interpreter.

## Specification

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
| Orange x 1 | Array operations, boolean             | [     | ]     | ,      | nth   |
| Orange x 2 |                                       | ==    | <     | >      |       |
| Yellow     | Math operators                        | \*    | +     | -      | /     |
| Green      | Stack operations                      | roll  | dup   | drop   | not   |
| Blue       | Input/output                          | print | input | printc | swap  |
| Purple     | Control flow                          | if    | else  | while  | end   |

As the program starts, the field is arranged as such:

```
b.....o.....
............
..r.....g...
............
....y.....p.
............
```

The stones, represented by letters, move around on the field. When a stone hits
the edge of the field, it simply wraps around the edge of the field. For
example, moving the orange stone up from the default program produces this
result:

```
b...........
............
..r.....g...
............
....y.....p.
.......o....
```

Each stone has a different weight, shown in the table above in increasing order.
Stones are unable to occupy the same spot on the field due to an unfortunate quirk
of physics. Consider the following program:

```
b.....o.....     start
............
..r.....g...
............
....y.....p.
............

b.....o.....
............
....r...g...     red right two: push 7
............
....y.....p.
............

b.....o.....
............
........g...     red down one: push 1
....r.......
....y.....p.
............

```

This example pushes a 7 and a 1 on the stack. If we were to attempt `red down one`
to push an additional 1 on the stack, the operation will not occur. The yellow
stone is heavier than the red stone and thus cannot be moved by it. Additionally,
the operation `yellow up` will not immediately occur because the red stone is
in the way. First, `red up one` will occur, pushing a zero on the stack, and
then `yellow up` will occur, multiplying 1 by 0.

In short, when a heavier stone collides with a lighter stone, the lighter stone moves
first, causing its action to occur, then the heavier stone moves, causing its
action to occur. When a lighter stone collides with a heavier stone, nothing
occurs.

Only the following keywords are valid stones code, any other sequence will be ignored:
* `red`
* `orange`
* `yellow`
* `green`
* `blue`
* `purple`
* `up`
* `down`
* `left`
* `right`
* `one`
* `two`
* `three`

