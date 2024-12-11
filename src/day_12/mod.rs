use nom::{
    branch::alt,
    character::complete::*,
    multi::*,
    sequence::separated_pair,
    Parser,
};

pub struct Input {
    records: Vec<Record>,
}

#[derive(Clone)]
pub struct Record {
    springs: Vec<Spring>,
    group_sizes: Vec<u8>,
}

#[derive(Copy, Clone)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_spring(input: &str) -> nom::IResult<&str, Spring> {
    alt((
        char('.').map(|_| Spring::Operational),
        char('#').map(|_| Spring::Damaged),
        char('?').map(|_| Spring::Unknown),
    ))(input)
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, records) = separated_list0(
        line_ending,
        separated_pair(
            many0(parse_spring),
            char(' '),
            separated_list0(char(','), u8),
        )
        .map(|(springs, group_sizes)| Record {
            springs,
            group_sizes,
        }),
    )(input)?;
    Ok((input, Input { records }))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

impl Record {
    fn is_inconsistent(&self) -> bool {
        let mut spring_idx = 0;
        let mut group_size = 0;
        let mut max_group_size = 0;
        while spring_idx < self.springs.len() {
            if matches!(self.springs[spring_idx], Spring::Damaged) {
                group_size += 1;
            }

            if group_size > 0
                && (spring_idx == self.springs.len() - 1
                    || matches!(
                        self.springs[spring_idx],
                        Spring::Operational | Spring::Unknown
                    ))
            {
                max_group_size = max_group_size.max(group_size);
                group_size = 0;
            }

            spring_idx += 1;
        }

        &max_group_size > self.group_sizes.iter().max().unwrap_or(&0)
    }

    fn is_solved(&self) -> bool {
        let mut group_idx = 0;
        let mut spring_idx = 0;
        let mut group_size = 0;
        while spring_idx < self.springs.len() {
            if matches!(self.springs[spring_idx], Spring::Damaged) {
                group_size += 1;
            }

            if group_size > 0
                && (spring_idx == self.springs.len() - 1
                    || matches!(self.springs[spring_idx], Spring::Operational))
            {
                if self
                    .group_sizes
                    .get(group_idx)
                    .filter(|size| group_size == **size)
                    .is_none()
                {
                    return false;
                }
                group_idx += 1;
                group_size = 0;
            }

            spring_idx += 1;
        }

        group_idx == self.group_sizes.len()
    }
}

fn count_possibilities_rec(record: &Record, pos: usize, last_group_idx: Option<usize>) -> u32 {
    let mut count = 0;
    let next_group_idx = last_group_idx.map(|p| p + 1).unwrap_or(0);

    if next_group_idx >= record.group_sizes.len() {
        return if record.is_solved() { 1 } else { 0 };
    }
    let group_size = record.group_sizes[next_group_idx] as usize;

    for next_available_pos in
        record
            .springs
            .iter()
            .enumerate()
            .skip(pos)
            .filter_map(|(i, spring)| {
                (i + group_size <= record.springs.len()
                    && matches!(spring, Spring::Unknown | Spring::Damaged))
                .then_some(i)
            })
    {
        if record
            .springs
            .iter()
            .skip(next_available_pos)
            .take(group_size)
            .any(|s| matches!(s, Spring::Operational))
        {
            continue;
        }

        let mut next = record.clone();

        next.springs
            .iter_mut()
            .take(next_available_pos)
            .skip(pos)
            .take_while(|s| matches!(s, Spring::Unknown))
            .for_each(|s| *s = Spring::Operational);

        next.springs
            .iter_mut()
            .skip(next_available_pos)
            .take(group_size)
            .for_each(|s| *s = Spring::Damaged);

        if next.is_inconsistent() {
            continue;
        }

        count += if next_group_idx == next.group_sizes.len() - 1 && next.is_solved() {
            1
        } else {
            count_possibilities_rec(&next, next_available_pos + group_size, Some(next_group_idx))
        };
    }

    count
}

fn count_possibilities(record: &Record) -> u32 {
    /*
       the idea here is to do a kind of DFS over the possible locations of the groups, pruning paths as they become inconsistent.
       Start with the leftmost group and find the iterate over the possible locations of it
       for each location toggle the required unknown positions (or skip if impossible) and start considering the next group with the altered state.
       if you get all the way to the end and find a valid state for the final group, then count it as a solution.
    */
    let out = count_possibilities_rec(record, 0, None);
    eprintln!("{out}");
    out
}

pub fn part_1(input: &Input) -> u32 {
    input.records.iter().map(count_possibilities).sum()
}

pub fn part_2(input: &Input) -> u32 {
    /*
       could split into 'overlapping' and 'non-overlapping' cases.
       in the 'non-overlapping' case, we can enumerate all the possible ways
       to insert 4 joining points in the new long list of group sizes, and for
       each of those ways, solve the original problem in each split and take
       the product of that as our final count.

       in the 'overlapping' case, you enumerate all the ways to pick 4 groups
       from the long list of group sizes to be sitting on the boundary, and for
       each of them you solve the original problem with the 4 groups fixed on the boundary
       (for each possible position of the group on the boundary).
    */
    input
        .records
        .iter()
        .map(|r| Record {
            springs: [(); 5].map(|_| r.springs.clone()).join(&Spring::Unknown),
            group_sizes: r.group_sizes.repeat(5),
        })
        .map(|r| count_possibilities(&r))
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        assert_eq!(part_1(&input_generator("???.### 1,1,3")), 1);
        assert_eq!(part_1(&input_generator(".??..??...?##. 1,1,3")), 4);
        assert_eq!(part_1(&input_generator("?#?#?#?#?#?#?#? 1,3,1,6")), 1);
        assert_eq!(part_1(&input_generator("????.#...#... 4,1,1")), 1);
        assert_eq!(part_1(&input_generator("????.######..#####. 1,6,5")), 4);
        assert_eq!(part_1(&input_generator("?###???????? 3,2,1")), 10);

        let input = input_generator(indoc! {
            "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
            "
        });
        assert_eq!(part_1(&input), 21);

        assert_eq!(part_2(&input_generator("???.### 1,1,3")), 1);
        assert_eq!(part_2(&input_generator(".??..??...?##. 1,1,3")), 16384);
        assert_eq!(part_2(&input_generator("?#?#?#?#?#?#?#? 1,3,1,6")), 1);
        assert_eq!(part_2(&input_generator("????.#...#... 4,1,1")), 16);
        assert_eq!(part_2(&input_generator("????.######..#####. 1,6,5")), 2500);
        assert_eq!(part_2(&input_generator("?###???????? 3,2,1")), 506250);

        assert_eq!(part_2(&input), 525152);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2023/day12.txt"));
        assert_eq!(part_1(&input), 7047);
        // assert_eq!(part_2(&input),);
    }

    #[test]
    fn test_record_is_solved() {
        let input = input_generator(indoc! {
            "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
            "
        });

        for rec in &input.records {
            assert!(!rec.is_solved());
        }

        let input = input_generator(indoc! {
            "
            #.#.### 1,1,3
            .#...#....###. 1,1,3
            .#.###.#.###### 1,3,1,6
            ####.#...#... 4,1,1
            #....######..#####. 1,6,5
            .###.##....# 3,2,1
            "
        });

        for rec in &input.records {
            assert!(rec.is_solved());
        }
    }
}
