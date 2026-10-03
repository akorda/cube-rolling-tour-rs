# Cube-Rolling Tours

This is problem number 9 from `Scientific American` magazine, issue _1965-11_. The solution appears in issue _1965-12_. Additionally, it appears in book `Mathematical Carnival` also by [Martin Gardner](https://en.wikipedia.org/wiki/Martin_Gardner).

---

Recreational mathematicians have devoted much attention in the past to chessboard "tours" in which a chess
piece is moved over the board to visit each square once and only once, in compliance with various constraints.
Last year John Harris of Santa Barbara, Calif., devised a fascinating new kind of tour-the "cube-rolling tour"-that
opens up a wealth of possibilities.

To work on two of Harris' best problems, obtain a small wooden cube from a set of children's blocks or make one
of cardboard. Its sides should be about the same size as the squares of your chessboard or checkerboard. Paint one
side red. The cube is moved from one square to an adjacent one by being tipped over an edge, the edge resting
on the line dividing the two cells. During each move, therefore, the cube makes one quarter-turn in a north,
south, east or west direction.

## Problem 1

Place the cube on the northwest corner of the board, red side up. Tour the board, resting once only
on every cell and ending with the cube red side up in the northeast corner. At no time during the tour,
however, is the cube allowed to rest with the red side up. (NOTE: It is not possible to make such a tour
from corner to diagonally opposite corner.)

## Problem 2

Place the cube on any cell, an uncolored side up. Make a "reentrant tour" of the board (one that
visits every cell once and returns the cube to its starting square) in such a way that at no time during the tour,
including at the finish, will the cube's red side be up.

Both problems have unique solutions, not counting rotations and reflections of the path.

## Rusty Cubes

In this repo a solver of _Problem 1_ aka `Red-Faced Cube` is implemented in [rust](https://rust-lang.org/).

> I just realized that we can create boards with sizes other than 8x8. In these cases, there may exist more than one solutions. The current implementation exits when finding the first one.

Sample solutions are shown in this [file](docs/sample_solutions.md).
