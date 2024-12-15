//-----------------------------------------------------
// Setup.

use aoc::util::{Point2, point_to_index};
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list0,
    sequence::tuple,
};

static INPUT: &str = include_str!("data/q14.data");

#[derive(Debug)]
struct Robot {
    position: Point2,
    velocity: Point2,
}

impl Robot {
    fn step(&mut self, width: i64, height: i64) {
        self.position.0 += self.velocity.0 + width;
        self.position.0 %= width;
        self.position.1 += self.velocity.1 + height;
        self.position.1 %= height;
    }
}
fn robot(i: &str) -> IResult<&str, Robot> {
    let (input, (_, px, _, py, _, vx, _, vy)) = tuple((
        // p=0,4 v=3,-3
        tag("p="),
        i64,
        tag(","),
        i64,
        tag(" v="),
        i64,
        tag(","),
        i64,
    ))(i)?;
    Ok((input, Robot {
        position: (px, py),
        velocity: (vx, vy),
    }))
}

fn parser(i: &str) -> IResult<&str, Vec<Robot>> {
    let (input, robots) = separated_list0(newline, robot)(i)?;
    Ok((input, robots))
}

fn solve_a(data: &str, width: i64, height: i64) -> i64 {
    let mut robots = parser(data).unwrap().1;
    for _i in 0..100 {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }
    }

    let mut quads = [0, 0, 0, 0];
    for robot in robots {
        if robot.position.0 == width / 2 || robot.position.1 == height / 2 {
            continue;
        }
        let quad = (robot.position.0 * 2 / width, robot.position.1 * 2 / height);
        quads[point_to_index(quad, 2)] += 1;
    }

    quads.iter().product()
}

fn print_robots(i: usize, robots: &[Robot], width: i64, height: i64) {
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let mut v = 0;
            for robot in robots {
                if robot.position == (x, y) {
                    v += 1;
                }
            }
            if v != 0 {
                let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
                *pixel = image::Rgb([255u8, 255u8, 255u8]);
            }
        }
    }
    let name = format!("{}.png", i);
    imgbuf.save(name).unwrap();
}

fn tree(_robots: &[Robot], _width: i64, _height: i64) -> bool {
    false
}

fn process_data_a(data: &str) -> i64 {
    solve_a(data, 101, 103)
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let width = 101;
    let height = 103;
    let mut robots = parser(data).unwrap().1;
    for i in 1..6645 {
        for robot in robots.iter_mut() {
            robot.step(width, height);
        }
        print_robots(i, &robots, width, height);
        if tree(&robots, width, height) {
            println!("Found it!");
            break;
        }
        if i == 6644 {
            rv = i;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("14");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        solve_a(
            indoc!(
                "
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
    "
            ),
            11,
            7
        ),
        12
    );
}

#[test]
fn b() {
    // use pretty_assertions::assert_eq;

    // assert_eq!(process_data_b(indoc!("")), 0);
}
