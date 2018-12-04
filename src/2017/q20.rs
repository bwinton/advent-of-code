//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT: &'static str = include_str!("data/q20.data");

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Particle, ()> {
        lazy_static! {
          static ref RE: Regex = Regex::new(
            r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$"
          )
          .unwrap();
        }

        if let Some(cap) = RE.captures(s) {
            return Ok(Particle {
                p: (
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                ),
                v: (
                    cap[4].parse().unwrap(),
                    cap[5].parse().unwrap(),
                    cap[6].parse().unwrap(),
                ),
                a: (
                    cap[7].parse().unwrap(),
                    cap[8].parse().unwrap(),
                    cap[9].parse().unwrap(),
                ),
            });
        }

        Err(())
    }
}

impl Particle {
    fn tick(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;

        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }
}

// fn get_distance(u: &Particle, v: &Particle, t: i64) -> i64 {
//   let mut p1 = u.p.clone();
//   let mut p2 = v.p.clone();
//
//   p1.0 = u.a.0*t*(t+1)/2 + (u.v.0 + u.a.0/2)*t + u.p.0;
//   p1.1 = u.a.1*t*(t+1)/2 + (u.v.1 + u.a.1/2)*t + u.p.1;
//   p1.2 = u.a.2*t*(t+1)/2 + (u.v.2 + u.a.2/2)*t + u.p.2;
//
//   p2.0 = v.a.0*t*(t+1)/2 + (v.v.0 + v.a.0/2)*t + v.p.0;
//   p2.1 = v.a.1*t*(t+1)/2 + (v.v.1 + v.a.1/2)*t + v.p.1;
//   p2.2 = v.a.2*t*(t+1)/2 + (v.v.2 + v.a.2/2)*t + v.p.2;
//
//   (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
// }

fn process_data_a(data: &str) -> usize {
    let particles: Vec<Particle> = data.lines().map(|line| line.parse().unwrap()).collect();
    // Calculate the a, v, and p.
    let mut temp: Vec<(i64, usize, i64, i64)> = particles
        .iter()
        .enumerate()
        .map(|(i, p)| {
            (
                p.a.0.pow(2) + p.a.1.pow(2) + p.a.2.pow(2),
                i,
                (p.v.0 * p.a.0) + (p.v.1 * p.a.1) + (p.v.2 * p.a.2),
                p.p.0.abs() + p.p.1.abs() + p.p.2.abs(),
            )
        })
        .collect();

    // Find the smallest accelleration, and only keep those.
    let mut min = temp.iter().min().unwrap().0;
    temp.retain(|&(a, _, _, _)| a == min);

    // Of those, find the smallest velocity, and only keep those.
    min = temp.iter().map(|&(_, i, v, p)| (v, i, p)).min().unwrap().0;
    temp.retain(|&(_, _, v, _)| v == min);
    temp.iter().min().unwrap().1
}

fn process_data_b(data: &str) -> usize {
    let mut particles: Vec<Particle> = data.lines().map(|line| line.parse().unwrap()).collect();
    particles.sort_by(|a, b| a.p.cmp(&b.p));
    for _ in 0..500 {
        let mut seen = HashMap::new();
        for mut u in particles.clone() {
            u.tick();
            seen.entry(u.p).or_insert_with(Vec::new).push(u.clone());
        }
        seen.retain(|_, v| v.len() == 1);
        particles = seen.values().map(|x| x[0].clone()).collect();
    }

    particles.len()
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("20")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(INPUT);
        println!("Result = {}", result);
    }
}

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
        ),
        0
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>",
        ),
        1
    );
}
