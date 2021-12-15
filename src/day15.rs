use crate::grid::{Grid, X, Y};
use std::{cmp::Ordering, collections::BinaryHeap};

use aoc_runner_derive::aoc;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let width = input.find('\n').unwrap();
    let data = input.bytes().filter(|&n| n != b'\n').map(|b| b - b'0');
    let grid = Grid::from_iter(data, width);
    let origin = (X(0), Y(0));
    let finish = (X(grid.width() - 1), Y(grid.height() - 1));
    find_lowest_cost(&grid, origin, finish)
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
    let grid = Grid::new(grid_data, input.find('\n').unwrap() * 5);
    let origin = (X(0), Y(0));
    let finish = (X(grid.width() - 1), Y(grid.height() - 1));
    find_lowest_cost(&grid, origin, finish)
}

fn find_lowest_cost(grid: &Grid<u8>, origin: (X, Y), finish: (X, Y)) -> usize {
    let mut heap = BinaryHeap::new();
    let mut costs = Grid::new(vec![usize::MAX; grid.num_cells()], grid.width());
    costs[origin] = 0;
    heap.push(NodeState {
        cost: 0,
        pos: origin,
    });
    while let Some(NodeState { cost, pos }) = heap.pop() {
        if pos == finish {
            return cost;
        }
        if cost > costs[pos] {
            continue;
        }
        for pos in grid.neighbors_4(pos.0, pos.1) {
            let next = NodeState {
                cost: cost + grid[pos] as usize,
                pos,
            };
            if next.cost < costs[pos] {
                heap.push(next);
                costs[pos] = next.cost;
            }
        }
    }
    panic!("no path exists")
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeState {
    pos: (X, Y),
    cost: usize,
}
impl Ord for NodeState {
    fn cmp(&self, other: &Self) -> Ordering {
        // lowest cost first
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for NodeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
