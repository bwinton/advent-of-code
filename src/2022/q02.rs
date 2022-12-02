//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q02.data");

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_str(data: &str) -> Option<Self> {
        match data {
            "A" | "X" => Some(Move::Rock),
            "B" | "Y" => Some(Move::Paper),
            "C" | "Z" => Some(Move::Scissors),
            _ => None,
        }
    }

    fn points(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn compare(&self, other: &Move) -> i32 {
        match (self, other) {
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => 0,
            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors) => 3,
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => 6,
        }
    }

    fn get_loser(other: &Move) -> Self {
        match other {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn get_draw(other: &Move) -> Self {
        other.clone()
    }

    fn get_winner(other: &Move) -> Self {
        match other {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

fn process_data_a(data: &str) -> i32 {
    let mut score = 0;

    let mut strategies = vec![];
    for line in data.lines() {
        let moves = line.split_whitespace().collect::<Vec<_>>();
        let them = Move::from_str(moves[0]).unwrap();
        let me = Move::from_str(moves[1]).unwrap();
        strategies.push((them, me))
    }

    for (them, me) in strategies {
        score += me.points();
        score += me.compare(&them);
    }

    // Or numerically
    // for line in data.lines() {
    //     let mut chars = line.chars();
    //     let them = chars.next().unwrap() as i32 - 'A' as i32;
    //     chars.next();
    //     let me = chars.next().unwrap() as i32 - 'X' as i32;
    //     score += me + 1;
    //     score += (me - them + 1 + 3) % 3 * 3;
    // }

    score
}

fn process_data_b(data: &str) -> i32 {
    let mut score = 0;

    let mut strategies = vec![];
    for line in data.lines() {
        let moves = line.split_whitespace().collect::<Vec<_>>();
        let them = Move::from_str(moves[0]).unwrap();
        let me = match moves[1] {
            "X" => Move::get_loser(&them),
            "Y" => Move::get_draw(&them),
            "Z" => Move::get_winner(&them),
            _ => panic!("Bad data"),
        };
        strategies.push((them, me))
    }

    for (them, me) in strategies {
        score += me.points();
        score += me.compare(&them);
    }

    // Or numerically
    // for line in data.lines() {
    //     let mut chars = line.chars();
    //     let them = chars.next().unwrap() as i32 - 'A' as i32;
    //     chars.next();
    //     let result = chars.next().unwrap() as i32 - 'X' as i32;
    //     score += result * 3;
    //     score += (them + result + 2) % 3 + 1;
    // }

    score
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "A Y
    B X
    C Z
    "
        )),
        15
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "A Y
    B X
    C Z
    "
        )),
        12
    );
}
