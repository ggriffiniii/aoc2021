use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

/// Spin the orientation 90 degrees leaving the z-axis alone.
const fn spin_90((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (y, -x, z)
}
/// Spin the orientation 180 degrees leaving the z-axis alone.
const fn spin_180((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (-x, -y, z)
}
/// Spin the orientation 270 degrees leaving the z-axis alone.
const fn spin_270((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (-y, x, z)
}

// These rotations are in reference to a rubik's cube that is setup where the
// red side is the positive z-axis facing up, the white side is the positive y-axis, and
// the blue side is the positive x-axis.
const fn top_red((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (x, y, z)
}
// Rotate the coordinates so that the green side is facing up.
const fn rot_top_green((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (z, y, -x)
}
// Rotate the coordinates so that the blue side is facing up.
const fn rot_top_blue((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (-z, y, x)
}
// Rotate the coordinates so that the white side is facing up.
const fn rot_top_white((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (x, -z, y)
}
// Rotate the coordinates so that the yellow side is facing up.
const fn rot_top_yellow((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (x, z, -y)
}
// Rotate the coordinates so that the orange side is facing up.
const fn rot_top_orange((x, y, z): (isize, isize, isize)) -> (isize, isize, isize) {
    (x, -y, -z)
}

type TransformFn = fn((isize, isize, isize)) -> (isize, isize, isize);

const ORIENTATIONS: [TransformFn; 24] = [
    top_red,
    |xyz| spin_90(top_red(xyz)),
    |xyz| spin_180(top_red(xyz)),
    |xyz| spin_270(top_red(xyz)),
    rot_top_green,
    |xyz| spin_90(rot_top_green(xyz)),
    |xyz| spin_180(rot_top_green(xyz)),
    |xyz| spin_270(rot_top_green(xyz)),
    rot_top_blue,
    |xyz| spin_90(rot_top_blue(xyz)),
    |xyz| spin_180(rot_top_blue(xyz)),
    |xyz| spin_270(rot_top_blue(xyz)),
    rot_top_white,
    |xyz| spin_90(rot_top_white(xyz)),
    |xyz| spin_180(rot_top_white(xyz)),
    |xyz| spin_270(rot_top_white(xyz)),
    rot_top_yellow,
    |xyz| spin_90(rot_top_yellow(xyz)),
    |xyz| spin_180(rot_top_yellow(xyz)),
    |xyz| spin_270(rot_top_yellow(xyz)),
    rot_top_orange,
    |xyz| spin_90(rot_top_orange(xyz)),
    |xyz| spin_180(rot_top_orange(xyz)),
    |xyz| spin_270(rot_top_orange(xyz)),
];

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut scanners = parse(input);
    let mut found = vec![scanners.remove(0)];

    while !scanners.is_empty() {
        let (scanner_idx, rotate_transform, offsets) = find_overlap(&found, &scanners);
        let mut found_scanner = scanners.remove(scanner_idx);
        let transform = |xyz| {
            let (x, y, z) = rotate_transform(xyz);
            (x + offsets.0, y + offsets.1, z + offsets.2)
        };
        for beacon in found_scanner.iter_mut() {
            *beacon = transform(*beacon);
        }
        found.push(found_scanner);
    }
    let unique_beacons: HashSet<_> = found.into_iter().flat_map(|x| x.into_iter()).collect();
    unique_beacons.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> isize {
    let mut scanners = parse(input);
    let mut found = vec![scanners.remove(0)];
    let mut found_offsets = vec![(0, 0, 0)];

    while !scanners.is_empty() {
        let (scanner_idx, rotate_transform, offsets) = find_overlap(&found, &scanners);
        let mut found_scanner = scanners.remove(scanner_idx);
        let transform = |xyz| {
            let (x, y, z) = rotate_transform(xyz);
            (x + offsets.0, y + offsets.1, z + offsets.2)
        };
        for beacon in found_scanner.iter_mut() {
            *beacon = transform(*beacon);
        }
        found.push(found_scanner);
        found_offsets.push(offsets);
    }
    found_offsets
        .iter()
        .flat_map(|offsets_a| {
            found_offsets
                .iter()
                .map(move |offsets_b| (offsets_a, offsets_b))
        })
        .map(|(offsets_a, offsets_b)| {
            (offsets_a.0 - offsets_b.0).abs()
                + (offsets_a.1 - offsets_b.1).abs()
                + (offsets_a.2 - offsets_b.2).abs()
        })
        .max()
        .unwrap()
}

fn find_overlap(
    already_found: &[Vec<(isize, isize, isize)>],
    looking: &[Vec<(isize, isize, isize)>],
) -> (usize, TransformFn, (isize, isize, isize)) {
    for found in already_found {
        for (idx, look) in looking.iter().enumerate() {
            if let Some((transform, offsets)) = is_overlapping(found, look) {
                return (idx, transform, offsets);
            }
        }
    }
    panic!("no overlap found");
}

fn is_overlapping(
    a: &[(isize, isize, isize)],
    b: &[(isize, isize, isize)],
) -> Option<(TransformFn, (isize, isize, isize))> {
    let mut offset_map = HashMap::new();
    for orientation in ORIENTATIONS.into_iter() {
        offset_map.clear();
        for beacon_b in b.iter().copied().map(orientation) {
            for beacon_a in a.iter().copied() {
                let offsets = (
                    beacon_a.0 - beacon_b.0,
                    beacon_a.1 - beacon_b.1,
                    beacon_a.2 - beacon_b.2,
                );
                *offset_map.entry(offsets).or_insert(0) += 1;
            }
        }
        if let Some(offsets) = offset_map
            .iter()
            .find(|(_, &count)| count >= 12)
            .map(|(&offsets, _)| offsets)
        {
            return Some((orientation, offsets));
        }
    }
    None
}

fn parse(input: &str) -> Vec<Vec<(isize, isize, isize)>> {
    input
        .split("\n\n")
        .map(|scanner_input| {
            scanner_input
                .lines()
                .skip(1)
                .map(|line| {
                    let mut iter = line.split(',').map(|x| x.parse().unwrap());
                    (
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}
