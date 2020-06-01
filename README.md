# SAT solver exercises

This project is a personal exercise for learning about working with a SAT
solver, and for getting practice with Rust. This is essentially a Rust port of
the code from
[Modern SAT solvers: fast, neat and underused (part 1 of N)][article]
by Martin Hořeňovský.

[article]: https://codingnest.com/modern-sat-solvers-fast-neat-underused-part-1-of-n/

The program reads a Sudoku puzzle from stdin, and writes the solution to stdout.

For example,

    $ <data/puzzle_01 cargo run

Puzzles are given in an ASCII art format. The format is flexible: the
requirements are that:

- each row of the puzzle is given as one line of input
- each cell is surrounded on the left and right by non-whitespace, non-numeric
  characters
- cells containing numbers are represented by numbers with optional whitespace
- empty cells are represented by whitespace

Row-separator lines are optional.

Here are some examples of valid puzzle inputs:

    ┌──┬──┬──┰──┬──┬──┰──┬──┬──┐
    │ 5│ 3│  ┃  │ 7│  ┃  │  │  │
    ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
    │ 6│  │  ┃ 1│ 9│ 5┃  │  │  │
    ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
    │  │ 9│ 8┃  │  │  ┃  │ 6│  │
    ┝━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┥
    │ 8│  │  ┃  │ 6│  ┃  │  │ 3│
    ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
    │ 4│  │  ┃ 8│  │ 3┃  │  │ 1│
    ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
    │ 7│  │  ┃  │ 2│  ┃  │  │ 6│
    ┝━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┥
    │  │ 6│  ┃  │  │  ┃ 2│ 8│  │
    ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
    │  │  │  ┃ 4│ 1│ 9┃  │  │ 5│
    ├──┼──┼──╂──┼──┼──╂──┼──┼──┤
    │  │  │  ┃  │ 8│  ┃  │ 7│ 9│
    └──┴──┴──┸──┴──┴──┸──┴──┴──┘

    +--+--+--+--+--+--+--+--+--+
    | 5| 3|  |  | 7|  |  |  |  |
    +--+--+--+--+--+--+--+--+--+
    | 6|  |  | 1| 9| 5|  |  |  |
    +--+--+--+--+--+--+--+--+--+
    |  | 9| 8|  |  |  |  | 6|  |
    +--+--+--+--+--+--+--+--+--+
    | 8|  |  |  | 6|  |  |  | 3|
    +--+--+--+--+--+--+--+--+--+
    | 4|  |  | 8|  | 3|  |  | 1|
    +--+--+--+--+--+--+--+--+--+
    | 7|  |  |  | 2|  |  |  | 6|
    +--+--+--+--+--+--+--+--+--+
    |  | 6|  |  |  |  | 2| 8|  |
    +--+--+--+--+--+--+--+--+--+
    |  |  |  | 4| 1| 9|  |  | 5|
    +--+--+--+--+--+--+--+--+--+
    |  |  |  |  | 8|  |  | 7| 9|
    +--+--+--+--+--+--+--+--+--+

    |5|3| | |7| | | | |
    |6| | |1|9|5| | | |
    | |9|8| | | | |6| |
    |8| | | |6| | | |3|
    |4| | |8| |3| | |1|
    |7| | | |2| | | |6|
    | |6| | | | |2|8| |
    | | | |4|1|9| | |5|
    | | | | |8| | |7|9|
