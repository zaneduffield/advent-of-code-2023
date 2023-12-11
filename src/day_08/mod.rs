use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending, multispace1},
    combinator::fail,
    multi::{many1, separated_list0},
    sequence::tuple,
    Parser,
};
use num::complex::ComplexFloat;
use ring_algorithm::chinese_remainder_theorem;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct Input {
    init: Option<Id>,
    goal: Option<Id>,
    instructions: Vec<Direction>,
    elements: Vec<(Id, Id)>,
    starting_mask: Vec<bool>,
    ending_mask: Vec<bool>,
}

pub enum Direction {
    Left,
    Right,
}

type Id = u16;

fn parse_dir(input: &str) -> nom::IResult<&str, Direction> {
    match input.chars().next() {
        Some('L') => Ok((&input[1..], Direction::Left)),
        Some('R') => Ok((&input[1..], Direction::Right)),
        _ => fail(input),
    }
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, instructions) = many1(parse_dir)(input)?;
    let (input, _) = multispace1(input)?;

    let (input, elements_str) = separated_list0(
        line_ending,
        tuple((
            alphanumeric1,
            tag(" = ("),
            alphanumeric1,
            tag(", "),
            alphanumeric1,
            tag(")"),
        ))
        .map(|(from, _, left, _, right, _)| (from, left, right)),
    )(input)?;

    let mut id_by_name = FxHashMap::<&str, Id>::default();
    let mut elements = Vec::with_capacity(elements_str.len());
    let mut init = None;
    let mut goal = None;
    let mut starting_mask = vec![];
    let mut ending_mask = vec![];
    for (from, _, _) in &elements_str {
        let len = id_by_name.len() as u16;
        let id = id_by_name.entry(from).or_insert(len);
        if from.eq(&"AAA") {
            init = Some(*id);
        } else if from.eq(&"ZZZ") {
            goal = Some(*id);
        }
        starting_mask.push(from.ends_with('A'));
        ending_mask.push(from.ends_with('Z'));
    }

    for (_, left, right) in &elements_str {
        let len = id_by_name.len() as u16;
        let id_left = *id_by_name.entry(left).or_insert(len);
        let len = id_by_name.len() as u16;
        let id_right = *id_by_name.entry(right).or_insert(len);
        elements.push((id_left, id_right));
    }

    Ok((
        input,
        Input {
            instructions,
            elements,
            init,
            goal,
            starting_mask,
            ending_mask,
        },
    ))
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

#[aoc(day8, part1)]
pub fn part_1(input: &Input) -> u64 {
    let mut id = input.init.expect("starting element not found");
    let goal = input.goal.expect("goal element not found");
    let mut steps = 0;
    while id != goal {
        let (left, right) = input.elements[id as usize];
        id = match input.instructions[steps % input.instructions.len()] {
            Direction::Left => left,
            Direction::Right => right,
        };
        steps += 1;
    }

    steps as u64
}

#[derive(Debug)]

struct Cycle {
    start_offset: usize,
    period: usize,
    end_offsets: Vec<usize>,
}

#[aoc(day8, part2)]
pub fn part_2(input: &Input) -> u64 {
    // idea: anayse every starting point and compute its cycle (when it starts, how long it goes for, what positions along its path it is on a finishing position)
    // input.starting_mask.iter().enumerate().filter(|(_, b)| **b).map(|(id, _)| )

    let mut cycles = vec![];
    for (id, _) in input.starting_mask.iter().enumerate().filter(|(i, b)| **b) {
        let mut ends = vec![];
        let mut steps = 0;
        let mut id = id as u16;
        let mut visited = FxHashMap::default();
        loop {
            let instr_pos = steps % input.instructions.len();
            if let Some(start_offset) = visited.insert((id, instr_pos), steps) {
                ends.retain(|end| *end >= start_offset);
                let period = steps - start_offset;
                let end_offsets = ends.into_iter().map(|o| o - start_offset).collect();
                cycles.push(Cycle {
                    start_offset,
                    period,
                    end_offsets,
                });
                break;
            }
            let (left, right) = input.elements[id as usize];
            id = match input.instructions[instr_pos] {
                Direction::Left => left,
                Direction::Right => right,
            };
            steps += 1;

            if input.ending_mask[id as usize] {
                ends.push(steps);
            }
        }
    }

    for cycle in &cycles {
        eprintln!("{cycle:?}")
    }

    // stupid lcm answer is somehow right for my input, but it doesn't generalise
    // cycles.iter().map(|c| c.start_offset + c.end_offsets[0]).reduce(num::integer::lcm).unwrap() as u64

    let best_cycle_to_iter_idx = cycles
        .iter()
        .position_max_by_key(|c| c.period / c.end_offsets.len())
        .unwrap();
    let best_cycle_to_iter = cycles.swap_remove(best_cycle_to_iter_idx);

    let mut steps = best_cycle_to_iter.start_offset;
    let mut loops = 0;
    loop {
        if let Some(solution) = best_cycle_to_iter.end_offsets.iter().find(|end| {
            let steps_to_end = steps + **end;
            cycles.iter().all(|c| {
                c.end_offsets
                    .iter()
                    .any(|end| (steps_to_end - c.start_offset) % c.period == *end)
            })
        }) {
            return (steps + solution) as u64;
        }
        steps += best_cycle_to_iter.period;
        loops += 1;
        if loops % (2 << 26) == 0 {
            eprintln!("{steps}");
        }
    }
}

fn egcd_(a: usize, b: usize) -> (usize, usize, usize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd_(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn crt_(remo: &[(usize, usize)]) -> usize {
    let prod = remo.iter().map(|n| n.1).product::<usize>();
    remo.iter()
        .map(|(re, mo)| {
            let p = prod / mo;
            re * ((egcd_(p, *mo).1 % mo + mo) % mo) * p
        })
        .sum::<usize>()
        % prod
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
            "
        });
        assert_eq!(part_1(&input), 2);
        let input2 = input_generator(indoc! {
            "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
            "
        });
        assert_eq!(part_1(&input2), 6);

        let input3 = input_generator(indoc! {
            "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
            "
        });
        assert_eq!(part_2(&input3), 6);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2023/day8.txt"));
        assert_eq!(part_2(&input), 16342438708751);
    }
}
