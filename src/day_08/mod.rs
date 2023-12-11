use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, multispace1},
    combinator::fail,
    multi::{many1, separated_list0},
    sequence::tuple,
    Parser,
};
use rustc_hash::FxHashMap;

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
    let mut cycles = vec![];
    for (id, _) in input.starting_mask.iter().enumerate().filter(|(_, b)| **b) {
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

    #[cfg(debug_assertions)]
    for cycle in &cycles {
        eprintln!("{cycle:?}")
    }

    // stupid lcm answer is somehow right for my input, but it doesn't generalise
    // cycles.iter().map(|c| c.start_offset + c.end_offsets[0]).reduce(num::integer::lcm).unwrap() as u64

    let starting_offset = cycles.iter().map(|cycle| cycle.start_offset).max().unwrap();
    for cycle in &mut cycles {
        let cycle_starting_offset = starting_offset - cycle.start_offset;
        let offset_in_cycle = cycle_starting_offset % cycle.period;
        for goal_offset in &mut cycle.end_offsets {
            *goal_offset = (*goal_offset + cycle.period - offset_in_cycle) % cycle.period;
        }
        cycle.start_offset = 0;
    }

    let mut new_offsets = Vec::new();

    // TODO understand and rewrite / convert to CRT
    let full_cycle =
        cycles
            .into_iter()
            .fold((1, vec![0]), |(acc_period, mut acc_offsets), cycle| {
                let mut pos = 0;
                let cycle_period = cycle.period;
                new_offsets.clear();
                loop {
                    for &i in &acc_offsets {
                        if cycle.end_offsets.contains(&((pos + i) % cycle_period)) {
                            new_offsets.push(pos + i);
                        }
                    }
                    pos += acc_period;
                    if pos % cycle_period == 0 {
                        std::mem::swap(&mut new_offsets, &mut acc_offsets);
                        break (pos, acc_offsets);
                    }
                }
            });

    (starting_offset + full_cycle.1[0]) as u64
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
