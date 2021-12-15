use std::cell::Cell;

use crate::grid::{Grid, X, Y};

use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let row_len = input.find('\n').unwrap();
    let data = input
        .bytes()
        .filter(|&n| n != b'\n')
        .map(|b| Cell::new(b - b'0'));
    let grid = Grid::from_iter(data, row_len);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&grid);
    }
    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let row_len = input.find('\n').unwrap();
    let data = input
        .bytes()
        .filter(|&n| n != b'\n')
        .map(|b| Cell::new(b - b'0'));
    let grid = Grid::from_iter(data, row_len);
    for step_num in 1.. {
        step(&grid);
        if grid
            .points_values()
            .filter(|(_, level)| level.get() != 0)
            .count()
            == 0
        {
            return step_num;
        }
    }
    unreachable!();
}

fn step(grid: &Grid<Cell<u8>>) -> usize {
    let mut flashes = 0;
    for ((x, y), level) in grid.points_values() {
        level.set(level.get() + 1);
        if level.get() == 10 {
            flashes += flash(grid, x, y);
        }
    }
    for (_, level) in grid.points_values() {
        if level.get() > 9 {
            level.set(0);
        }
    }
    flashes
}

fn flash(grid: &Grid<Cell<u8>>, x: X, y: Y) -> usize {
    let mut flashes = 1;
    for (adj_col, adj_row) in grid.neighbors_8(x, y) {
        let level = &grid[(adj_col, adj_row)];
        level.set(level.get() + 1);
        if level.get() == 10 {
            flashes += flash(grid, adj_col, adj_row);
        }
    }
    flashes
}
