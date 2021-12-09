use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut risk_level = 0;
    for row in 0..grid.num_rows() {
        for col in 0..grid.num_cols() {
            let level = grid.get(col, row);
            let min_adjacent = grid
                .adjacent_points(col, row)
                .map(|(col, row)| grid.get(col, row))
                .min()
                .unwrap();
            if level < min_adjacent {
                risk_level += level as usize + 1;
            }
        }
    }
    risk_level
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut basin_sizes = Vec::new();
    for row in 0..grid.num_rows() {
        for col in 0..grid.num_cols() {
            let level = grid.get(col, row);
            let min_adjacent = grid
                .adjacent_points(col, row)
                .map(|(col, row)| grid.get(col, row))
                .min()
                .unwrap();
            if level < min_adjacent {
                basin_sizes.push(find_basin_size(&grid, col, row));
            }
        }
    }
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

fn find_basin_size(grid: &Grid, col: usize, row: usize) -> usize {
    fn _find_basin_size(
        grid: &Grid,
        col: usize,
        row: usize,
        seen: &mut HashSet<(usize, usize)>,
    ) -> usize {
        if seen.contains(&(col, row)) {
            return 0;
        }
        seen.insert((col, row));
        let level = grid.get(col, row);
        let mut basin_size = 1;
        for (adj_col, adj_row) in grid.adjacent_points(col, row) {
            let adj_level = grid.get(adj_col, adj_row);
            if adj_level != 9 && adj_level > level {
                basin_size += _find_basin_size(grid, adj_col, adj_row, seen);
            }
        }
        basin_size
    }
    _find_basin_size(grid, col, row, &mut HashSet::new())
}

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

    fn get(&self, col: usize, row: usize) -> u8 {
        self.data[row * self.row_len + col]
    }

    fn adjacent_points(&self, col: usize, row: usize) -> AdjacentIter {
        AdjacentIter {
            col,
            row,
            num_rows: self.num_rows(),
            num_cols: self.num_cols(),
            state: AdjacentIterState::Above,
        }
    }
}

#[derive(Debug)]
struct AdjacentIter {
    col: usize,
    row: usize,
    num_rows: usize,
    num_cols: usize,
    state: AdjacentIterState,
}
impl Iterator for AdjacentIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        match self.state {
            AdjacentIterState::Above => {
                self.state = AdjacentIterState::Left;
                if self.row > 0 {
                    Some((self.col, self.row - 1))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Left => {
                self.state = AdjacentIterState::Below;
                if self.col > 0 {
                    Some((self.col - 1, self.row))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Below => {
                self.state = AdjacentIterState::Right;
                if self.row + 1 < self.num_rows {
                    Some((self.col, self.row + 1))
                } else {
                    self.next()
                }
            }
            AdjacentIterState::Right => {
                self.state = AdjacentIterState::Done;
                if self.col + 1 < self.num_cols {
                    Some((self.col + 1, self.row))
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
