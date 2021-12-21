use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, PartialEq)]
enum LitOrDark {
    Lit,
    Dark,
}
impl LitOrDark {
    fn inverse(self) -> Self {
        match self {
            LitOrDark::Lit => LitOrDark::Dark,
            LitOrDark::Dark => LitOrDark::Lit,
        }
    }

    fn is_lit(self) -> bool {
        self == LitOrDark::Lit
    }
}

#[derive(Debug, Default)]
struct EnhancementBits([u64; 8]);
impl EnhancementBits {
    fn get_bit(&self, b: u16) -> LitOrDark {
        assert!(b <= 512);
        let chunk = b as usize / 64;
        let bit = b % 64;
        if self.0[chunk] & (1 << bit) == 0 {
            LitOrDark::Dark
        } else {
            LitOrDark::Lit
        }
    }

    fn set_bit(&mut self, b: u16) {
        assert!(b <= 512);
        let chunk = b as usize / 64;
        let bit = b % 64;
        self.0[chunk] |= 1 << bit;
    }
}

#[derive(Debug)]
struct Image {
    default_pixel: LitOrDark,
    pixels: HashSet<(isize, isize)>,
}

impl Image {
    fn get_pixel(&self, pos: (isize, isize)) -> LitOrDark {
        if self.pixels.contains(&pos) {
            self.default_pixel.inverse()
        } else {
            self.default_pixel
        }
    }

    fn enhance(&self, enhancement: &EnhancementBits) -> Self {
        let default_pixel = match self.default_pixel {
            LitOrDark::Lit => enhancement.get_bit(0x1ff),
            LitOrDark::Dark => enhancement.get_bit(0),
        };
        let mut pixels = HashSet::new();
        let (min_x, max_x, min_y, max_y) = self.pixels.iter().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        );
        for y in min_y - 2..=max_y + 2 {
            for x in min_x - 2..=max_x + 2 {
                let mut enhance_idx = 0;
                enhance_idx |= (self.get_pixel((x - 1, y - 1)).is_lit() as u16) << 8;
                enhance_idx |= (self.get_pixel((x, y - 1)).is_lit() as u16) << 7;
                enhance_idx |= (self.get_pixel((x + 1, y - 1)).is_lit() as u16) << 6;
                enhance_idx |= (self.get_pixel((x - 1, y)).is_lit() as u16) << 5;
                enhance_idx |= (self.get_pixel((x, y)).is_lit() as u16) << 4;
                enhance_idx |= (self.get_pixel((x + 1, y)).is_lit() as u16) << 3;
                enhance_idx |= (self.get_pixel((x - 1, y + 1)).is_lit() as u16) << 2;
                enhance_idx |= (self.get_pixel((x, y + 1)).is_lit() as u16) << 1;
                enhance_idx |= self.get_pixel((x + 1, y + 1)).is_lit() as u16;
                if enhancement.get_bit(enhance_idx) != default_pixel {
                    pixels.insert((x, y));
                }
            }
        }
        Image {
            default_pixel,
            pixels,
        }
    }
}

fn solve(input: &str, num_iters: usize) -> usize {
    let (enhancement_input, image_input) = input.split_once("\n\n").unwrap();
    assert_eq!(enhancement_input.len(), 512);
    let mut enhancement = EnhancementBits::default();
    for i in enhancement_input
        .bytes()
        .enumerate()
        .filter(|&(_, b)| b == b'#')
        .map(|(i, _)| i)
    {
        enhancement.set_bit(i as u16);
    }
    let mut pixels = HashSet::new();
    for (y, line) in image_input.split('\n').enumerate() {
        for (x, is_lit) in line.bytes().enumerate().map(|(i, b)| (i, b == b'#')) {
            if is_lit {
                pixels.insert((x as isize, y as isize));
            }
        }
    }
    let image = Image {
        default_pixel: LitOrDark::Dark,
        pixels,
    };
    let enhanced = (0..num_iters).fold(image, |image, _| image.enhance(&enhancement));
    enhanced.pixels.len()
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 50)
}
