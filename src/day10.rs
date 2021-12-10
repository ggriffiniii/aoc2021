use aoc_runner_derive::aoc;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .split('\n')
        .map(|line| {
            let mut stack = Vec::new();
            for b in line.bytes() {
                match b {
                    b'(' | b'[' | b'{' | b'<' => {
                        stack.push(b);
                    }
                    b')' => {
                        if stack.pop() != Some(b'(') {
                            return 3;
                        }
                    }
                    b']' => {
                        if stack.pop() != Some(b'[') {
                            return 57;
                        }
                    }
                    b'}' => {
                        if stack.pop() != Some(b'{') {
                            return 1197;
                        }
                    }
                    b'>' => {
                        if stack.pop() != Some(b'<') {
                            return 25137;
                        }
                    }
                    _ => panic!("invalid input"),
                }
            }
            0
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .split('\n')
        .filter_map(|line| {
            let mut stack = Vec::new();
            for b in line.bytes() {
                match b {
                    b'(' => stack.push(b')'),
                    b'[' => stack.push(b']'),
                    b'{' => stack.push(b'}'),
                    b'<' => stack.push(b'>'),
                    b')' | b']' | b'}' | b'>' => {
                        if stack.pop() != Some(b) {
                            return None;
                        }
                    }
                    _ => panic!("invalid input"),
                }
            }
            stack.reverse();
            Some(stack.into_iter().fold(0, |total, c| {
                total * 5
                    + match c {
                        b')' => 1,
                        b']' => 2,
                        b'}' => 3,
                        b'>' => 4,
                        _ => panic!("invalid input"),
                    }
            }))
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}
