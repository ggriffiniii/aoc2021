use std::{
    cmp::{max, Ordering},
    collections::HashMap,
    num::ParseIntError,
};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct XY {
    x: u32,
    y: u32,
}
impl std::str::FromStr for XY {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(XY {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}
#[derive(Debug)]
struct Line {
    start: XY,
    end: XY,
}
impl std::str::FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        Ok(Line {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}
impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn points(&self) -> PointIter {
        let x_step = match self.start.x.cmp(&self.end.x) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };
        let y_step = match self.start.y.cmp(&self.end.y) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };
        let steps_remaining = max(
            (self.start.x as i32 - self.end.x as i32).abs() + 1,
            (self.start.y as i32 - self.end.y as i32).abs() + 1,
        ) as usize;
        PointIter {
            steps_remaining,
            x: self.start.x,
            y: self.start.y,
            x_step,
            y_step,
        }
    }
}

struct PointIter {
    steps_remaining: usize,
    x: u32,
    y: u32,
    x_step: i32,
    y_step: i32,
}
impl Iterator for PointIter {
    type Item = XY;
    fn next(&mut self) -> Option<XY> {
        if self.steps_remaining == 0 {
            return None;
        }
        let item = XY {
            x: self.x,
            y: self.y,
        };
        self.x = (self.x as i32 + self.x_step) as u32;
        self.y = (self.y as i32 + self.y_step) as u32;
        self.steps_remaining -= 1;
        Some(item)
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let lines: Vec<Line> = input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .filter(|line: &Line| line.is_horizontal() || line.is_vertical())
        .collect();
    num_overlapping_points(&lines)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let lines: Vec<Line> = input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();
    num_overlapping_points(&lines)
}

fn num_overlapping_points(lines: &[Line]) -> usize {
    let mut lines_per_point = HashMap::new();
    for line in lines {
        for point in line.points() {
            *lines_per_point.entry(point).or_insert(0) += 1;
        }
    }
    lines_per_point
        .values()
        .filter(|&&num_lines| num_lines > 1)
        .count()
}
