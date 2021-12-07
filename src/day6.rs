use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    simulation(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    simulation(input, 256)
}

fn simulation(input: &str, num_days: usize) -> usize {
    let mut fish_states = [0; 9];
    for fish in input.split(',').map(|s| -> usize { s.parse().unwrap() }) {
        fish_states[fish] += 1;
    }
    for _day in 0..num_days {
        fish_states.rotate_left(1);
        let fish_created = fish_states[8];
        fish_states[6] += fish_states[8];
        fish_states[8] = fish_created;
    }
    fish_states.iter().sum()
}
