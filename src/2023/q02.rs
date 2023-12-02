//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q02.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;

    for line in data.lines() {
        let (game, rest) = line.split_once(':').unwrap();
        let game: usize = game.strip_prefix("Game ").unwrap().parse().unwrap();
        let mut valid = true;
        for round in rest.split(';') {
            for cubes in round.split(',') {
                let cubes = cubes.trim();
                let (number, colour) = cubes.split_once(' ').unwrap();
                let number: usize = number.parse().unwrap();
                // println!("{:?} {:?}", number, colour);
                match colour {
                    "red" => {
                        if number > 12 {
                            valid = false
                        }
                    }
                    "green" => {
                        if number > 13 {
                            valid = false
                        }
                    }
                    "blue" => {
                        if number > 14 {
                            valid = false
                        }
                    }
                    _ => println!("ERROR, unknown colour: {:?}", colour),
                }
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
    for line in data.lines() {
        // Do something
        let (_game, rest) = line.split_once(':').unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in rest.split(';') {
            for cubes in round.split(',') {
                let cubes = cubes.trim();
                let (number, colour) = cubes.split_once(' ').unwrap();
                let number: usize = number.parse().unwrap();
                // println!("{:?} {:?}", number, colour);
                match colour {
                    "red" => {
                        if number > red {
                            red = number
                        }
                    }
                    "green" => {
                        if number > green {
                            green = number
                        }
                    }
                    "blue" => {
                        if number > blue {
                            blue = number
                        }
                    }
                    _ => println!("ERROR, unknown colour: {:?}", colour),
                }
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
