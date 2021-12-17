use std::{cmp::Ordering, ops::RangeInclusive};

use aoc_runner_derive::aoc;

#[aoc(day17, part1)]
pub fn part1(input: &str) -> isize {
    let (_, target_y) = target_ranges(input);
    // The probe goes up and comes down. when it reaches y=0 again the y
    // velocity will be the negated original velocity. The maximum speed it can
    // have at that point is the difference between y=0 and the lowest point of
    // the target range.
    (target_y.start() + 1) * target_y.start() / 2
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let (target_x, target_y) = target_ranges(input);
    // find bounds for the x and y velocities. Then simulate each one to see if
    // it is ever within the target area.

    // x distance covered for any initial velocity `xv` is xv*(xv+1)/2
    // to find minimum x velocity that will travel distance `d`
    // d = xv*(xv+1)/2
    // 2d = xv*(xv+1)
    // 2d = x^2 + x
    // 0 = x^2 + x - 2d
    // use quadratic equation:
    // x = (-1 + sqrt(1-4*2d)) / 2
    let d = *target_x.start() as f64;
    let x_velo_min = ((-1.0 + (1.0 - 4.0 * 2.0 * d).abs().sqrt()) / 2.0).ceil() as isize;
    // max is self explanatory. It can't ever be greater than the end of the target range.
    let x_velo_max = *target_x.end();
    // min is self explanatory. It can't ever be less than the end of the target range.
    let y_velo_min = *target_y.start();
    // see the description in part1 for maximum y velocity.
    let y_velo_max = target_y.start().abs();

    // For every (x_vel, y_vel) combination simulate a probe and count the
    // number that fall within the target range.
    (x_velo_min..=x_velo_max)
        .flat_map(|x_velo| (y_velo_min..=y_velo_max).map(move |y_velo| (x_velo, y_velo)))
        .filter_map(|(x_velo, y_velo)| {
            probe_iter(x_velo, y_velo)
                .take_while(|(x, y)| x <= target_x.end() && y >= target_y.start())
                .find(|(x, y)| target_x.contains(x) && target_y.contains(y))
        })
        .count()
}

fn target_ranges(input: &str) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    fn str_to_range(input: &str) -> RangeInclusive<isize> {
        let (start, end) = input.split_once("..").unwrap();
        start.parse().unwrap()..=end.parse().unwrap()
    }

    let input = input.strip_prefix("target area: ").unwrap();
    let (x, y) = input.split_once(", ").unwrap();
    let x = x.strip_prefix("x=").unwrap();
    let y = y.strip_prefix("y=").unwrap();
    (str_to_range(x), str_to_range(y))
}

fn probe_iter(x_vel: isize, y_vel: isize) -> ProbeIter {
    ProbeIter {
        pos: (0, 0),
        vel: (x_vel, y_vel),
    }
}

struct ProbeIter {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Iterator for ProbeIter {
    type Item = (isize, isize);
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.pos;
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.vel.0 += match self.vel.0.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };
        self.vel.1 -= 1;
        Some(curr)
    }
}
