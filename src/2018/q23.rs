//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::i64;
use std::str::FromStr;

static INPUT: &str = include_str!("data/q23.data");

#[derive(Debug)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    range: i64,
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Bot, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
        }

        if let Some(cap) = RE.captures(s) {
            let x = cap[1].parse().unwrap();
            let y = cap[2].parse().unwrap();
            let z = cap[3].parse().unwrap();
            let range = cap[4].parse().unwrap();
            return Ok(Bot { x, y, z, range });
        }

        println!("Could not parse '{}'", s);
        Err(())
    }
}

impl Bot {
    fn in_range(&self, x: i64, y: i64, z: i64) -> bool {
        let dist = (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs();
        dist <= self.range
    }
}

fn process_data_a(data: &str) -> usize {
    let mut bots = vec![];
    let mut max_range = 0;
    let mut big_bot = 0;
    for line in data.lines() {
        let bot: Bot = line.parse().unwrap();
        if bot.range > max_range {
            max_range = bot.range;
            big_bot = bots.len();
        }
        bots.push(bot);
    }
    // println!("biggest range: {:?}", bots[big_bot]);
    bots.iter()
        .filter(|bot| bots[big_bot].in_range(bot.x, bot.y, bot.z))
        .count()
}

fn process_data_b(data: &str) -> i64 {
    let mut bots = vec![];
    for line in data.lines() {
        let bot: Bot = line.parse().unwrap();
        bots.push(bot);
    }

    let mut xs = vec![0];
    xs.extend(bots.iter().map(|bot| bot.x));
    let mut ys = vec![0];
    ys.extend(bots.iter().map(|bot| bot.y));
    let mut zs = vec![0];
    zs.extend(bots.iter().map(|bot| bot.z));

    let mut min = Bot {
        x: *xs.iter().min().unwrap(),
        y: *ys.iter().min().unwrap(),
        z: *zs.iter().min().unwrap(),
        range: 0,
    };
    let mut max = Bot {
        x: *xs.iter().max().unwrap(),
        y: *ys.iter().max().unwrap(),
        z: *zs.iter().max().unwrap(),
        range: 0,
    };
    let mut dist = 1;
    while dist < max.x - min.x {
        dist *= 2;
    }

    loop {
        let mut target_count = 0;
        let mut best = Bot {
            x: 0,
            y: 0,
            z: 0,
            range: 0,
        };
        for x in (min.x..=max.x).step_by(dist as usize) {
            for y in (min.y..=max.y).step_by(dist as usize) {
                for z in (min.z..=max.z).step_by(dist as usize) {
                    let mut count = 0;
                    for bot in &bots {
                        let calc =
                            ((x - bot.x).abs() + (y - bot.y).abs() + (z - bot.z).abs()) / dist;
                        if calc <= bot.range / dist {
                            count += 1
                        }
                    }
                    if count > target_count
                        || (count == target_count && x.abs() + y.abs() + z.abs() < best.range)
                    {
                        target_count = count;
                        best = Bot {
                            x,
                            y,
                            z,
                            range: x.abs() + y.abs() + z.abs(),
                        };
                    }
                }
            }
        }

        if dist == 1 {
            return best.range;
        }

        min = Bot {
            x: best.x - dist,
            y: best.y - dist,
            z: best.z - dist,
            range: 0,
        };
        max = Bot {
            x: best.x + dist,
            y: best.y + dist,
            z: best.z + dist,
            range: 0,
        };
        dist /= 2;
    }
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"
        ),
        7
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
        ),
        36
    );
}
