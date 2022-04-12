# stones

A bytecode-compiled esoteric programming language.

[Original implementation](https://github.com/zphixon/stones-rewrite) - [Non-bytecode](https://github.com/zphixon/stones/tree/e040305b8eec695b0cbbb0647f6e00af4b33597f)

## Specification

Instructions in stones are carried out by moving colored stones around in a hypothetical field.
Each color of stone has a separate purpose, and each movement a stone can carry out has a separate
effect. For example, moving the red stone upward twice pushes an 8, moving it right once pushes a
three, and then moving the yellow stone down pops those two values and pushes their sum.

| Color             | Purpose            | Up    | Down  | Left   | Right |
|-------------------|--------------------|-------|------ |--------|-------|
| `red ____ one`    | Constants          | 0     | 1     | 2      | 3     |
| `red ____ two`    |                    | 4     | 5     | 6      | 7     |
| `red ____ three`  |                    | 8     | 9     | true   | false |
| `orange ____ one` | Array operations   | [     | ]     | ,      | nth   |
| `orange ____ two` | Comparisons        | ==    | <     | >      |       |
| `yellow ____`     | Math operators     | \*    | +     | -      | /     |
| `green ____`      | Stack manipulation | roll  | dup   | drop   | not   |
| `blue ____`       | Input/output       | print | input | printc | swap  |
| `purple ____`     | Control flow       | if    | else  | while  | end   |

As the program starts, the field is arranged as such:

```
b.....o.....
............
..r.....g...
............
....y.....p.
............
```

The stones, represented by letters, move around on the field. When a stone hits the edge of the
field, it simply wraps around the edge of the field. For example, moving the orange stone up from
the default program produces this result:

```
b...........
............
..r.....g...
............
....y.....p.
.......o....
```

Each stone has a different weight, shown in the table above in increasing order.  Stones are unable
to occupy the same spot on the field due to an unfortunate quirk of physics. Consider the following
program:

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

This example pushes a 7 and a 1 on the stack. If we were to attempt `red down one` to push an
additional 1 on the stack, the operation will not occur. The yellow stone is heavier than the red
stone and thus cannot be moved by it. Additionally, the operation `yellow up` will not immediately
occur because the red stone is in the way. First, `red up one` will occur, pushing a zero on the
stack, and then `yellow up` will occur, multiplying 1 by 0.

In short, when a heavier stone collides with a lighter stone, the lighter stone moves first,
causing its action to occur, then the heavier stone moves, causing its action to occur. When a
lighter stone collides with a heavier stone, nothing occurs.

When a heavier stone moves the orange or red stone, their actions are always the single-move
actions (e.g. `blue up` results in `red up one`). However, for the edge case of the orange stone
moving twice (say, `orange down two`) and the red stone being pushed twice, the authors reserve the
right to change the behavior of the red stone to execute the two-move action rather than the
one-move action twice.

Notably, since the purple stone is the heaviest, there is no way to accidentally destroy your
control flow.

Only the following whitespace-delimited keywords are valid stones code, any other sequence will be
ignored. This makes it possible to include stones in a polyglot program.

`red orange yellow green blue purple up down left right one two three`

