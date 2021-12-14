use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 10)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 40)
}

fn solve(input: &str, num_steps: usize) -> usize {
    let (start, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules
        .split('\n')
        .map(|line| {
            let (pair, mid) = line.split_once(" -> ").unwrap();
            let pair = pair.as_bytes();
            ([pair[0], pair[1]], mid.as_bytes()[0])
        })
        .collect();

    let last_letter = start.as_bytes()[start.len() - 1];
    let pairs = start
        .as_bytes()
        .windows(2)
        .fold(HashMap::new(), |mut pairs, pair| {
            *pairs.entry([pair[0], pair[1]]).or_insert(0) += 1;
            pairs
        });

    let pairs = (0..num_steps).fold(pairs, |pairs, _step_num| step(&rules, pairs));
    let mut letters = pairs
        .into_iter()
        .fold(HashMap::new(), |mut letters, (pair, occurrences)| {
            *letters.entry(pair[0]).or_insert(0) += occurrences;
            letters
        });
    *letters.entry(last_letter).or_insert(0) += 1;
    let (min, max) = letters
        .values()
        .fold((usize::MAX, 0), |(min, max), &v| (min.min(v), max.max(v)));
    max - min
}

fn step(
    rules: &HashMap<[u8; 2], u8>,
    before_pairs: HashMap<[u8; 2], usize>,
) -> HashMap<[u8; 2], usize> {
    let mut after = HashMap::new();
    for (pair, occurrences) in before_pairs {
        let mid = rules[&pair];
        *after.entry([pair[0], mid]).or_insert(0) += occurrences;
        *after.entry([mid, pair[1]]).or_insert(0) += occurrences;
    }
    after
}
