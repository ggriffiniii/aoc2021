use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

fn is_small_cave(cave: &str) -> bool {
    cave.as_bytes()[0] > b'a'
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    for (start, end) in input.split('\n').map(|line| line.split_once('-').unwrap()) {
        neighbors.entry(start).or_default().push(end);
        neighbors.entry(end).or_default().push(start);
    }
    count_all_paths(&neighbors)
}

fn count_all_paths(neighbors: &HashMap<&str, Vec<&str>>) -> usize {
    let mut small_caves_visited = HashSet::new();
    traverse(neighbors, &mut small_caves_visited, "start")
}

fn traverse<'a, 'b>(
    neighbors: &'a HashMap<&'b str, Vec<&'b str>>,
    small_caves_visited: &'a mut HashSet<&'b str>,
    current: &'b str,
) -> usize {
    if small_caves_visited.contains(current) {
        return 0;
    }
    if current == "end" {
        return 1;
    }
    if is_small_cave(current) {
        small_caves_visited.insert(current);
    }
    let num_paths = neighbors[current]
        .iter()
        .map(|n| traverse(neighbors, small_caves_visited, n))
        .sum();
    small_caves_visited.remove(current);
    num_paths
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    for (start, end) in input.split('\n').map(|line| line.split_once('-').unwrap()) {
        neighbors.entry(start).or_default().push(end);
        neighbors.entry(end).or_default().push(start);
    }
    count_all_paths2(&neighbors)
}

fn count_all_paths2(neighbors: &HashMap<&str, Vec<&str>>) -> usize {
    let mut small_caves_visited = HashMap::new();
    traverse2(neighbors, &mut small_caves_visited, "start")
}

fn traverse2<'a, 'b>(
    neighbors: &'a HashMap<&'b str, Vec<&'b str>>,
    small_caves_visited: &'a mut HashMap<&'b str, usize>,
    current: &'b str,
) -> usize {
    let already_visited_current_cave = *small_caves_visited.entry(current).or_default() > 0;
    let already_visited_small_cave_twice = small_caves_visited.values().copied().max().unwrap() > 1;
    if already_visited_current_cave && (current == "start" || already_visited_small_cave_twice) {
        return 0;
    }
    if current == "end" {
        return 1;
    }
    if is_small_cave(current) {
        *small_caves_visited.entry(current).or_default() += 1;
    }
    let num_paths = neighbors[current]
        .iter()
        .map(|n| traverse2(neighbors, small_caves_visited, n))
        .sum();
    if is_small_cave(current) {
        *small_caves_visited.get_mut(current).unwrap() -= 1;
    }
    num_paths
}
