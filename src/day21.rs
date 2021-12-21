use std::{collections::HashMap, iter::Cycle, ops::RangeInclusive};

use aoc_runner_derive::aoc;

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    let mut players = parse(input);
    for (num_rolls, roll) in dice_iter().enumerate() {
        let player = &mut players[roll % 2];
        player.advance(roll);
        player.score += player.pos;
        if player.score >= 1000 {
            return &players[(roll + 1) % 2].score * ((num_rolls + 1) * 3);
        }
    }
    unreachable!();
}

fn parse(input: &str) -> [Player; 2] {
    let mut starting_positions = input.split('\n').map(|line| {
        let (_, start_pos) = line.split_once(": ").unwrap();
        start_pos.parse().unwrap()
    });
    [
        Player {
            score: 0,
            pos: starting_positions.next().unwrap(),
        },
        Player {
            score: 0,
            pos: starting_positions.next().unwrap(),
        },
    ]
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    score: usize,
    pos: usize,
}
impl Player {
    fn advance(&mut self, steps: usize) {
        self.pos = ((self.pos - 1) + steps) % 10 + 1;
    }
}

fn dice_iter() -> impl Iterator<Item = usize> {
    struct DiceIter(Cycle<RangeInclusive<usize>>);
    impl Iterator for DiceIter {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            Some(self.0.next().unwrap() + self.0.next().unwrap() + self.0.next().unwrap())
        }
    }
    DiceIter((1..=100).cycle())
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    let players = parse(input);
    play_quantum(&mut HashMap::new(), players)
        .into_iter()
        .max()
        .unwrap()
}

fn play_quantum(cache: &mut HashMap<[Player; 2], [usize; 2]>, players: [Player; 2]) -> [usize; 2] {
    if let Some(cached) = cache.get(&players) {
        return *cached;
    }
    if players[0].score >= 21 {
        return [1, 0];
    }
    if players[1].score >= 21 {
        return [0, 1];
    }
    let mut wins = [0, 0];
    for die1 in 1..=3 {
        for die2 in 1..=3 {
            for die3 in 1..=3 {
                let sum = die1 + die2 + die3;
                let mut players = players.clone();
                let player = &mut players[0];
                player.advance(sum);
                player.score += player.pos;
                // play from this point forward letting the other player go
                // next. Note the inverted order of players.
                let [p2_wins, p1_wins] = play_quantum(cache, [players[1], players[0]]);
                wins[0] += p1_wins;
                wins[1] += p2_wins;
            }
        }
    }
    cache.insert(players, wins);
    wins
}
