use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let (start, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules
        .split('\n')
        .map(|line| line.split_once(" -> ").unwrap())
        .collect();
    let mut polymer = start.to_string();
    for _ in 0..10 {
        polymer = step(&rules, &polymer);
    }
    let occurrences =
        polymer
            .into_bytes()
            .into_iter()
            .fold(HashMap::new(), |mut occurrences, c| {
                *occurrences.entry(c).or_insert(0) += 1;
                occurrences
            });
    let max = occurrences.values().max().unwrap();
    let min = occurrences.values().min().unwrap();
    max - min
}

fn step(rules: &HashMap<&str, &str>, start: &str) -> String {
    let mut polymer = String::new();
    for i in 0..start.len() - 1 {
        polymer.push_str(&start[i..=i]);
        polymer.push_str(rules[&start[i..i + 2]]);
    }
    polymer.push_str(&start[start.len() - 1..start.len()]);
    polymer
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let (start, rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules
        .split('\n')
        .map(|line| line.split_once(" -> ").unwrap())
        .collect();
    
    let mut pairs = (0..start.len() - 1).fold(HashMap::new(), |pairs, i| {
        *pairs.entry(&start[i..i+2]).or_insert(0) += 1;
        pairs
    });
    for _ in 0..40 {
        pairs = step2(&pairs);
    }
    pairs.len()
}

fn step2(rules: &HashMap<&str, &str>, before: HashMap<String, usize>) -> HashMap<String, usize> {
    let mut after = HashMap::new();
    for (pair, occurrences) in before.into_iter() {
        let new_mid = rules[pair.as_str()];
        let mut new_first = pair[0..1].to_owned();
        new_first.push_str(new_mid);
        let mut new_second = new_mid.to_owned();
        new_second.push_str(&pair[1..2]);
        *after.entry(new_first).or_insert(0) += occurrences;
        *after.entry(new_second).or_insert(0) += occurrences;
    }
    after
}