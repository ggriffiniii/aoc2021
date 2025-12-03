use aoc_runner_derive::aoc;

use std::fmt;

const EXAMPLE_INPUT: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LocationState {
    East = b'>',
    South = b'v',
    Empty = b'.',
}
impl fmt::Display for LocationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8 as char)
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<LocationState>>);

impl Grid {
    fn new(input: &str) -> Self {
        Grid(
            input
                .as_bytes()
                .split(|&b| b == b'\n')
                .map(|bytes| {
                    bytes
                        .iter()
                        .map(|&b| match b {
                            b'>' => LocationState::East,
                            b'v' => LocationState::South,
                            b'.' => LocationState::Empty,
                            x => panic!("invalid input: {x}"),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn tick(&mut self) -> usize {
        let mut num_moves = 0;
        let row_count = self.0.len();
        let col_count = self.0[0].len();

        let initial_state = self.0.clone();

        // move east
        for rowi in 0..row_count {
            for coli in 0..col_count {
                if initial_state[rowi][coli] != LocationState::East {
                    continue;
                }

                let neighbor_col = (coli + 1) % col_count;
                if initial_state[rowi][neighbor_col] == LocationState::Empty {
                    self.0[rowi][neighbor_col] = LocationState::East;
                    self.0[rowi][coli] = LocationState::Empty;
                    num_moves += 1;
                }
            }
        }

        let initial_state = self.0.clone();

        // move south
        for rowi in 0..row_count {
            for coli in 0..col_count {
                if initial_state[rowi][coli] != LocationState::South {
                    continue;
                }

                let neighbor_row = (rowi + 1) % row_count;
                if initial_state[neighbor_row][coli] == LocationState::Empty {
                    self.0[neighbor_row][coli] = LocationState::South;
                    self.0[rowi][coli] = LocationState::Empty;
                    num_moves += 1;
                }
            }
        }
        num_moves
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0.iter() {
            for location in row.iter() {
                write!(f, "{location}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let mut num_ticks = 0;
    loop {
        let num_moves = grid.tick();
        num_ticks += 1;
        if num_moves == 0 {
            break num_ticks;
        }
    }
}

#[aoc(day25, part2)]
fn part2(input: &str) -> String {
    "unnecesary".to_string()
}
