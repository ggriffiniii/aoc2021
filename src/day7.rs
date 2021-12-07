use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let mut pos: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();
    pos.sort();
    let median = pos[pos.len() / 2];
    pos.iter()
        .map(|&s| (s as isize - median as isize).abs() as usize)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let pos: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mean: usize = pos.iter().sum::<usize>() / pos.iter().count();
    pos.iter()
        .map(|&s| (s as isize - mean as isize).abs() as usize)
        .map(p2_fuel_for_distance)
        .sum()
}

fn p2_fuel_for_distance(d: usize) -> usize {
    (d * (d + 1)) / 2
}
