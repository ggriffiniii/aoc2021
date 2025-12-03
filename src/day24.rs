use aoc_runner_derive::{aoc, aoc_generator};

/*
Input file is machine instructions. One per digit. Each digit is remarkably
similar. There are only two large varieties of subroutine per digit.

first variety:

inp w
mul x 0
add x z
mod x 26
div z 1     <-- divide z by 1 (effectively a no-op)
add x 13    <-- immediate value changes for each digit
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14    <-- immediate value changes for each digit
mul y x
add z y

second variety:

inp w
mul x 0
add x z
mod x 26
div z 26    <-- divide z by 26
add x 0     <-- immediate value changes for each digit
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4     <-- immediate value changes for each digit
mul y x
add z y

Using the instructions above along with the additional constraints that the
input value will always be between 1 and 9 (inclusive) the subroutines above can
be distilled down to something like:

immediate_{4,5,15} are the immediate values provided in instruction 4 (div z), 5
(add x), and 15 (add y) respectively.

fn digit_subroutine(w, z, immediate_4, immediate_5, immediate_15) -> new_z {
    x = 0;
    x = x + z;
    x = x % 26;
    z = z / immediate_4;
    x = x + immediate_5;
    x = if x != w { 1 } else { 0 };
    y = 0;
    y = y + 25;
    y = y * x;
    y = y + 1;
    z = z * y;
    y = 0;
    y = w;
    y = y + immediate_15;
    y = y * x;
    z = z + y;
    return z;
}

further simplified:

fn digit_subroutine(w, z, immediate_4, immediate_5, immediate_15) -> new_z {
    x = z % 26 + immediate_5;
    z = z / immediate_4;
    x = if x != w { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + immediate_15) * x;
    z = z + y;
    return z;
}

now treating x as a boolean mask:

fn digit_subroutine(w, z, immediate_4, immediate_5, immediate_15) -> new_z {
    x = z % 26 + immediate_5;
    z = z / immediate_4;

    if x != w {
        z = (z * 26) + (w + immediate_15);
    }
    return z;
}

We know that immediate_4 is always either 1 or 26. Further inspection shows that
there are an equal number of routines where immediate_4 is 1 and 26 (7 of each).
Also whenever immediate_4 is 1 immediate_5 is greater than 9. This means that x
will never equal w in the conditional when immediate_4 is 1.

If we write out each of the variations as a separate routine it would look like:

fn digit_subroutine_one(w, z, immediate_15) -> new_z {
    return z * 26 + (w + immediate_15);
}

fn digit_subroutine_two(w, z, immediate_4, immediate_5, immediate_15) -> new_z {
    x = z % 26 + immediate_5;
    z = z / 26;
    if x != w {
        z = (z * 26) + (w + immediate_15);
    }
    return z;
}

Looking at this it's clear that if the `x == w` in subroutine_two then an
invocation of subroutine_two will return z back to where it was before the last
invocation of subroutine_one. `z % 26` within subroutine_two is going to be
equal to `w + immediate_15` in the previous invocation of subroutine_one. So the
goal is:

prev_w + prev_immediate_15 + immediate_5 == w

Note that every invocation of subroutine_two effectively undoes an invocation of
subroutine_one. So the `prev_` variables above are the variables from the
"matching" subroutine_one, not necessarily the most recent subroutine_one
invocation. e.g.

subroutine_one() ←────┐
subroutine_one() ←──┐ │
subroutine_two() ←──┘ │
subroutine_two() ←────┘

prev_immediate_15 and immediate_5 are hard-coded in the input. We only get to
control prev_w and w. The challenge is that we don't know what a valid value for
prev_w or w is until we know both prev_immediate_15 and immediate_5. So the w
value of subroutine_one will not be known until the matching subroutine_two is
reached.

Finally it makes sense.

When subroutine_one is encountered add it's immediate_15 to the stack along with
a reference to which digit of input it's associated with. When subroutine_two is
encountered pop the matching values from the stack. The goal is to choose prev_w and w values that satisfy

prev_w + prev_immediate_15 + immediate_5 == w

and given the choice we want the numbers to be as large as possible, preferring
to make prev_w larger than w. Now it's pretty simple.

prev_w = min(9, 9 - (prev_immediate_15 + immediate_5))
w = prev_w + prev_immediate_15 + immediate_5

*/

#[derive(Debug)]
enum SubRoutine {
    One { immediate_15: isize },
    Two { immediate_5: isize },
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<SubRoutine> {
    let mut subs = Vec::new();
    let mut lines = input.split("\n");

    while let Some("inp w") = lines.next() {
        let div_instr = lines.nth(3).unwrap();
        let immediate_5 = &lines.nth(0).unwrap()[6..];
        let immediate_15 = &lines.nth(9).unwrap()[6..];
        lines.nth(1).unwrap();

        subs.push(match div_instr {
            "div z 1" => SubRoutine::One {
                immediate_15: immediate_15.parse().unwrap(),
            },
            "div z 26" => SubRoutine::Two {
                immediate_5: immediate_5.parse().unwrap(),
            },
            _ => panic!("unrecognized subroutine"),
        })
    }
    subs
}

enum Part {
    One,
    Two,
}

fn solve(input: &[SubRoutine], part: Part) -> isize {
    let mut result = 0;
    let mut stack = Vec::new();
    for (digit, routine) in input.into_iter().enumerate() {
        match routine {
            SubRoutine::One { immediate_15 } => stack.push((digit, immediate_15)),
            SubRoutine::Two { immediate_5 } => {
                let (matching_digit, matching_immediate_15) = stack.pop().unwrap();
                let offset = matching_immediate_15 + immediate_5;

                let prev_w = match part {
                    Part::One => std::cmp::min(9, 9 - offset),
                    Part::Two => std::cmp::max(1, 1 - offset),
                };
                let w = prev_w + offset;
                result += prev_w * 10isize.pow((13 - matching_digit) as u32);
                result += w * 10isize.pow((13 - digit) as u32);
            }
        }
    }
    result
}

#[aoc(day24, part1)]
fn part1(input: &[SubRoutine]) -> isize {
    solve(input, Part::One)
}

#[aoc(day24, part2)]
fn part2(input: &[SubRoutine]) -> isize {
    solve(input, Part::Two)
}
