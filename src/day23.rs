use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fmt,
    iter::successors,
    ops::Range,
};

use aoc_runner_derive::aoc;

// Example Cave:
// #  #  #  #  #  #  #  #  #  #  #  #  #
// #  .  .  .  .  .  .  .  .  .  .  .  #
// #  #  #  B  #  C  #  B  #  D  #  #  #
//       #  D  #  C  #  B  #  A  #
//       #  D  #  B  #  A  #  C  #
//       #  A  #  D  #  C  #  A  #
//       #  #  #  #  #  #  #  #  #
//
// In code the tiles will be indexed as
//
// #  #  #  #  #  #  #  #  #  #  #  #  #
// #  0  1  .  2  .  3  .  4  .  5  6  #
// #  #  #  8  #  9  #  10 #  11 #  #  #
//       #  12 #  13 #  14 #  15 #
//       #  16 #  17 #  18 #  19 #
//       #  20 #  21 #  22 #  23 #
//       #  #  #  #  #  #  #  #  #
//
// Room numbers's
//       #  0  #  1  #  2  #  3  #

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Amphipod {
    Amber = 1,
    Bronze = 2,
    Copper = 3,
    Desert = 4,
}
const EMPTY: u8 = 0;
const DONE: Cave = Cave::empty()
    .with_tile(8, Some(Amphipod::Amber))
    .with_tile(9, Some(Amphipod::Bronze))
    .with_tile(10, Some(Amphipod::Copper))
    .with_tile(11, Some(Amphipod::Desert))
    .with_tile(12, Some(Amphipod::Amber))
    .with_tile(13, Some(Amphipod::Bronze))
    .with_tile(14, Some(Amphipod::Copper))
    .with_tile(15, Some(Amphipod::Desert))
    .with_tile(16, Some(Amphipod::Amber))
    .with_tile(17, Some(Amphipod::Bronze))
    .with_tile(18, Some(Amphipod::Copper))
    .with_tile(19, Some(Amphipod::Desert))
    .with_tile(20, Some(Amphipod::Amber))
    .with_tile(21, Some(Amphipod::Bronze))
    .with_tile(22, Some(Amphipod::Copper))
    .with_tile(23, Some(Amphipod::Desert));

impl Amphipod {
    const fn from_u8(x: u8) -> Self {
        match x {
            1 => Amphipod::Amber,
            2 => Amphipod::Bronze,
            3 => Amphipod::Copper,
            4 => Amphipod::Desert,
            _ => panic!("invalid Amphipod repr"),
        }
    }

    fn try_from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Amphipod::Amber),
            'B' => Some(Amphipod::Bronze),
            'C' => Some(Amphipod::Copper),
            'D' => Some(Amphipod::Desert),
            _ => None,
        }
    }

    fn as_char(self) -> char {
        match self {
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D',
        }
    }

    fn energy(self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn dest_room(self) -> usize {
        match self {
            Amphipod::Amber => 20,
            Amphipod::Bronze => 21,
            Amphipod::Copper => 22,
            Amphipod::Desert => 23,
        }
    }
}

fn x_coord(tile_idx: usize) -> usize {
    if tile_idx < 7 {
        tile_idx
            + (tile_idx > 1) as usize
            + (tile_idx > 2) as usize
            + (tile_idx > 3) as usize
            + (tile_idx > 4) as usize
    } else {
        tile_idx % 4 * 2 + 2
    }
}

fn y_coord(tile_idx: usize) -> usize {
    (tile_idx > 6) as usize
        + (tile_idx > 11) as usize
        + (tile_idx > 15) as usize
        + (tile_idx > 19) as usize
}

fn distance(from_idx: usize, to_idx: usize) -> usize {
    let x_dist = (x_coord(from_idx) as isize - x_coord(to_idx) as isize).abs() as usize;
    let y_dist = y_coord(from_idx) + y_coord(to_idx);
    x_dist + y_dist
}
// The range of tiles that need to be unoccupied in order to travel from
// from_idx to to_idx. The resulting range includes to_idx if to_idx is a
// hallway tile.
fn hallway_range(mut from_idx: usize, mut to_idx: usize) -> Range<usize> {
    match (from_idx > 7, to_idx > 7) {
        (false, false) => panic!("needs to be to or from a room"),
        (true, false) => {
            // from_idx in a room
            // convert to the hallway position just outside left
            from_idx = from_idx % 4 + 1;
            match from_idx.cmp(&to_idx) {
                Ordering::Equal => from_idx..to_idx + 1,
                Ordering::Less => from_idx + 1..to_idx + 1,
                Ordering::Greater => to_idx..from_idx + 1,
            }
        }
        (false, true) => {
            // to_idx > 7 (in a room)
            // convert to the hallway position just outside left
            to_idx = to_idx % 4 + 1;
            match from_idx.cmp(&to_idx) {
                Ordering::Equal => 0..0, // empty range
                Ordering::Less => from_idx + 1..to_idx + 1,
                Ordering::Greater => to_idx + 1..from_idx,
            }
        }
        (true, true) => {
            from_idx = from_idx % 4 + 1;
            to_idx = to_idx % 4 + 1;
            match from_idx.cmp(&to_idx) {
                Ordering::Equal => 0..0, // empty range
                Ordering::Less => from_idx + 1..to_idx + 1,
                Ordering::Greater => to_idx + 1..from_idx + 1,
            }
        }
    }
}

#[test]
fn test_hallway_range() {
    assert_eq!(hallway_range(9, 15), 3..5);
    assert_eq!(hallway_range(10, 12), 2..4);
    assert!(hallway_range(9, 13).is_empty());

    assert_eq!(hallway_range(9, 1), 1..3);
    assert_eq!(hallway_range(9, 5), 3..6);
    assert_eq!(hallway_range(9, 3), 3..4);
    assert_eq!(hallway_range(9, 2), 2..3);

    assert_eq!(hallway_range(1, 9), 2..3);
    assert_eq!(hallway_range(5, 9), 3..5);
    assert!(hallway_range(3, 9).is_empty());
    assert!(hallway_range(2, 9).is_empty());
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Cave(u128);
impl Cave {
    const fn empty() -> Self {
        Cave(0)
    }

    fn part1_from_input(input: &str) -> Self {
        assert_eq!(input.len(), 65);
        let mut cave = Cave::empty()
            .with_tile(16, Some(Amphipod::Amber))
            .with_tile(20, Some(Amphipod::Amber))
            .with_tile(17, Some(Amphipod::Bronze))
            .with_tile(21, Some(Amphipod::Bronze))
            .with_tile(18, Some(Amphipod::Copper))
            .with_tile(22, Some(Amphipod::Copper))
            .with_tile(19, Some(Amphipod::Desert))
            .with_tile(23, Some(Amphipod::Desert));
        for (tile_idx, byte_offset) in [15, 16, 18, 20, 22, 24, 25, 31, 33, 35, 37, 45, 47, 49, 51]
            .into_iter()
            .enumerate()
            .map(|(idx, byte_offset)| (idx + ((idx > 6) as usize), byte_offset))
        {
            let ch = input.as_bytes()[byte_offset].into();
            cave = cave.with_tile(tile_idx as usize, Amphipod::try_from_char(ch));
        }
        cave
    }

    fn part2_from_input(input: &str) -> Self {
        assert_eq!(input.len(), 65);
        let mut cave = Cave::empty()
            .with_tile(12, Some(Amphipod::Desert))
            .with_tile(13, Some(Amphipod::Copper))
            .with_tile(14, Some(Amphipod::Bronze))
            .with_tile(15, Some(Amphipod::Amber))
            .with_tile(16, Some(Amphipod::Desert))
            .with_tile(17, Some(Amphipod::Bronze))
            .with_tile(18, Some(Amphipod::Amber))
            .with_tile(19, Some(Amphipod::Copper));
        for (tile_idx, byte_offset) in [15, 16, 18, 20, 22, 24, 25, 31, 33, 35, 37, 45, 47, 49, 51]
            .into_iter()
            .enumerate()
            .map(|(idx, byte_offset)| {
                (
                    idx + ((idx > 6) as usize + ((idx > 10) as usize) * 8),
                    byte_offset,
                )
            })
        {
            let ch = input.as_bytes()[byte_offset].into();
            cave = cave.with_tile(tile_idx as usize, Amphipod::try_from_char(ch));
        }
        cave
    }

    const fn get_tile(self, idx: usize) -> Option<Amphipod> {
        assert!(idx <= 23);
        let raw_tile = (self.0 >> (idx * 3) & 0b111) as u8;
        if raw_tile == EMPTY {
            None
        } else {
            Some(Amphipod::from_u8(raw_tile))
        }
    }

    fn move_tile(self, from_idx: usize, to_idx: usize) -> (Self, usize) {
        assert!(self.get_tile(to_idx).is_none());
        let amphipod = self
            .get_tile(from_idx)
            .expect("no amphipod in tile being moved");
        (
            self.with_tile(to_idx, self.get_tile(from_idx))
                .with_tile(from_idx, None),
            distance(from_idx, to_idx) * amphipod.energy(),
        )
    }

    const fn with_tile(self, idx: usize, contents: Option<Amphipod>) -> Self {
        let raw_tile = match contents {
            None => EMPTY as u128,
            Some(amphipod) => amphipod as u128,
        };
        let raw_tile = raw_tile << (idx * 3);
        let clear_tile = self.0 & !(0b111 << (idx * 3));
        Cave(clear_tile | raw_tile)
    }

    fn hallway_range_clear(self, hallway_range: Range<usize>) -> bool {
        let start = hallway_range.start as u32;
        let end = hallway_range.end as u32;
        assert!(start < 7 && end < 8);
        let mask = (8u128.pow(end) - 1) - (8u128.pow(start) - 1);
        self.0 & mask == 0
    }
}

fn in_hallway(tile_idx: usize) -> bool {
    tile_idx < 7
}

fn all_possible_moves(cave: Cave) -> Vec<(Cave, usize)> {
    let mut moves = Vec::new();
    for (tile_idx, amphipod) in (0..7)
        .chain(8..24)
        .filter_map(|tile_idx| Some((tile_idx, cave.get_tile(tile_idx)?)))
    {
        if in_hallway(tile_idx) {
            let mut dest_room_tiles =
                successors(Some(amphipod.dest_room()), |&tile_idx| Some(tile_idx - 4))
                    .take_while(|&tile_idx| tile_idx > 7);
            if dest_room_tiles
                .clone()
                .filter_map(|tile_idx| cave.get_tile(tile_idx))
                .any(|amphipod_in_room| amphipod_in_room != amphipod)
            {
                // destination room contains wrong type of amphipod. Don't move into the room and block it.
                continue;
            }
            let dest_tile_idx = dest_room_tiles
                .find(|&tile_idx| cave.get_tile(tile_idx).is_none())
                .unwrap();
            if cave.hallway_range_clear(hallway_range(tile_idx, dest_tile_idx)) {
                moves.push(cave.move_tile(tile_idx, dest_tile_idx));
            }
        } else {
            // in a room
            let mut room_tiles_above =
                successors(Some(tile_idx - 4), |&tile_idx| Some(tile_idx - 4))
                    .take_while(|&tile_idx| tile_idx > 7);
            let mut room_tiles_below =
                successors(Some(tile_idx + 4), |&tile_idx| Some(tile_idx + 4))
                    .take_while(|&tile_idx| tile_idx < 24);
            if tile_idx % 4 == amphipod.dest_room() % 4 {
                // in the destination room.
                if room_tiles_below.all(|tile_idx| cave.get_tile(tile_idx) == Some(amphipod)) {
                    // all amphipods below are in the correct destination room.
                    continue;
                }
            }
            if room_tiles_above.any(|tile_idx| cave.get_tile(tile_idx).is_some()) {
                // blocked from leaving the room.
                continue;
            }
            // Add moves for all valid hallway positions.
            // Find the hallway tile to the left of the door.
            let mut hallway_tile_idx = tile_idx % 4 + 1;
            for low in (0..=hallway_tile_idx).rev() {
                if cave.hallway_range_clear(low..hallway_tile_idx + 1) {
                    moves.push(cave.move_tile(tile_idx, low));
                } else {
                    break;
                }
            }
            // Move to the hallway tile to the right of the door.
            hallway_tile_idx += 1;
            for hi in hallway_tile_idx..=6 {
                if cave.hallway_range_clear(hallway_tile_idx..hi + 1) {
                    moves.push(cave.move_tile(tile_idx, hi));
                } else {
                    break;
                }
            }
        }
    }
    moves
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn as_char(x: Option<Amphipod>) -> char {
            match x {
                None => '.',
                Some(amphipod) => amphipod.as_char(),
            }
        }
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            as_char(self.get_tile(0)),
            as_char(self.get_tile(1)),
            as_char(self.get_tile(2)),
            as_char(self.get_tile(3)),
            as_char(self.get_tile(4)),
            as_char(self.get_tile(5)),
            as_char(self.get_tile(6)),
        )?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            as_char(self.get_tile(8)),
            as_char(self.get_tile(9)),
            as_char(self.get_tile(10)),
            as_char(self.get_tile(11)),
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#  ",
            as_char(self.get_tile(12)),
            as_char(self.get_tile(13)),
            as_char(self.get_tile(14)),
            as_char(self.get_tile(15)),
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#  ",
            as_char(self.get_tile(16)),
            as_char(self.get_tile(17)),
            as_char(self.get_tile(18)),
            as_char(self.get_tile(19)),
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#  ",
            as_char(self.get_tile(20)),
            as_char(self.get_tile(21)),
            as_char(self.get_tile(22)),
            as_char(self.get_tile(23)),
        )?;
        writeln!(f, "  #########  ")
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    let cave = Cave::part1_from_input(input);
    solve(cave)
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    let cave = Cave::part2_from_input(input);
    dbg!(cave);
    solve(cave)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    cave: Cave,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cave.cmp(&other.cave))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(cave: Cave) -> usize {
    let mut candidates = BinaryHeap::new();
    candidates.push(State { cost: 0, cave });
    let mut costs = HashMap::new();
    costs.insert(cave, 0);

    while let Some(State { cost, cave }) = candidates.pop() {
        if cave == DONE {
            return cost;
        }
        if cost > *costs.get(&cave).unwrap_or(&usize::MAX) {
            continue;
        }

        for (move_cave, move_cost) in all_possible_moves(cave) {
            let next = State {
                cave: move_cave,
                cost: cost + move_cost,
            };
            match costs.entry(next.cave) {
                Entry::Occupied(mut occupied) => {
                    if next.cost < *occupied.get() {
                        occupied.insert(next.cost);
                        candidates.push(next);
                    }
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(next.cost);
                    candidates.push(next);
                }
            }
        }
    }
    unreachable!("no solution found");
}
