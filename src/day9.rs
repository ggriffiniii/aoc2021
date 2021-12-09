use std::{collections::HashSet, ops::Index};

use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.points_levels()
        .filter_map(|((col, row), level)| {
            let min_adjacent = grid
                .adjacent_points(col, row)
                .map(|(col, row)| grid[(col, row)])
                .min()
                .unwrap();
            if level < min_adjacent {
                Some(level as usize + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut basin_sizes: Vec<_> = grid
        .points_levels()
        .filter_map(|((col, row), level)| {
            let min_adjacent = grid
                .adjacent_points(col, row)
                .map(|(col, row)| grid[(col, row)])
                .min()
                .unwrap();
            if level < min_adjacent {
                Some(find_basin_size(&grid, col, row))
            } else {
                None
            }
        })
        .collect();
    basin_sizes.sort_unstable_by_key(|k| std::cmp::Reverse(*k));
    basin_sizes.iter().take(3).product()
}

fn find_basin_size(grid: &Grid, col: Col, row: Row) -> usize {
    fn _find_basin_size(grid: &Grid, col: Col, row: Row, seen: &mut HashSet<(Col, Row)>) -> usize {
        if seen.contains(&(col, row)) {
            return 0;
        }
        seen.insert((col, row));
        let level = grid[(col, row)];
        let mut basin_size = 1;
        for (adj_col, adj_row) in grid.adjacent_points(col, row) {
            let adj_level = grid[(adj_col, adj_row)];
            if adj_level != 9 && adj_level > level {
                basin_size += _find_basin_size(grid, adj_col, adj_row, seen);
            }
        }
        basin_size
    }
    _find_basin_size(grid, col, row, &mut HashSet::new())
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
            num_rows: self.num_rows(),
            num_cols: self.num_cols(),
            state: AdjacentIterState::Above,
        }
    }

    fn points_levels(&self) -> impl Iterator<Item = ((Col, Row), u8)> + '_ {
        self.data.iter().copied().enumerate().map(|(idx, v)| {
            let col = Col(idx % self.num_cols());
            let row = Row(idx / self.num_cols());
            ((col, row), v)
        })
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
