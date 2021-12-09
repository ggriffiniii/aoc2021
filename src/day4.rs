use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug)]
struct Marked(u32);

const fn bitmask_for_row(row: u8) -> u32 {
    0b11111 << (row * 5)
}

#[allow(clippy::unusual_byte_groupings)]
const fn bitmask_for_col(col: u8) -> u32 {
    0b00001_00001_00001_00001_00001 << col
}

#[derive(Debug, Clone, Copy)]
struct RowCol {
    row_idx: u8,
    col_idx: u8,
}

#[derive(Debug)]
struct Board {
    numbers: HashMap<u8, RowCol>,
    marked: u32,
    has_won: bool,
}
impl Board {
    fn from_input(input: &str) -> Self {
        let mut numbers = HashMap::new();
        for (row_idx, row) in input.split('\n').enumerate() {
            for (col_idx, number) in row.split_ascii_whitespace().enumerate() {
                numbers.insert(
                    number.parse().unwrap(),
                    RowCol {
                        row_idx: row_idx as u8,
                        col_idx: col_idx as u8,
                    },
                );
            }
        }
        Board {
            numbers,
            marked: 0,
            has_won: false,
        }
    }

    fn is_marked(&self, pos: RowCol) -> bool {
        self.marked & (1 << (5 * pos.row_idx + pos.col_idx)) != 0
    }

    fn mark(&mut self, pos: RowCol) {
        self.marked |= 1 << (5 * pos.row_idx + pos.col_idx);
        self.has_won =
            self.has_won || self.row_complete(pos.row_idx) || self.col_complete(pos.col_idx);
    }

    fn has_won(&self) -> bool {
        self.has_won
    }

    fn row_complete(&self, row: u8) -> bool {
        let mask = bitmask_for_row(row);
        self.marked & mask == mask
    }

    fn col_complete(&self, col: u8) -> bool {
        let mask = bitmask_for_col(col);
        self.marked & mask == mask
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    let mut records = input.split("\n\n");
    let drawn_numbers = records
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| -> u8 { s.parse().unwrap() });

    let mut boards: Vec<_> = records.map(Board::from_input).collect();

    for number in drawn_numbers {
        for board in boards.iter_mut() {
            if let Some(pos) = board.numbers.get(&number).copied() {
                board.mark(pos);
                if board.has_won() {
                    return board
                        .numbers
                        .iter()
                        .filter_map(|(&n, &pos)| {
                            if !board.is_marked(pos) {
                                Some(n as u64)
                            } else {
                                None
                            }
                        })
                        .sum::<u64>()
                        * number as u64;
                }
            }
        }
    }
    panic!("no winner");
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    let mut records = input.split("\n\n");
    let drawn_numbers = records
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| -> u8 { s.parse().unwrap() });

    let mut boards: Vec<_> = records.map(Board::from_input).collect();

    for number in drawn_numbers {
        let num_boards = boards.len();
        for board in boards.iter_mut() {
            if let Some(pos) = board.numbers.get(&number).copied() {
                board.mark(pos);
                if num_boards == 1 && board.has_won() {
                    return board
                        .numbers
                        .iter()
                        .filter_map(|(&n, &pos)| {
                            if !board.is_marked(pos) {
                                Some(n as u64)
                            } else {
                                None
                            }
                        })
                        .sum::<u64>()
                        * number as u64;
                }
            }
        }
        boards = boards
            .into_iter()
            .filter(|board| !board.has_won())
            .collect();
    }
    panic!("no loser")
}
