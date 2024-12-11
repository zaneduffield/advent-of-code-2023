use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    character::complete::{anychar, line_ending, space1, u32},
    multi::{count, separated_list0},
    sequence::tuple,
    Parser,
};

pub struct Input {
    hands: Vec<(Hand, u32)>,
}

pub struct Hand {
    cards: [u8; 5],
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, hands) = separated_list0(
        line_ending,
        tuple((
            count(anychar, 5).map(|nums| Hand {
                cards: nums
                    .iter()
                    .map(|c| *c as u8)
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            }),
            space1,
            u32,
        ))
        .map(|(a, _, b)| (a, b)),
    )(input)?;
    Ok((input, Input { hands }))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

#[derive(Debug, PartialEq, Eq)]
enum HandKind {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn eval_hand(hand: &Hand) -> HandKind {
    if hand.cards.iter().all(|elm| *elm == hand.cards[0]) {
        return HandKind::FiveKind;
    } else if hand
        .cards
        .iter()
        .any(|elm| hand.cards.iter().filter(|elm2| *elm == **elm2).count() == 4)
    {
        return HandKind::FourKind;
    }

    if let Some(triple) = hand
        .cards
        .iter()
        .find(|elm| hand.cards.iter().filter(|elm2| *elm == *elm2).count() == 3)
    {
        if hand.cards.iter().any(|elm| {
            *elm != *triple && hand.cards.iter().filter(|elm2| *elm == **elm2).count() == 2
        }) {
            return HandKind::FullHouse;
        }
        return HandKind::ThreeKind;
    }

    if let Some(pair) = hand
        .cards
        .iter()
        .find(|elm| hand.cards.iter().filter(|elm2| *elm == *elm2).count() == 2)
    {
        if hand.cards.iter().any(|elm| {
            *elm != *pair && hand.cards.iter().filter(|elm2| *elm == **elm2).count() == 2
        }) {
            return HandKind::TwoPair;
        }
        return HandKind::OnePair;
    }

    HandKind::HighCard
}

const JOKER: u8 = b'J';

fn eval_hand_joker(hand: &Hand) -> HandKind {
    if hand.cards.iter().any(|elm1| {
        hand.cards
            .iter()
            .all(|elm2| *elm2 == *elm1 || *elm2 == JOKER)
    }) {
        return HandKind::FiveKind;
    } else if hand.cards.iter().any(|elm| {
        hand.cards
            .iter()
            .filter(|elm2| *elm == **elm2 || **elm2 == JOKER)
            .count()
            == 4
    }) {
        return HandKind::FourKind;
    }

    let mut jokers_used = 0;
    if let Some(triple) = hand.cards.iter().find(|elm| {
        **elm != JOKER
            && hand
                .cards
                .iter()
                .filter(|elm2| {
                    if **elm2 == JOKER {
                        jokers_used += 1;
                        true
                    } else {
                        *elm == *elm2
                    }
                })
                .count()
                == 3
    }) {
        if hand.cards.iter().any(|elm| {
            let mut this_jokers_used = jokers_used;
            *elm != *triple
                && hand
                    .cards
                    .iter()
                    .filter(|elm2| {
                        if **elm2 == JOKER {
                            if this_jokers_used <= 0 {
                                true
                            } else {
                                this_jokers_used -= 1;
                                false
                            }
                        } else {
                            *elm == **elm2
                        }
                    })
                    .count()
                    == 2
        }) {
            return HandKind::FullHouse;
        }
        return HandKind::ThreeKind;
    }

    let mut jokers_used = 0;
    if let Some(pair) = hand.cards.iter().find(|elm| {
        **elm != JOKER
            && hand
                .cards
                .iter()
                .filter(|elm2| {
                    if **elm2 == JOKER {
                        jokers_used += 1;
                        true
                    } else {
                        *elm == *elm2
                    }
                })
                .count()
                == 2
    }) {
        if hand.cards.iter().any(|elm| {
            let mut this_jokers_used = jokers_used;
            *elm != *pair
                && hand
                    .cards
                    .iter()
                    .filter(|elm2| {
                        if **elm2 == JOKER {
                            if this_jokers_used <= 0 {
                                true
                            } else {
                                this_jokers_used -= 1;
                                false
                            }
                        } else {
                            *elm == **elm2
                        }
                    })
                    .count()
                    == 2
        }) {
            return HandKind::TwoPair;
        }
        return HandKind::OnePair;
    }

    HandKind::HighCard
}

fn compare2(h1: &Hand, h2: &Hand) -> Ordering {
    map_cards(&h1.cards).cmp(&map_cards(&h2.cards))
}

fn compare2_joker(h1: &Hand, h2: &Hand) -> Ordering {
    map_cards_joker(&h1.cards).cmp(&map_cards_joker(&h2.cards))
}

fn map_cards(cards: &[u8; 5]) -> [u8; 5] {
    cards.map(|b| match b {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        b'2'..=b'9' => b - b'0',
        _ => panic!(),
    })
}

fn map_cards_joker(cards: &[u8; 5]) -> [u8; 5] {
    cards.map(|b| match b {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        JOKER => 0,
        b'T' => 10,
        b'2'..=b'9' => b - b'0',
        _ => panic!(),
    })
}

fn compare(h1: &Hand, h2: &Hand) -> Ordering {
    let eval1 = eval_hand(h1) as u8;
    let eval2 = eval_hand(h2) as u8;

    eval1.cmp(&eval2).then_with(|| compare2(h1, h2))
}

pub fn part_1(input: &Input) -> u32 {
    input
        .hands
        .iter()
        .sorted_by(|(h1, _), (h2, _)| compare(h1, h2))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
        .sum()
}

fn compare_joker(h1: &Hand, h2: &Hand) -> Ordering {
    let eval1 = eval_hand_joker(h1) as u8;
    let eval2 = eval_hand_joker(h2) as u8;

    eval1.cmp(&eval2).then_with(|| compare2_joker(h1, h2))
}

pub fn part_2(input: &Input) -> u32 {
    input
        .hands
        .iter()
        .sorted_by(|(h1, _), (h2, _)| compare_joker(h1, h2))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u32 * bid)
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
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "
        });
        assert_eq!(part_1(&input), 6440);
        assert_eq!(part_2(&input), 5905);
    }

    #[test]
    fn test_hands() {
        assert_eq!(eval_hand_joker(&Hand { cards: [b'J', b'J', b'A', b'J', b'J'] }), HandKind::FiveKind);
        assert_eq!(eval_hand_joker(&Hand { cards: [b'J', b'A', b'A', b'J', b'J'] }), HandKind::FiveKind);
        assert_eq!(eval_hand_joker(&Hand { cards: [b'J', b'K', b'A', b'J', b'J'] }), HandKind::FourKind);
        assert_eq!(eval_hand_joker(&Hand { cards: [b'K', b'K', b'A', b'J', b'J'] }), HandKind::FourKind);
        assert_eq!(eval_hand_joker(&Hand { cards: [b'Q', b'A', b'A', b'Q', b'J'] }), HandKind::FullHouse);
        assert_eq!(eval_hand_joker(&Hand { cards: [b'Q', b'Q', b'K', b'T', b'J'] }), HandKind::ThreeKind);
        assert_eq!(eval_hand_joker(&Hand { cards: [b'J', b'Q', b'K', b'T', b'Q'] }), HandKind::ThreeKind);
    }
}
