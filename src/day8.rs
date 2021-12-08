use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split('\n')
        .flat_map(|line| {
            let (_, output) = line.split_once(" | ").unwrap();
            output.split(' ')
        })
        .filter(|s| [2usize, 3, 4, 7].contains(&s.len()))
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    input.split('\n').map(decode_entry).sum()
}

fn decode_entry(input: &str) -> usize {
    let (signals, output) = input.split_once(" | ").unwrap();
    let signals: Vec<_> = signals.split(' ').map(|s| s.as_bytes()).collect();
    // find 2 digits, that's '1'. one of the two letters will appear in 8
    // outputs that' s the top half, the other will appear in 9 that's the
    // bottom half.
    let one = signals.iter().find(|s| s.len() == 2).unwrap();
    let (top_right, bottom_right) = match signals.iter().filter(|s| s.contains(&one[0])).count() {
        8 => (one[0], one[1]),
        9 => (one[1], one[0]),
        _ => panic!("too many signals contain {:?}", one),
    };
    // find 3 digits, that's 7. it will contain two of the same letters as '1'
    // above, the third letter is the top segment.
    let three = signals.iter().find(|s| s.len() == 3).unwrap();
    let top = *three.iter().find(|b| !one.contains(b)).unwrap();
    // find 4 digits, that's 4. it will contain two of the same letters as '1'
    // above, the other 2 letters should be found in 6 outputs that's the top
    // left, and 7 outputs that's the middle.
    let four = signals.iter().find(|s| s.len() == 4).unwrap();
    let mut four = four.iter().filter(|b| !one.contains(b));
    let first_four = *four.next().unwrap();
    let second_four = *four.next().unwrap();
    let (top_left, middle) = match signals.iter().filter(|s| s.contains(&first_four)).count() {
        6 => (first_four, second_four),
        7 => (second_four, first_four),
        _ => panic!("too many signals contain {:?}", first_four),
    };
    // there are two remaining letters. they will be found in 4 outputs that's
    // the bottom left and 7 outputs that's the bottom center
    let mut rem = b"abcdefg"
        .iter()
        .filter(|b| ![top, top_left, top_right, middle, bottom_right].contains(b));
    let first = *rem.next().unwrap();
    let second = *rem.next().unwrap();
    let (bottom_left, bottom) = match signals.iter().filter(|s| s.contains(&first)).count() {
        4 => (first, second),
        7 => (second, first),
        _ => panic!("too many signals contain {:?}", first),
    };

    let decode_output = |output: &str| -> usize {
        let bitmask = output.as_bytes().iter().fold(0u8, |accum, &b| {
            let bit = if b == top {
                0
            } else if b == top_left {
                1
            } else if b == top_right {
                2
            } else if b == middle {
                3
            } else if b == bottom_left {
                4
            } else if b == bottom_right {
                5
            } else if b == bottom {
                6
            } else {
                panic!("invalid output char");
            };
            accum | 1 << bit
        });

        match bitmask {
            0b0111_0111 => 0,
            0b0010_0100 => 1,
            0b0101_1101 => 2,
            0b0110_1101 => 3,
            0b0010_1110 => 4,
            0b0110_1011 => 5,
            0b0111_1011 => 6,
            0b0010_0101 => 7,
            0b0111_1111 => 8,
            0b0110_1111 => 9,
            x => panic!("invalid bitmask {:0b}", x),
        }
    };
    output
        .split(' ')
        .fold(0, |accum, output| accum * 10 + decode_output(output))
}
