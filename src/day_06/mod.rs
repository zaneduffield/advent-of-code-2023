use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

pub struct Input {
    times: Vec<u64>,
    best_distances: Vec<u64>,
}

const START_SPEED_MM_PER_MS: u64 = 0;
const ACCEL_MM_PER_MS_2: u64 = 1;

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    tuple((
        tag("Time:"),
        space1,
        separated_list1(space1, u64),
        line_ending,
        tag("Distance:"),
        space1,
        separated_list1(space1, u64),
    ))
    .map(|(_, _, times, _, _, _, best_distances)| Input {
        times,
        best_distances,
    })
    .parse(input)
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u64 {
    input
        .times
        .iter()
        .zip(&input.best_distances)
        .map(|(&time, &dist)| count_ways_to_win(time, dist))
        .product()
}

fn count_ways_to_win(time: u64, dist: u64) -> u64 {
    // d < (v_0 + a * t)(T - t)
    // 0 < -at^2 + (aT - v_0)t + Tv_0 - d
    // roots are
    //   (v_0 - aT +- sqrt((v_0 - aT)^2 + 4a(Tv_0 - d))) / (-2a)
    // solutions to the inequality are between the roots

    const V0: i64 = START_SPEED_MM_PER_MS as i64;
    const A: i64 = ACCEL_MM_PER_MS_2 as i64;
    let sqrt_discriminant =
        (((V0 - A * time as i64).pow(2) + 4 * A * (V0 * time as i64 - dist as i64)) as f64).sqrt();

    let left_term = V0 - A * time as i64;
    let left_root = (left_term as f64 + sqrt_discriminant) / (-2f64 * A as f64);
    let right_root = (left_term as f64 - sqrt_discriminant) / (-2f64 * A as f64);

    1 + ((right_root - 1f64).ceil() as u64).min(time) - ((left_root + 1f64).floor() as u64).max(0)
}

pub fn part_2(input: &Input) -> u64 {
    let time = merge_nums(&input.times);
    let dist = merge_nums(&input.best_distances);

    count_ways_to_win(time, dist)
}

fn merge_nums(nums: &[u64]) -> u64 {
    nums.iter()
        .fold(0, |sum, next| sum * 10_u64.pow(1 + next.ilog10()) + *next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Time:      7  15   30
            Distance:  9  40  200
            "
        });
        assert_eq!(part_1(&input), 288);
        assert_eq!(part_2(&input), 71503);
    }
}
