const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve(input: &str, f: impl Fn(&str) -> (u32, u32)) -> u32 {
    input
        .lines()
        .map(f)
        .map(|(first, last)| first * 10 + last)
        .sum()
}

fn part_1_next_digit(line: impl IntoIterator<Item = u8>) -> u8 {
    line.into_iter()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .next()
        .expect("no digit found in line")
}

fn part_1_digits(line: &str) -> (u32, u32) {
    (
        part_1_next_digit(line.bytes()) as u32,
        part_1_next_digit(line.bytes().rev()) as u32,
    )
}

pub fn part_1(input: &str) -> u32 {
    solve(input, part_1_digits)
}

fn next_digit_part_2(line: &str, range: impl IntoIterator<Item = usize>) -> Option<u32> {
    range.into_iter().find_map(|i| {
        let byte = line.as_bytes()[i];
        if byte.is_ascii_digit() {
            Some((byte - b'0') as u32)
        } else {
            DIGIT_WORDS
                .iter()
                .enumerate()
                .filter(|(_, w)| line[i..].starts_with(**w))
                .map(|(j, _)| j as u32 + 1)
                .next()
        }
    })
}

fn part_2_digits(line: &str) -> (u32, u32) {
    (
        next_digit_part_2(line, 0..line.len()).expect("no digit found in line"),
        next_digit_part_2(line, (0..line.len()).rev()).expect("no digit found in line"),
    )
}

pub fn part_2(input: &str) -> u32 {
    solve(input, part_2_digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input1 = indoc! {
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "
        };
        assert_eq!(part_1(input1), 142);

        let input2 = indoc! {
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "
        };
        assert_eq!(part_2(input2), 281);
    }
}
