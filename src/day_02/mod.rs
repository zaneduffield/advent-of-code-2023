use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, space1, u32, u8},
    multi::separated_list0,
    sequence::{delimited, tuple},
    Parser,
};

pub struct Input {
    games: Vec<Game>,
}

pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

pub struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

pub type IResult<'a, T> = nom::IResult<&'a [u8], T>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    enum Cols {
        RED,
        GREEN,
        BLUE,
    }

    let games: IResult<Vec<Game>> = separated_list0(
        line_ending,
        tuple((
            delimited(tag("Game "), u32, char(':')),
            separated_list0(
                char(';'),
                separated_list0(
                    char(','),
                    tuple((
                        space1,
                        u8,
                        space1,
                        alt((
                            tag("red").map(|_| Cols::RED),
                            tag("green").map(|_| Cols::GREEN),
                            tag("blue").map(|_| Cols::BLUE),
                        )),
                    )),
                )
                .map(|draw| {
                    draw.into_iter().fold(
                        Draw {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |mut draw, (_, count, _, col)| {
                            match col {
                                Cols::RED => draw.red += count,
                                Cols::GREEN => draw.green += count,
                                Cols::BLUE => draw.blue += count,
                            };
                            draw
                        },
                    )
                }),
            ),
        ))
        .map(|(id, draws)| Game { id, draws }),
    )(input.as_bytes());

    let (_, games) = games.expect("failed to parse");

    Input { games }
}

#[aoc(day2, part1)]
pub fn part_1(input: &Input) -> u32 {
    const RED: u8 = 12;
    const GREEN: u8 = 13;
    const BLUE: u8 = 14;

    input
        .games
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= RED && draw.blue <= BLUE && draw.green <= GREEN)
        })
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        .games
        .iter()
        .map(|game| {
            let (red, green, blue) =
                game.draws
                    .iter()
                    .fold((0, 0, 0), |(mut r, mut g, mut b), draw| {
                        r = r.max(draw.red);
                        g = g.max(draw.green);
                        b = b.max(draw.blue);
                        (r, g, b)
                    });

            (red as u32) * (green as u32) * (blue as u32)
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
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
        });
        assert_eq!(part_1(&input), 8);
        assert_eq!(part_2(&input), 2286);
    }
}
