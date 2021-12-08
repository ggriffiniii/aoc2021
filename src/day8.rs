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
    let mapping = Mapping::new(signals);
    output.split(' ').fold(0, |accum, output| {
        accum * 10 + mapping.decode_output(output) as usize
    })
}

const A_SEGMENT: u8 = 1 << 0;
const B_SEGMENT: u8 = 1 << 1;
const C_SEGMENT: u8 = 1 << 2;
const D_SEGMENT: u8 = 1 << 3;
const E_SEGMENT: u8 = 1 << 4;
const F_SEGMENT: u8 = 1 << 5;
const G_SEGMENT: u8 = 1 << 6;

// Convert the ascii range 'a'..='g' to 0..=6
const fn char_to_idx(char: u8) -> usize {
    (char - 97) as usize
}

#[derive(Debug)]
struct Mapping {
    signal_to_output: [u8; 7],
}
impl Mapping {
    fn new(signals: &str) -> Self {
        let mut signal_to_output = [0; 7];
        let signals: Vec<_> = signals.split(' ').map(|s| s.as_bytes()).collect();
        // find 2 digits, that's '1'. one of the two letters will appear in 8
        // outputs that' s the top half (C_SEGMENT), the other will appear in 9
        // that's the bottom half (F_SEGMENT).
        let one = signals.iter().find(|s| s.len() == 2).unwrap();
        match signals.iter().filter(|s| s.contains(&one[0])).count() {
            8 => {
                signal_to_output[char_to_idx(one[0])] = C_SEGMENT;
                signal_to_output[char_to_idx(one[1])] = F_SEGMENT;
            }
            9 => {
                signal_to_output[char_to_idx(one[0])] = F_SEGMENT;
                signal_to_output[char_to_idx(one[1])] = C_SEGMENT;
            }
            _ => panic!("too many signals contain {:?}", one),
        };
        // find 3 digits, that's '7'. it will contain two of the same letters as
        // '1' above, the third letter is the top (A_SEGMENT).
        let seven = signals.iter().find(|s| s.len() == 3).unwrap();
        let top = *seven.iter().find(|b| !one.contains(b)).unwrap();
        signal_to_output[char_to_idx(top)] = A_SEGMENT;
        // find 4 digits, that's '4'. it will contain two of the same letters as
        // '1' above, of the other 2 letters one is contained in 6 outputs and
        // is the top left (B_SEGMENT); the one contained in 7 outputs is the
        // middle (D_SEGMENT).
        let four = signals.iter().find(|s| s.len() == 4).unwrap();
        let mut four = four.iter().filter(|b| !one.contains(b));
        let first_four = *four.next().unwrap();
        let second_four = *four.next().unwrap();
        match signals.iter().filter(|s| s.contains(&first_four)).count() {
            6 => {
                signal_to_output[char_to_idx(first_four)] = B_SEGMENT;
                signal_to_output[char_to_idx(second_four)] = D_SEGMENT;
            }
            7 => {
                signal_to_output[char_to_idx(first_four)] = D_SEGMENT;
                signal_to_output[char_to_idx(second_four)] = B_SEGMENT;
            }
            _ => panic!("too many signals contain {:?}", first_four),
        };
        // there are two remaining letters. the one contained in 4 outputs is
        // the bottom left (E_SEGMENT); the one contained in 7 outputs is the
        // bottom center (G_SEGMENT)
        let mut rem = b"abcdefg"
            .iter()
            .filter(|b| ![top, one[0], one[1], first_four, second_four].contains(b));
        let first = *rem.next().unwrap();
        let second = *rem.next().unwrap();
        match signals.iter().filter(|s| s.contains(&first)).count() {
            4 => {
                signal_to_output[char_to_idx(first)] = E_SEGMENT;
                signal_to_output[char_to_idx(second)] = G_SEGMENT;
            }
            7 => {
                signal_to_output[char_to_idx(first)] = G_SEGMENT;
                signal_to_output[char_to_idx(second)] = E_SEGMENT;
            }
            _ => panic!("too many signals contain {:?}", first),
        };
        Mapping { signal_to_output }
    }

    // Return the number 0..=9 corresponding to a series of letters contained in
    // the output string provided.
    fn decode_output(&self, output: &str) -> u8 {
        const fn lookup_table() -> [u8; 256] {
            const ZERO_SEGMENTS: u8 =
                A_SEGMENT | B_SEGMENT | C_SEGMENT | E_SEGMENT | F_SEGMENT | G_SEGMENT;
            const ONE_SEGMENTS: u8 = C_SEGMENT | F_SEGMENT;
            const TWO_SEGMENTS: u8 = A_SEGMENT | C_SEGMENT | D_SEGMENT | E_SEGMENT | G_SEGMENT;
            const THREE_SEGMENTS: u8 = A_SEGMENT | C_SEGMENT | D_SEGMENT | F_SEGMENT | G_SEGMENT;
            const FOUR_SEGMENTS: u8 = B_SEGMENT | C_SEGMENT | D_SEGMENT | F_SEGMENT;
            const FIVE_SEGMENTS: u8 = A_SEGMENT | B_SEGMENT | D_SEGMENT | F_SEGMENT | G_SEGMENT;
            const SIX_SEGMENTS: u8 =
                A_SEGMENT | B_SEGMENT | D_SEGMENT | E_SEGMENT | F_SEGMENT | G_SEGMENT;
            const SEVEN_SEGMENTS: u8 = A_SEGMENT | C_SEGMENT | F_SEGMENT;
            const EIGHT_SEGMENTS: u8 =
                A_SEGMENT | B_SEGMENT | C_SEGMENT | D_SEGMENT | E_SEGMENT | F_SEGMENT | G_SEGMENT;
            const NINE_SEGMENTS: u8 =
                A_SEGMENT | B_SEGMENT | C_SEGMENT | D_SEGMENT | F_SEGMENT | G_SEGMENT;
            let mut t = [0; 256];
            t[ZERO_SEGMENTS as usize] = 0;
            t[ONE_SEGMENTS as usize] = 1;
            t[TWO_SEGMENTS as usize] = 2;
            t[THREE_SEGMENTS as usize] = 3;
            t[FOUR_SEGMENTS as usize] = 4;
            t[FIVE_SEGMENTS as usize] = 5;
            t[SIX_SEGMENTS as usize] = 6;
            t[SEVEN_SEGMENTS as usize] = 7;
            t[EIGHT_SEGMENTS as usize] = 8;
            t[NINE_SEGMENTS as usize] = 9;
            t
        }
        static LOOKUP_TABLE: [u8; 256] = lookup_table();
        let bitmask = output.as_bytes().iter().fold(0u8, |accum, &b| {
            accum | self.signal_to_output[char_to_idx(b)]
        });
        LOOKUP_TABLE[bitmask as usize]
    }
}
