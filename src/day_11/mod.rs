use itertools::Itertools;

pub struct Input {
    points: Vec<(u64, u64)>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let points = input
        .lines()
        .enumerate()
        .fold(vec![], |mut points, (row, line)| {
            points.extend(
                line.bytes()
                    .enumerate()
                    .filter_map(|(col, b)| (b == b'#').then_some((row as u64, col as u64))),
            );
            points
        });

    Ok(("", Input { points }))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

fn solve(input: &Input, growth: u64) -> u64 {
    let mut points = input.points.clone();

    {
        let mut expansion = 0;
        let mut last_0 = 0;
        for p in &mut points {
            if p.0 != last_0 {
                expansion += (p.0 - last_0 - 1) * growth;
                last_0 = p.0;
            }
            p.0 += expansion;
        }
    }

    points.sort_by_key(|p| p.1);
    {
        let mut expansion = 0;
        let mut last_1 = 0;
        for p in &mut points {
            if p.1 != last_1 {
                expansion += (p.1 - last_1 - 1) * growth;
                last_1 = p.1;
            }
            p.1 += expansion;
        }
    }

    let mut sum = 0;
    for (p1, p2) in points.iter().tuple_combinations() {
        sum += p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);
    }

    sum
}

pub fn part_1(input: &Input) -> u64 {
    solve(input, 1)
}

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
