use std::ops::Index;

use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::tuple, Parser};

pub struct Input {
    data: Vec<u8>,
    width: usize,
    height: usize,
    // jump_table: Vec<usize>,
    // init_pos: usize,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let width = input.bytes().position(|b| b == b'\n').unwrap() + 1;
    let height = input.len() / width;

    Ok((
        "",
        Input {
            data: input.as_bytes().to_owned(),
            width,
            height,
        },
    ))
}

// struct Pos(usize, usize);

// impl Pos {
//     fn add(self, dpos: (isize, isize)) -> Option<Self> {
//         let new = (self.0 as isize + dpos.0, self.1 as isize + dpos.1);
//         if
//     }
// }

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

fn cycle_len(input: &Input) -> u32 {
    let start_pos = input
        .data
        .iter()
        .position(|b| *b == b'S')
        .expect("starting position not found");
    let starting_pos = (start_pos / input.width, start_pos % input.width);

    let mut first_moves = [None; 4];

    const STEP: (usize, usize) = (1, 1);
    if starting_pos.1 + STEP.1 < input.width {
        first_moves[0] = Some((starting_pos.0, starting_pos.1 + STEP.1));
    }
    if starting_pos.1 >= STEP.1 {
        first_moves[1] = Some((starting_pos.0, starting_pos.1 - STEP.1));
    }
    if starting_pos.0 + STEP.0 < input.height {
        first_moves[2] = Some((starting_pos.0 + STEP.0, starting_pos.1));
    }
    if starting_pos.0 >= STEP.0 {
        first_moves[3] = Some((starting_pos.0 - STEP.0, starting_pos.1));
    }

    'outer: for mut pos in first_moves.into_iter().flatten() {
        let mut last_pos = starting_pos;
        let mut steps = 0;
        while pos != starting_pos {
            steps += 1;
            let diffs = match input.data[pos.0 * input.width + pos.1] {
                b'|' => [(-1, 0), (1, 0)],
                b'-' => [(0, -1), (0, 1)],
                b'L' => [(-1, 0), (0, 1)],
                b'J' => [(0, -1), (-1, 0)],
                b'7' => [(0, -1), (1, 0)],
                b'F' => [(1, 0), (0, 1)],
                _ => continue 'outer,
            };

            let next_positions = &diffs.map(|diff| {
                let new_pos = (diff.0 + pos.0 as isize, diff.1 + pos.1 as isize);
                (new_pos.0 >= 0
                    && new_pos.0 < input.height as isize
                    && new_pos.1 >= 0
                    && new_pos.1 < input.width as isize)
                    .then(|| (new_pos.0 as usize, new_pos.1 as usize))
            });
            let next_pos = next_positions
                .iter()
                .flatten()
                .find(|next_pos| last_pos != **next_pos);

            if !next_positions.iter().flatten().any(|next_pos| last_pos == *next_pos) {
                continue 'outer;
            }

            if let Some(next_pos) = next_pos {
                (last_pos, pos) = (pos, *next_pos);
            } else {
                continue 'outer;
            }
        }
        return steps;
    }

    panic!("no solution found");
}

#[aoc(day10, part1)]
pub fn part_1(input: &Input) -> u32 {
    (cycle_len(input) + 1) / 2
}
#[aoc(day10, part2)]
pub fn part_2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
            "
        });
        assert_eq!(part_1(&input), 4);
        // assert_eq!(part_2(&input),);

        let input = input_generator(indoc! {
            "
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
            "
        });
        assert_eq!(part_1(&input), 8);
        // assert_eq!(part_2(&input),);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2023/day10.txt"));
        assert_eq!(part_1(&input), 6942);
        // assert_eq!(part_2(&input),);
    }
}
