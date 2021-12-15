use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    ops::Index,
};

use aoc_runner_derive::aoc;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    find_lowest_cost(&Grid::new(input))
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let grid_data: Vec<u8> = input
        .split('\n')
        .flat_map(|line| {
            (0..5).flat_map(|i| {
                line.as_bytes()
                    .iter()
                    .copied()
                    .map(|b| b - b'0')
                    .map(move |b| ((b - 1) + i) % 9 + 1)
            })
        })
        .collect();
    let grid_data: Vec<u8> = (0..5)
        .flat_map(|i| {
            grid_data
                .iter()
                .copied()
                .map(move |b| ((b - 1) + i) % 9 + 1)
        })
        .collect();
    let grid = Grid {
        data: grid_data,
        row_len: input.find('\n').unwrap() * 5,
    };
    find_lowest_cost(&grid)
}

fn find_lowest_cost(grid: &Grid) -> usize {
    let mut heap = BinaryHeap::new();
    let start = (Col(0), Row(0));
    let finish = (Col(grid.num_cols() - 1), Row(grid.num_rows() - 1));
    let mut costs = HashMap::new();
    costs.insert(start, 0);
    heap.push(State {
        cost: 0,
        pos: start,
    });
    while let Some(State { cost, pos }) = heap.pop() {
        if pos == finish {
            return cost;
        }
        if cost > costs.get(&pos).copied().unwrap_or(usize::MAX) {
            continue;
        }
        for pos in grid.adjacent_points(pos.0, pos.1) {
            let next = State {
                cost: cost + grid[pos] as usize,
                pos,
            };
            if next.cost < costs.get(&pos).copied().unwrap_or(usize::MAX) {
                heap.push(next);
                costs.insert(next.pos, next.cost);
            }
        }
    }
    panic!("no path exists")
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: (Col, Row),
    cost: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
