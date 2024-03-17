# N-Puzzle

```sh
$ ./n_puzzle 3 --verbose
Complexity in time: 4
Complexity in size: 4
Elapsed time: 0.000435 seconds
Number of moves: 1
Complexity in time: 1905
Complexity in size: 1880
Elapsed time: 0.004878 seconds
Number of moves: 25
3 8 7
5 4 0
2 1 6
↓ Left

...

↓ Right
1 2 3
8 0 4
7 6 5
```

## Overview

This program solves the N Puzzle, a sliding puzzle that requires arranging numbered tiles in a specific pattern.  
The goal is to achieve a configuration like this:

- 3x3 board:
  ```
  1 2 3
  8 0 4
  7 6 5
  ```
- 4x4 board:
  ```
   1  2  3  4
  12 13 14  5
  11  0 15  6
  10  9  8  7
  ```
- 5x5 board:
  ```
   1  2  3  4  5
  16 17 18 19  6
  15 24  0 20  7
  14 23 22 21  8
  13 12 11 10  9
  ```

- nxn board: ...

## Requirement

- rustc: 1.76.0

## Usage

After cloning the repository, build the project with the following command:

```sh
make
```

You can check how to run the program by executing:

```sh
$ ./n_puzzle
usage: ./n_puzzle (file | size) [-a algorithm] [-h heuristic] [-t timeout] [--verbose]
```

For a quick start, try running the following:

```sh
$ ./n_puzzle 3
Complexity in time: 684
Complexity in size: 675
Elapsed time: 0.003156 seconds
Number of moves: 19
6 0 3
1 8 2
5 7 4
Moves: Left Down Right Up Left Down Right Down Left Up Right Right Down Left Left Up Up Right Down
```

Here are the details on the command options:

- **file**: Specifies a puzzle file in .txt format.
  - Some examples are available in the `puzzles/` directory.
- **size**: Creates and solves a puzzle of the specified size.
  - Note: Sizes 4 and above may take a long time to solve.
- **algorithm**: Choose the solving algorithm from the following:
  - astar
  - uniform_cost
  - greedy
- **heuristic**: Choose the heuristic function from the following:
  - manhattan
  - hamming
  - linear_conflict
  - inversion_distance
- **timeout**: Prevents the program from running indefinitely by specifying a time limit in seconds.
- **verbose**: Enables detailed output.

## Features

### Input File

- Any text following a `#` within a line is considered a comment and is ignored.
- The first number specifies the size (N) for the N x N puzzle.

Example:

```
# Puzzle!!
3
3 6 0
8 5 7
2 4 1
```

### Puzzle

- Accepts puzzles ranging from 2 x 2 up to 100 x 100 in size.
    - However, for puzzles 4 x 4 and larger, solving may take a considerable amount of time unless they are simple.

### Output

At the end of the program, the following information is output:

- If the puzzle is unsolvable, it states that the puzzle cannot be solved.
- If the puzzle is solvable, it provides the following details:
    - Complexity in time: The total number of states considered.
    - Complexity in size: The total number of states expanded at the same time.
    - Elapsed time: The total time taken to solve the puzzle.
    - Number of moves: The number of moves from the initial state to the final state as determined by the search.
    - Moves: The sequence of directions to move from the initial state to the final state as determined by the search.

## Author

- [ksuzuki(kota)](https://twitter.com/Kotabrog)
- [yahokari](https://github.com/Shigurex)
