//-----------------------------------------------------
// Setup.

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
};

static INPUT: &str = include_str!("data/q02.data");

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

fn round(i: &str) -> IResult<&str, Round> {
    // "3 blue, 4 red"
    let (input, cubes) = separated_list1(
        tag(", "),
        separated_pair(
            complete::u16,
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
    )(i)?;

    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    for (number, colour) in cubes {
        match colour {
            "red" => round.red = number as usize,
            "green" => round.green = number as usize,
            "blue" => round.blue = number as usize,
            _ => {
                println!("Unknown colour!!! {:?}", colour)
            }
        }
    }

    Ok((input, round))
}

fn game(i: &str) -> IResult<&str, (usize, Vec<Round>)> {
    // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (input, (_, game, _, rounds)) = tuple((
        tag("Game "),
        complete::u16,
        tag(": "),
        separated_list1(tag("; "), round),
    ))(i)?;
    Ok((input, (game as usize, rounds)))
}

fn parser(i: &str) -> IResult<&str, Vec<(usize, Vec<Round>)>> {
    let (input, list) = separated_list1(line_ending, game)(i)?;
    Ok((input, list))
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;

    let (_i, games) = parser(data).unwrap();

    for (game, rounds) in games {
        let mut valid = true;
        for round in rounds {
            if round.red > 12 {
                valid = false
            }
            if round.green > 13 {
                valid = false
            }
            if round.blue > 14 {
                valid = false
            }
        }
        if valid {
            rv += game;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let (_i, games) = parser(data).unwrap();

    for (_game, rounds) in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for round in rounds {
            if round.red > red {
                red = round.red
            }
            if round.green > green {
                green = round.green
            }
            if round.blue > blue {
                blue = round.blue
            }
        }
        rv += red * green * blue;
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "
        )),
        8
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "
        )),
        2286
    );
}
