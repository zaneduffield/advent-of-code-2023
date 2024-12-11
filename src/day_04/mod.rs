use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, space0, space1, u8},
    multi::separated_list0,
    sequence::tuple,
    Parser,
};

pub struct Input {
    cards: Vec<Card>,
}

pub struct Card {
    winners: Vec<u8>,
    mine: Vec<u8>,
}

pub type IResult<'a, T> = nom::IResult<&'a str, T>;

pub fn input_generator(input: &str) -> Input {
    let cards: IResult<Vec<Card>> = separated_list0(
        line_ending,
        tuple((
            tuple((tag("Card"), space1, digit1, tag(":"), space0)),
            separated_list0(space1, u8),
            tuple((space0, char('|'), space0)),
            separated_list0(space1, u8),
        ))
        .map(|(_, mut winners, _, mut mine)| {
            winners.sort();
            mine.sort();
            Card { winners, mine }
        }),
    )(input);
    let cards = cards.expect("failed to parse input");

    assert!(cards.0.trim().is_empty(), "failed to parse entire input");

    Input { cards: cards.1 }
}

pub fn part_1(input: &Input) -> u32 {
    input
        .cards
        .iter()
        .map(|card| {
            // TODO refactor into iterator of winners and share with part 2
            let mut winners = card.winners.iter().peekable();
            card.mine.iter().fold(0, |mut score, num| {
                loop {
                    match winners.peek() {
                        None => break,
                        Some(w) if *w == num => {
                            score = (2 * score).max(1);
                            break;
                        }
                        Some(w) if *w > num => break,
                        _ => {
                            winners.next();
                        }
                    }
                }
                score
            })
        })
        .sum()
}

pub fn part_2(input: &Input) -> u32 {
    let mut counts = vec![1; input.cards.len()];

    input
        .cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            // TODO refactor into iterator of winners and share with part 2
            let mut winners = card.winners.iter().peekable();
            let num_winners = card
                .mine
                .iter()
                .filter(|num| {
                    loop {
                        match winners.peek() {
                            None => break,
                            Some(w) if w == num => {
                                return true;
                            }
                            Some(w) if *w > num => break,
                            _ => {
                                winners.next();
                            }
                        }
                    }
                    false
                })
                .count();

            ((i + 1)..(i + 1 + num_winners)).for_each(|j| counts[j] += counts[i]);
            counts[i]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        });
        assert_eq!(part_1(&input), 13);
        assert_eq!(part_2(&input), 30);
    }
}
