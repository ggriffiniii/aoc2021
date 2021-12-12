use std::{cell::Cell, ops::Index};

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&grid);
    }
    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    for step_num in 1.. {
        step(&grid);
        if grid
            .points_levels()
            .filter(|(_, level)| level.get() != 0)
            .count()
            == 0
        {
            return step_num;
        }
    }
    unreachable!();
}

fn step(grid: &Grid) -> usize {
    let mut flashes = 0;
    for ((col, row), level) in grid.points_levels() {
        level.set(level.get() + 1);
        if level.get() == 10 {
            flashes += flash(grid, col, row);
        }
    }
    for (_, level) in grid.points_levels() {
        if level.get() > 9 {
            level.set(0);
        }
    }
    flashes
}

fn flash(grid: &Grid, col: Col, row: Row) -> usize {
    let mut flashes = 1;
    for (adj_col, adj_row) in grid.adjacent_points(col, row) {
        let level = &grid[(adj_col, adj_row)];
        level.set(level.get() + 1);
        if level.get() == 10 {
            flashes += flash(grid, adj_col, adj_row);
        }
    }
    flashes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Row(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Col(usize);

#[derive(Debug)]
struct Grid {
    data: Vec<Cell<u8>>,
    row_len: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let (first_row, _) = input.split_once('\n').unwrap();
        let row_len = first_row.len();
        let data = input
            .bytes()
            .filter(|&n| n != b'\n')
            .map(|b| Cell::new(b - b'0'))
            .collect();
        Grid { data, row_len }
    }

    fn adjacent_points(&self, col: Col, row: Row) -> AdjacentIter {
        AdjacentIter {
            col,
            row,
            num_rows: self.data.len() / self.row_len,
            num_cols: self.row_len,
            state: AdjacentIterState::Above,
        }
    }

    fn points_levels(&self) -> impl Iterator<Item = ((Col, Row), &Cell<u8>)> + '_ {
        self.data.iter().enumerate().map(|(idx, v)| {
            let col = Col(idx % self.row_len);
            let row = Row(idx / self.row_len);
            ((col, row), v)
        })
    }
}

impl Index<(Col, Row)> for Grid {
    type Output = Cell<u8>;

    fn index(&self, (col, row): (Col, Row)) -> &Self::Output {
        &self.data[row.0 * self.row_len + col.0]
    }
}

#[derive(Debug)]
struct AdjacentIter {
    col: Col,
    row: Row,
    num_rows: usize,
    num_cols: usize,
    state: AdjacentIterState,
}
impl Iterator for AdjacentIter {
    type Item = (Col, Row);
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            AdjacentIterState::Above => {
                self.state = AdjacentIterState::AboveLeft;
                if self.row.0 > 0 {
                    Some((self.col, Row(self.row.0 - 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::AboveLeft => {
                self.state = AdjacentIterState::Left;
                if self.row.0 > 0 && self.col.0 > 0 {
                    Some((Col(self.col.0 - 1), Row(self.row.0 - 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Left => {
                self.state = AdjacentIterState::BelowLeft;
                if self.col.0 > 0 {
                    Some((Col(self.col.0 - 1), self.row))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::BelowLeft => {
                self.state = AdjacentIterState::Below;
                if self.col.0 > 0 && self.row.0 + 1 < self.num_rows {
                    Some((Col(self.col.0 - 1), Row(self.row.0 + 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Below => {
                self.state = AdjacentIterState::BelowRight;
                if self.row.0 + 1 < self.num_rows {
                    Some((self.col, Row(self.row.0 + 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::BelowRight => {
                self.state = AdjacentIterState::Right;
                if self.row.0 + 1 < self.num_rows && self.col.0 + 1 < self.num_cols {
                    Some((Col(self.col.0 + 1), Row(self.row.0 + 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Right => {
                self.state = AdjacentIterState::AboveRight;
                if self.col.0 + 1 < self.num_cols {
                    Some((Col(self.col.0 + 1), self.row))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::AboveRight => {
                self.state = AdjacentIterState::Done;
                if self.col.0 + 1 < self.num_cols && self.row.0 > 0 {
                    Some((Col(self.col.0 + 1), Row(self.row.0 - 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Done => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AdjacentIterState {
    Above,
    AboveLeft,
    Left,
    BelowLeft,
    Below,
    BelowRight,
    Right,
    AboveRight,
    Done,
}
