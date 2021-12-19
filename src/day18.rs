use aoc_runner_derive::aoc;

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let sum = input
        .lines()
        .map(parse_pair_or_num)
        .reduce(|left, right| {
            let mut pair = PairOrNum::Pair(Box::new(left), Box::new(right));
            reduce(&mut pair);
            pair
        })
        .unwrap();
    magnitude(&sum)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    let numbers: Vec<_> = input.lines().map(parse_pair_or_num).collect();
    numbers
        .iter()
        .flat_map(|a| numbers.iter().map(move |b| (a, b)))
        .map(|(left, right)| {
            let mut pair = PairOrNum::Pair(Box::new(left.clone()), Box::new(right.clone()));
            reduce(&mut pair);
            magnitude(&pair)
        })
        .max()
        .unwrap()
}

#[derive(Debug, Clone)]
enum PairOrNum {
    Num(usize),
    Pair(Box<PairOrNum>, Box<PairOrNum>),
}

fn parse_pair_or_num(input: &str) -> PairOrNum {
    fn _parse_pair_or_num(i: &[u8]) -> (&[u8], PairOrNum) {
        if i[0] == b'[' {
            let (rem, left) = _parse_pair_or_num(&i[1..]);
            let (rem, right) = _parse_pair_or_num(&rem[1..]);
            (&rem[1..], PairOrNum::Pair(Box::new(left), Box::new(right)))
        } else {
            (&i[1..], PairOrNum::Num((i[0] - b'0') as usize))
        }
    }
    let (rem, pn) = _parse_pair_or_num(input.as_bytes());
    assert!(rem.is_empty());
    pn
}

fn reduce(pn: &mut PairOrNum) {
    loop {
        while explode(pn) {}
        if !split(pn) {
            return;
        }
    }
}

fn magnitude(pn: &PairOrNum) -> usize {
    match pn {
        &PairOrNum::Num(x) => x,
        PairOrNum::Pair(left, right) => 3 * magnitude(&*left) + 2 * magnitude(&*right),
    }
}

/// Returns true if any numbers exploded
fn explode(pn: &mut PairOrNum) -> bool {
    fn _add_left(pn: &mut PairOrNum, v: usize) {
        match pn {
            PairOrNum::Num(x) => {
                *x += v;
            }
            PairOrNum::Pair(left, _) => _add_left(&mut *left, v),
        }
    }
    fn _add_right(pn: &mut PairOrNum, v: usize) {
        match pn {
            PairOrNum::Num(x) => {
                *x += v;
            }
            PairOrNum::Pair(_, right) => _add_right(&mut *right, v),
        }
    }
    fn _explode(pn: &mut PairOrNum, depth: usize) -> (bool, Option<usize>, Option<usize>) {
        match pn {
            PairOrNum::Num(_) => (false, None, None),
            PairOrNum::Pair(left, right) if depth > 3 => match (&**left, &**right) {
                (PairOrNum::Num(left), PairOrNum::Num(right)) => {
                    let res = (true, Some(*left), Some(*right));
                    *pn = PairOrNum::Num(0);
                    res
                }
                _ => panic!(),
            },
            PairOrNum::Pair(left, right) => {
                let (exploded, left_left, left_right) = _explode(&mut *left, depth + 1);
                if exploded {
                    if let Some(left_right) = left_right {
                        _add_left(&mut *right, left_right);
                    }
                    return (true, left_left, None);
                }
                let (exploded, right_left, right_right) = _explode(&mut *right, depth + 1);
                if exploded {
                    if let Some(right_left) = right_left {
                        _add_right(&mut *left, right_left);
                    }
                    return (true, None, right_right);
                }
                (false, None, None)
            }
        }
    }
    _explode(pn, 0).0
}

/// Returns true if any numbers were split
fn split(pn: &mut PairOrNum) -> bool {
    match pn {
        &mut PairOrNum::Num(x) => {
            if x >= 10 {
                *pn = PairOrNum::Pair(
                    Box::new(PairOrNum::Num(x / 2)),
                    Box::new(PairOrNum::Num((x + 1) / 2)),
                );
                true
            } else {
                false
            }
        }
        PairOrNum::Pair(ref mut left, ref mut right) => split(&mut *left) || split(&mut *right),
    }
}
