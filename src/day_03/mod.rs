use std::str::Lines;

struct LineIterator<'a> {
    lines: Lines<'a>,
    head: Option<&'a str>,
    mid: Option<&'a str>,
    tail: Option<&'a str>,
}

impl<'a> LineIterator<'a> {
    fn new(input: &'a str) -> Self {
        LineIterator {
            lines: input.lines(),
            head: None,
            mid: None,
            tail: None,
        }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = (Option<&'a str>, Option<&'a str>, Option<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.mid.is_none() {
            self.head = None;
            self.mid = self.lines.next();
            self.tail = self.lines.next();
        } else {
            self.head = self.mid;
            self.mid = self.tail;
            self.tail = self.lines.next();

            if self.head.is_none() {
                return None;
            }
        }

        Some((self.head, self.mid, self.tail))
    }
}

fn parse_num_at(line: &str, pos: usize) -> u32 {
    let pos = line
        .bytes()
        .enumerate()
        .rev()
        .skip(line.len() - pos - 1)
        .take_while(|(_, b)| b.is_ascii_digit())
        .map(|(i, _)| i)
        .last()
        .unwrap();

    line.bytes()
        .skip(pos)
        .take_while(|b| b.is_ascii_digit())
        .fold(0, |sum, digit| sum as u32 * 10 + (digit - b'0') as u32)
}

fn nums_at_nbours(line: &str, i: usize) -> [Option<u32>; 3] {
    let mut out = [None; 3];
    if matches!(line.as_bytes().get(i), Some(b) if b.is_ascii_digit()) {
        out[1] = Some(parse_num_at(line, i));
        // if the middle one has a number, then it must be the only one (since it would be neighbouring the other two positions)
        // exit now to avoid counting a number twice
        return out;
    }

    if i > 0 {
        if matches!(line.as_bytes().get(i - 1), Some(b) if b.is_ascii_digit()) {
            out[0] = Some(parse_num_at(line, i - 1));
        }
    }
    if matches!(line.as_bytes().get(i + 1), Some(b) if b.is_ascii_digit()) {
        out[2] = Some(parse_num_at(line, i + 1));
    }

    out
}

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> u32 {
    let mut line_iter = LineIterator::new(input);
    let mut sum = 0;

    while let Some((head, Some(mid_line), tail)) = line_iter.next() {
        for (i, _) in mid_line
            .bytes()
            .enumerate()
            .filter(|(_, b)| !b.is_ascii_digit() && b != &b'.')
        {
            let (head_line, tail_line) = (head.unwrap_or(""), tail.unwrap_or(""));
            sum += nums_at_nbours(head_line, i).iter().flatten().sum::<u32>();
            sum += nums_at_nbours(mid_line, i).iter().flatten().sum::<u32>();
            sum += nums_at_nbours(tail_line, i).iter().flatten().sum::<u32>();
        }
    }

    sum
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> u32 {
    let mut line_iter = LineIterator::new(input);
    let mut sum = 0;

    while let Some((head, Some(mid_line), tail)) = line_iter.next() {
        for (i, _) in mid_line.bytes().enumerate().filter(|(_, b)| b == &b'*') {
            let mut first = None;
            let (head_line, tail_line) = (head.unwrap_or(""), tail.unwrap_or(""));
            let items = nums_at_nbours(mid_line, i)
                .into_iter()
                .chain(nums_at_nbours(head_line, i))
                .chain(nums_at_nbours(tail_line, i));

            for item in items.flatten() {
                match first {
                    Some(first) => {
                        sum += item * first;
                        break;
                    }
                    None => first = Some(item),
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
        };
        assert_eq!(part_1(&input), 4361);
        assert_eq!(part_2(&input), 467835);
    }
}
