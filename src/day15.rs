use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use aoc_runner_derive::aoc;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    find_lowest_cost(&Grid::new(input))
}

fn find_lowest_cost(grid: &Grid) -> usize {
    let start = (Col(0), Row(0));
    let finish = (Col(grid.num_cols() - 1), Row(grid.num_rows() - 1));
    let mut costs = HashMap::new();
    let mut visited = HashSet::new();
    for (col, row) in grid.adjacent_points(start.0, start.1) {
        costs.insert((col, row), grid[(col, row)] as usize);
    }
    visited.insert(start);
    while !visited.contains(&finish) {
        let (&(curr_col, curr_row), &curr_cost) = costs
            .iter()
            .filter(|(pos, _cost)| !visited.contains(pos))
            .min_by_key(|&(&_pos, &cost)| cost)
            .unwrap();
        for (col, row) in grid.adjacent_points(curr_col, curr_row) {
            let entry = costs.entry((col, row)).or_insert(usize::MAX);
            *entry = (*entry).min(curr_cost + grid[(col, row)] as usize);
        }
        visited.insert((curr_col, curr_row));
    }
    costs[&finish]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Row(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Col(usize);

#[derive(Debug)]
struct Grid {
    data: Vec<u8>,
    row_len: usize,
}
impl Grid {
    fn new(input: &str) -> Self {
        let (first_row, _) = input.split_once('\n').unwrap();
        let row_len = first_row.len();
        let data = input
            .bytes()
            .filter(|&n| n != b'\n')
            .map(|b| b - b'0')
            .collect();
        Grid { data, row_len }
    }

    fn num_rows(&self) -> usize {
        self.data.len() / self.row_len
    }

    fn num_cols(&self) -> usize {
        self.row_len
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
}

impl Index<(Col, Row)> for Grid {
    type Output = u8;

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
                self.state = AdjacentIterState::Left;
                if self.row.0 > 0 {
                    Some((self.col, Row(self.row.0 - 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Left => {
                self.state = AdjacentIterState::Below;
                if self.col.0 > 0 {
                    Some((Col(self.col.0 - 1), self.row))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Below => {
                self.state = AdjacentIterState::Right;
                if self.row.0 + 1 < self.num_rows {
                    Some((self.col, Row(self.row.0 + 1)))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Right => {
                self.state = AdjacentIterState::Done;
                if self.col.0 + 1 < self.num_cols {
                    Some((Col(self.col.0 + 1), self.row))
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
    Left,
    Below,
    Right,
    Done,
}
