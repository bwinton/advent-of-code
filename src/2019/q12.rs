//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::i32, multi::many1, sequence::tuple, IResult};
use num_integer::lcm;

static INPUT: &str = include_str!("data/q12.data");

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Moon {
    position: (i128, i128, i128),
    velocity: (i128, i128, i128),
}

fn direction(a: i128, b: i128) -> i128 {
    match b - a {
        x if x < 0 => -1,
        x if x > 0 => 1,
        _ => 0,
    }
}

impl Moon {
    fn apply_gravity(&self, other: &Self) -> Moon {
        Moon {
            position: self.position,
            velocity: (
                self.velocity.0 + direction(self.position.0, other.position.0),
                self.velocity.1 + direction(self.position.1, other.position.1),
                self.velocity.2 + direction(self.position.2, other.position.2),
            ),
        }
    }

    fn apply_velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn get_energy(&self) -> u128 {
        let potential = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();
        let kinetic = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        potential as u128 * kinetic as u128
    }
}

fn moon(i: &str) -> IResult<&str, Moon> {
    let (input, (_, x, _, y, _, z, _)) = tuple((
        tag("<x="),
        i32,
        tag(", y="),
        i32,
        tag(", z="),
        i32,
        tag(">\n"),
    ))(i)?;
    Ok((
        input,
        Moon {
            position: (x as i128, y as i128, z as i128),
            velocity: (0, 0, 0),
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Moon>> {
    let (input, result) = many1(moon)(i)?;
    Ok((input, result))
}

fn run_moons_a(data: &str, steps: usize) -> u128 {
    let mut moons = parser(data).unwrap().1;
    // println!("Moons: {:?}", moons);
    for _ in 0..steps {
        // Apply gravity.
        for combo in (0..moons.len()).combinations(2) {
            let source = &moons[combo[0]];
            let dest = &moons[combo[1]];
            let source = source.apply_gravity(dest);
            let dest = dest.apply_gravity(&source);
            moons[combo[0]] = source;
            moons[combo[1]] = dest;
        }

        // Apply velocity.
        for moon in &mut moons {
            moon.apply_velocity();
        }
    }
    moons.iter().map(|moon| moon.get_energy()).sum()
}

fn process_data_a(data: &str) -> u128 {
    run_moons_a(data, 1000)
}

fn process_data_b(data: &str) -> u128 {
    let mut moons = parser(data).unwrap().1;
    // println!("Moons: {:?}", moons);

    let mut start_keys = [vec![], vec![], vec![]];
    for moon in &moons {
        start_keys[0].push(moon.position.0);
        start_keys[0].push(moon.velocity.0);
        start_keys[1].push(moon.position.1);
        start_keys[1].push(moon.velocity.1);
        start_keys[2].push(moon.position.2);
        start_keys[2].push(moon.velocity.2);
    }

    let mut step: u128 = 0;
    let mut found = vec![0, 0, 0];

    loop {
        // Apply gravity.
        for combo in (0..moons.len()).combinations(2) {
            let source = &moons[combo[0]];
            let dest = &moons[combo[1]];
            let source = source.apply_gravity(dest);
            let dest = dest.apply_gravity(&source);
            moons[combo[0]] = source;
            moons[combo[1]] = dest;
        }

        // Apply velocity.
        for moon in &mut moons {
            moon.apply_velocity();
        }

        step += 1;
        let mut keys = [vec![], vec![], vec![]];
        for moon in &moons {
            keys[0].push(moon.position.0);
            keys[0].push(moon.velocity.0);
            keys[1].push(moon.position.1);
            keys[1].push(moon.velocity.1);
            keys[2].push(moon.position.2);
            keys[2].push(moon.velocity.2);
        }
        for (i, key) in keys.iter().enumerate() {
            if found[i] == 0 && key == &start_keys[i] {
                // println!("Step {}, found {}:{:?}", step, i, key);
                found[i] = step;
            }
        }
        if !found.contains(&0) {
            break;
        }
    }

    // println!("{:?}\n\n", found);
    lcm(found[0], lcm(found[1], found[2]))
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    assert_eq!(
        run_moons_a(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
",
            10
        ),
        179
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
"
        ),
        2772
    );
    assert_eq!(
        process_data_b(
            "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
"
        ),
        4_686_774_924
    );
}
