use crate::grid::{Grid, X, Y};
use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let width = input.find('\n').unwrap();
    let grid = Grid::from_iter(
        input.bytes().filter(|&b| b != b'\n').map(|b| b - b'0'),
        width,
    );
    grid.points_values()
        .filter_map(|((x, y), &level)| {
            let min_adjacent = grid
                .neighbors_4(x, y)
                .map(|(x, y)| grid[(x, y)])
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
    let width = input.find('\n').unwrap();
    let grid = Grid::from_iter(
        input.bytes().filter(|&b| b != b'\n').map(|b| b - b'0'),
        width,
    );
    let mut basin_sizes: Vec<_> = grid
        .points_values()
        .filter_map(|((x, y), &level)| {
            let min_adjacent = grid
                .neighbors_4(x, y)
                .map(|(x, y)| grid[(x, y)])
                .min()
                .unwrap();
            if level < min_adjacent {
                Some(find_basin_size(&grid, x, y))
            } else {
                None
            }
        })
        .collect();
    basin_sizes.sort_unstable_by_key(|k| std::cmp::Reverse(*k));
    basin_sizes.iter().take(3).product()
}

fn find_basin_size(grid: &Grid<u8>, x: X, y: Y) -> usize {
    fn _find_basin_size(grid: &Grid<u8>, x: X, y: Y, seen: &mut HashSet<(X, Y)>) -> usize {
        if seen.contains(&(x, y)) {
            return 0;
        }
        seen.insert((x, y));
        let level = grid[(x, y)];
        let mut basin_size = 1;
        for (adj_x, adj_y) in grid.neighbors_4(x, y) {
            let adj_level = grid[(adj_x, adj_y)];
            if adj_level != 9 && adj_level > level {
                basin_size += _find_basin_size(grid, adj_x, adj_y, seen);
            }
        }
        basin_size
    }
    _find_basin_size(grid, x, y, &mut HashSet::new())
}
