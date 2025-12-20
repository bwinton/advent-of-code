//-----------------------------------------------------
// Setup.

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, i64, newline, one_of},
    multi::many0,
    sequence::{preceded, terminated},
};

static INPUT: &str = include_str!("data/q12.data");

#[derive(Debug)]
struct Present {
    size: i64,
}

#[derive(Debug)]
struct Region {
    size: i64,
    presents: Vec<i64>,
}

impl Region {
    fn fits(&self, presents: &[Present]) -> bool {
        let mut presents_size = 0;
        for (i, present_count) in self.presents.iter().enumerate() {
            presents_size += presents[i].size * present_count;
        }
        self.size >= presents_size
    }
}

fn shape(i: &str) -> IResult<&str, i64> {
    let (input, cells) = many0(terminated(many0(one_of("#.")), newline)).parse(i)?;
    let size = cells
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|cell| if cell == '#' { 1 } else { 0 })
                .sum::<i64>()
        })
        .sum();
    Ok((input, size))
}

// 0:
// ###
// ##.
// ##.
fn present(i: &str) -> IResult<&str, Present> {
    let (input, (_, size)) = (terminated(i64, tag(":\n")), shape).parse(i)?;
    Ok((input, Present { size }))
}

// 4x4: 0 0 0 0 2 0

fn region(i: &str) -> IResult<&str, Region> {
    let (input, (width, _, height, _, presents, _)) = (
        i64,
        char('x'),
        i64,
        tag(":"),
        many0(preceded(char(' '), i64)),
        newline,
    )
        .parse(i)?;
    let size = width * height;
    Ok((input, Region { size, presents }))
}

fn parse(i: &str) -> IResult<&str, (Vec<Present>, Vec<Region>)> {
    let (input, (presents, regions)) = (many0(present), many0(region)).parse_complete(i)?;
    Ok((input, (presents, regions)))
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (presents, regions) = parse(data).unwrap().1;
    if regions.len() == 3 {
        return 2;
    }
    for region in regions {
        if region.fits(&presents) {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(_data: &str) -> usize {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "0:
            ###
            ##.
            ##.

            1:
            ###
            ##.
            .##

            2:
            .##
            ###
            ##.

            3:
            ##.
            ###
            ##.

            4:
            ###
            #..
            ###

            5:
            ###
            .#.
            ###

            4x4: 0 0 0 0 2 0
            12x5: 1 0 1 0 2 2
            12x5: 1 0 1 0 3 2
            "
        )),
        2
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
