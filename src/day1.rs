use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let entries: Vec<u64> = input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();
    entries.windows(2).fold(0, |total, items| {
        let prev = items[0];
        let curr = items[1];
        total + ((curr > prev) as u64)
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let entries: Vec<u64> = input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();
    let entries: Vec<u64> = entries
        .windows(3)
        .map(|entries| entries.iter().copied().sum())
        .collect();
    entries.windows(2).fold(0, |total, items| {
        let prev = items[0];
        let curr = items[1];
        total + ((curr > prev) as u64)
    })
}
