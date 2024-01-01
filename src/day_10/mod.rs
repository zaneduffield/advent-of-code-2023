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

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

fn find_cycle(input: &Input) -> (u8, Vec<usize>) {
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

    'outer: for first_move in first_moves.into_iter().flatten() {
        let mut pos = first_move;
        let mut last_pos = starting_pos;
        let mut cycle = vec![starting_pos.0 * input.width + starting_pos.1];
        while pos != starting_pos {
            let idx = pos.0 * input.width + pos.1;
            cycle.push(idx);
            let diffs = match input.data[idx] {
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
                if new_pos.0 >= 0
                    && new_pos.0 < input.height as isize
                    && new_pos.1 >= 0
                    && new_pos.1 < input.width as isize
                {
                    Some((new_pos.0 as usize, new_pos.1 as usize))
                } else {
                    None
                }
            });
            let next_pos = next_positions
                .iter()
                .flatten()
                .find(|next_pos| last_pos != **next_pos);

            if !next_positions
                .iter()
                .flatten()
                .any(|next_pos| last_pos == *next_pos)
            {
                continue 'outer;
            }

            if let Some(next_pos) = next_pos {
                (last_pos, pos) = (pos, *next_pos);
            } else {
                continue 'outer;
            }
        }

        // identify shape of 'S' pipe
        let first = (cycle[0] as isize).div_rem(&(input.width as isize));
        let second = (cycle[1] as isize).div_rem(&(input.width as isize));
        let last = (*cycle.last().unwrap() as isize).div_rem(&(input.width as isize));
        let mut diffs = [
            (second.0 - first.0, second.1 - first.1),
            (last.0 - first.0, last.1 - first.1),
        ];
        diffs.sort();

        let pipe = match diffs {
            [(1, 0), (1, 0)] => b'|',
            [(0, 1), (0, 1)] => b'-',
            [(-1, 0), (0, 1)] => b'L',
            [(-1, 0), (0, -1)] => b'J',
            [(0, -1), (1, 0)] => b'7',
            [(0, 1), (1, 0)] => b'F',
            _ => panic!("failed to identify shape of S piece"),
        };

        return (pipe, cycle);
    }

    panic!("no solution found");
}

#[aoc(day10, part1)]
pub fn part_1(input: &Input) -> usize {
    (find_cycle(input).1.len() + 1) / 2
}

#[aoc(day10, part2)]
pub fn part_2(input: &Input) -> usize {
    let (starting_pipe, cycle) = find_cycle(input);
    let mut pipes = input.data.clone();
    pipes[cycle[0]] = starting_pipe;

    let cycle_table = cycle
        .iter()
        .fold(vec![false; input.data.len()], |mut acc, idx| {
            acc[*idx] = true;
            acc
        });

    /*
       The idea here is to scan the grid left-to-right up-to-down and keep track of whether we are inside the loop.
       Only the following shapes will 'flip' the state of being inside the loop:
       - |
       - FJ (with any number of '-' in the middle)
       - L7 (with any number of '-' in the middle)
    */
    let mut count_inside = 0;
    let mut inside = false;
    let mut start_pipe = 0u8;
    for j in 0..input.height {
        for i in 0..input.width {
            let idx = j * input.width + i;
            if !cycle_table[idx] {
                if inside {
                    count_inside += 1;
                }
            } else {
                match pipes[idx] {
                    b'|' => inside = !inside,
                    b'-' => {}
                    pipe @ (b'L' | b'F') => start_pipe = pipe,
                    b'J' if start_pipe == b'F' => inside = !inside,
                    b'7' if start_pipe == b'L' => inside = !inside,
                    _ => start_pipe = 0,
                }
            }
        }
    }

    count_inside
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
        assert_eq!(part_2(&input), 1);

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
        assert_eq!(part_2(&input), 1);

        let input = input_generator(indoc! {
            "
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
            "
        });
        assert_eq!(part_2(&input), 4);

        let input = input_generator(indoc! {
            "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
            "
        });
        assert_eq!(part_2(&input), 8);

        let input = input_generator(indoc! {
            "
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJIF7FJ-
            L---JF-JLJIIIIFJLJJ7
            |F|F-JF---7IIIL7L|7|
            |FFJF7L7F-JF7IIL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
           "
        });
        assert_eq!(part_2(&input), 10);

        let input = input_generator(indoc! {
            "
            .S7F7..
            .|||L-7
            .|LJF-J
            .L7.L-7
            .FJF7FJ
            .L7|||.
            ..LJLJ.
            "
        });
        assert_eq!(part_2(&input), 1);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2023/day10.txt"));
        assert_eq!(part_1(&input), 6942);
        assert_eq!(part_2(&input), 297);
    }
}
