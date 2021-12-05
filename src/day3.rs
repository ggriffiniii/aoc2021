use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let mut cols = Vec::new();
    let mut total_lines = 0;
    for line in input.split('\n') {
        total_lines += 1;
        cols.resize(line.bytes().len(), 0);
        for (col_idx, col_val) in line.bytes().enumerate() {
            match col_val {
                b'0' => {}
                b'1' => cols[col_idx] += 1,
                _ => panic!("bad input"),
            }
        }
    }
    let mut epsilon = 0;
    let mut gamma = 0;
    for col in cols.into_iter() {
        gamma <<= 1;
        epsilon <<= 1;
        if col > (total_lines / 2) {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let input: Vec<_> = input.split('\n').collect();
    let oxy = oxy_rating(input.clone());
    let co2 = co2_rating(input);
    oxy * co2
}

fn oxy_rating(mut input: Vec<&str>) -> u64 {
    let num_cols = input[0].len();
    for col in 0..num_cols {
        let num_rows = input.len();
        let ones = count_ones_in_col(&input, col);
        if ones * 2 >= num_rows {
            input = input
                .into_iter()
                .filter(|&line| line.as_bytes()[col] == b'1')
                .collect();
        } else {
            input = input
                .into_iter()
                .filter(|&line| line.as_bytes()[col] == b'0')
                .collect();
        }
        if input.len() == 1 {
            let mut rating = 0;
            for digit in input[0].bytes() {
                rating <<= 1;
                if digit == b'1' {
                    rating |= 1;
                }
            }
            return rating;
        }
    }
    panic!("unable to find oxy rating");
}

fn co2_rating(mut input: Vec<&str>) -> u64 {
    let num_cols = input[0].len();
    for col in 0..num_cols {
        let num_rows = input.len();
        let ones = count_ones_in_col(&input, col);
        if ones * 2 < num_rows {
            input = input
                .into_iter()
                .filter(|&line| line.as_bytes()[col] == b'1')
                .collect();
        } else {
            input = input
                .into_iter()
                .filter(|&line| line.as_bytes()[col] == b'0')
                .collect();
        }
        if input.len() == 1 {
            let mut rating = 0;
            for digit in input[0].bytes() {
                rating <<= 1;
                if digit == b'1' {
                    rating |= 1;
                }
            }
            return rating;
        }
    }
    panic!("unable to find co2 rating");
}

fn count_ones_in_col(input: &[&str], col: usize) -> usize {
    let mut count = 0;
    for line in input {
        count += (line.as_bytes()[col] == b'1') as usize;
    }
    count
}
