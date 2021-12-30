use aoc_runner_derive::aoc;

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let input = input
        .split('\n')
        .map(RebootStep::from_input)
        .take_while(|step| {
            let cuboid = &step.cuboid;
            cuboid.min().x >= -50
                && cuboid.min().y >= -50
                && cuboid.min().z >= -50
                && cuboid.max().x <= 51
                && cuboid.max().y <= 51
                && cuboid.max().z <= 51
        });
    solve(input)
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.split('\n').map(RebootStep::from_input);
    solve(input)
}

#[derive(Debug, Clone, Copy)]
struct V3 {
    x: isize,
    y: isize,
    z: isize,
}
impl V3 {
    fn min(lhs: &Self, rhs: &Self) -> Self {
        V3 {
            x: lhs.x.min(rhs.x),
            y: lhs.y.min(rhs.y),
            z: lhs.z.min(rhs.z),
        }
    }
    fn max(lhs: &Self, rhs: &Self) -> Self {
        V3 {
            x: lhs.x.max(rhs.x),
            y: lhs.y.max(rhs.y),
            z: lhs.z.max(rhs.z),
        }
    }
}

#[derive(Debug, Clone)]
struct RebootStep {
    cuboid: Cuboid,
    on_off: OnOff,
}
impl RebootStep {
    fn from_input(input: &str) -> Self {
        use std::str::FromStr;
        let (on_off, rest) = input.split_once(' ').unwrap();
        let on_off = match on_off {
            "on" => OnOff::On,
            "off" => OnOff::Off,
            _ => panic!("invalid on/off step"),
        };
        let mut ranges = rest.split(',').map(|r| {
            let (_axis, range) = r.split_once('=').unwrap();
            let (min, max) = range.split_once("..").unwrap();
            (
                isize::from_str(min).unwrap(),
                isize::from_str(max).unwrap() + 1,
            )
        });
        let x_range = ranges.next().unwrap();
        let y_range = ranges.next().unwrap();
        let z_range = ranges.next().unwrap();
        let min = V3 {
            x: x_range.0,
            y: y_range.0,
            z: z_range.0,
        };
        let max = V3 {
            x: x_range.1,
            y: y_range.1,
            z: z_range.1,
        };
        let cuboid = Cuboid::new(min, max);
        Self { cuboid, on_off }
    }
}

#[derive(Debug, Clone)]
struct Cuboid {
    min: V3,
    max: V3,
}

impl Cuboid {
    fn new(min: V3, max: V3) -> Self {
        assert!(min.x < max.x && min.y < max.y && min.z < max.z);
        Cuboid { min, max }
    }

    fn min(&self) -> V3 {
        self.min
    }
    fn max(&self) -> V3 {
        self.max
    }
    fn volume(&self) -> usize {
        let volume = (self.max.x - self.min.x).abs()
            * (self.max.y - self.min.y).abs()
            * (self.max.z - self.min.z).abs();
        volume as usize
    }
    fn intersection(&self, rhs: &Self) -> Option<Cuboid> {
        let min = V3::max(&self.min, &rhs.min);
        let max = V3::min(&self.max, &rhs.max);
        if min.x < max.x && min.y < max.y && min.z < max.z {
            Some(Cuboid::new(min, max))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OnOff {
    On,
    Off,
}
impl OnOff {
    fn invert(self) -> Self {
        match self {
            OnOff::On => OnOff::Off,
            OnOff::Off => OnOff::On,
        }
    }
    fn is_on(self) -> bool {
        matches!(self, OnOff::On)
    }
}

#[derive(Debug, Default)]
struct Reactor {
    cuboids: Vec<(Cuboid, OnOff)>,
}
impl Reactor {
    fn follow_reboot_step(&mut self, step: RebootStep) {
        for idx in 0..self.cuboids.len() {
            let (curr_cuboid, curr_onoff) = &self.cuboids[idx];
            if let Some(overlap) = step.cuboid.intersection(curr_cuboid) {
                let inverted = curr_onoff.invert();
                self.cuboids.push((overlap, inverted))
            }
        }
        if step.on_off.is_on() {
            self.cuboids.push((step.cuboid, step.on_off))
        }
    }

    fn num_cubes_on(&self) -> usize {
        let volume: isize = self
            .cuboids
            .iter()
            .map(|(cuboid, on_off)| match on_off {
                OnOff::On => cuboid.volume() as isize,
                OnOff::Off => -(cuboid.volume() as isize),
            })
            .sum();
        volume as usize
    }
}

fn solve(reboot_steps: impl IntoIterator<Item = RebootStep>) -> usize {
    let mut reactor = Reactor::default();
    for step in reboot_steps {
        reactor.follow_reboot_step(step);
    }
    reactor.num_cubes_on()
}
