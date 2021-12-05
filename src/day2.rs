use aoc_runner_derive::aoc;

struct XY {
    x: u64,
    y: u64,
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    let pos = input
        .split('\n')
        .map(|line| {
            let (dir, mag) = line.split_once(' ').unwrap();
            let mag: u64 = mag.parse().unwrap();
            (dir, mag)
        })
        .fold(XY { x: 0, y: 0 }, |curr, (dir, mag)| match dir {
            "forward" => XY {
                x: curr.x + mag,
                ..curr
            },
            "up" => XY {
                y: curr.y - mag,
                ..curr
            },
            "down" => XY {
                y: curr.y + mag,
                ..curr
            },
            _ => panic!("bad input"),
        });
    pos.x * pos.y
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    let mut aim = 0;
    let mut pos = XY { x: 0, y: 0 };
    for (dir, mag) in input.split('\n').map(|line| {
        let (dir, mag) = line.split_once(' ').unwrap();
        let mag: u64 = mag.parse().unwrap();
        (dir, mag)
    }) {
        match dir {
            "forward" => {
                pos.x += mag;
                pos.y += aim * mag;
            }
            "up" => aim -= mag,
            "down" => aim += mag,
            _ => panic!("bad input"),
        }
    }
    pos.x * pos.y
}
