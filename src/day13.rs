use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let (x_or_y, fold_value) = folds
        .split('\n')
        .next()
        .unwrap()
        .strip_prefix("fold along ")
        .unwrap()
        .split_once('=')
        .unwrap();
    dbg!(x_or_y, fold_value);
    let fold_value: usize = fold_value.parse().unwrap();
    let transform = |(x, y)| match x_or_y {
        "x" if x > fold_value => (fold_value - (x - fold_value), y),
        "y" if y > fold_value => (x, fold_value - (y - fold_value)),
        _ => (x, y),
    };
    let points: HashSet<_> = dots
        .split('\n')
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            transform((x, y))
        })
        .collect();
    points.len()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let transform = folds.split('\n').fold(
        Box::new(|(x, y)| (x, y)) as Box<dyn Fn((usize, usize)) -> (usize, usize)>,
        |transform, fold_spec| {
            let (x_or_y, fold_value) = fold_spec
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            let fold_value: usize = fold_value.parse().unwrap();
            Box::new(move |(x, y)| {
                let (x, y) = transform((x, y));
                match x_or_y {
                    "x" if x > fold_value => (fold_value - (x - fold_value), y),
                    "y" if y > fold_value => (x, fold_value - (y - fold_value)),
                    _ => (x, y),
                }
            })
        },
    );
    let points: HashSet<_> = dots
        .split('\n')
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            transform((x, y))
        })
        .collect();
    let max_x = points.iter().copied().map(|(x, _y)| x).max().unwrap();
    let max_y = points.iter().copied().map(|(_x, y)| y).max().unwrap();
    for y in 0..=max_y + 1 {
        for x in 0..=max_x + 1 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    points.len()
}
