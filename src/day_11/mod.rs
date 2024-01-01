use itertools::Itertools;
use num::Integer;

pub struct Input {
    data: Vec<u8>,
    width: usize,
    height: usize,
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

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

fn solve(input: &Input, growth: u64) -> u64 {
    let expand_count_hor = {
        let mut grid = vec![0; input.data.len()];
        'outer: for j in 0..input.height {
            for i in 0..input.width {
                let idx = j * input.width + i;
                if input.data[idx] == b'#' {
                    continue 'outer;
                }
            }
            for i in 0..input.width {
                // note rotation of grid for linear access later
                let idx = i * input.height + j;
                grid[idx] = growth;
            }
        }
        grid
    };
    let expand_count_ver = {
        let mut grid = vec![0; input.data.len()];
        'outer: for i in 0..input.width {
            for j in 0..input.height {
                let idx = j * input.width + i;
                if input.data[idx] == b'#' {
                    continue 'outer;
                }
            }
            for j in 0..input.height {
                let idx = j * input.width + i;
                grid[idx] = growth;
            }
        }
        grid
    };

    let galaxies = input.data.iter().positions(|b| *b == b'#').collect_vec();
    let mut sum = 0;
    for (from, to) in galaxies.iter().tuple_combinations() {
        let from_pos = from.div_rem(&input.width);
        let to_pos = to.div_rem(&input.width);
        let steps_ver =
            (from_pos.0.min(to_pos.0)..from_pos.0.max(to_pos.0)).fold(0, |steps, pos| {
                // note rotation of grid
                steps + 1 + expand_count_hor[from_pos.1 * input.height + (pos + 1)]
            });
        let steps_hor = (from_pos.1.min(to_pos.1)..from_pos.1.max(to_pos.1))
            .fold(0, |steps, pos| {
                steps + 1 + expand_count_ver[from_pos.0 * input.width + (pos + 1)]
            });

        let steps = steps_hor + steps_ver;
        sum += steps;
    }

    sum
}

#[aoc(day11, part1)]
pub fn part_1(input: &Input) -> u64 {
    solve(input, 1)
}

#[aoc(day11, part2)]
pub fn part_2(input: &Input) -> u64 {
    solve(input, 1_000_000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            "
        });
        assert_eq!(part_1(&input), 374);
        assert_eq!(part_2(&input), 82000210);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2023/day11.txt"));
        assert_eq!(part_1(&input), 9693756);
        assert_eq!(part_2(&input), 717878258016);
    }
}
