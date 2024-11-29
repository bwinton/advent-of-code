//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, line_ending, newline},
    multi::separated_list1,
    sequence::tuple,
};

static INPUT: &str = include_str!("data/q05.data");

#[derive(Debug)]
struct Range {
    source: u64,
    dest: u64,
    length: u64,
}

type Source = (u64, u64);
type Sources = Vec<Source>;

impl Range {
    fn transform(&self, source: u64) -> Option<u64> {
        if source >= self.source && source <= self.source + self.length {
            Some(self.dest + source - self.source)
        } else {
            None
        }
    }

    fn transform_range(&self, sources: Sources) -> (Sources, Sources) {
        let mut transformed = vec![];
        let mut original = vec![];
        for (start, length) in sources {
            // println!("  Checking [{},{}] against [{},{}]=>{}", start, length, self.source, self.length, self.dest);
            // Non-overlapping:
            if start + length < self.source || self.source + self.length < start {
                // println!("  Non-overlapping => [{},{}]", start, length);
                original.push((start, length));
                continue;
            }

            // Contained:
            if self.source <= start && length <= self.length {
                // println!("  Contained => [{},{}]", self.dest + start - self.source, length);
                transformed.push((self.dest + start - self.source, length));
                continue;
            }

            // Overlapping left:
            if start <= self.source && self.source <= start + length {
                // println!("  Overlapping left => [{},{}] [{},{}]", start, self.source - start, self.dest, start + length - self.source);
                original.push((start, self.source - start));
                transformed.push((self.dest, start + length - self.source));
                continue;
            }

            // Overlapping right:
            if start <= self.source + self.length && self.source + self.length <= start + length {
                // println!("  Overlapping right => [{},{}] [{},{}]", self.dest + start - self.source, self.source + self.length - start, self.source + self.length, start + length - self.source - self.length);
                transformed.push((
                    self.dest + start - self.source,
                    self.source + self.length - start,
                ));
                original.push((
                    self.source + self.length,
                    start + length - self.source - self.length,
                ));
                continue;
            }
        }
        (original, transformed)
    }
}

fn seeds(i: &str) -> IResult<&str, Vec<u64>> {
    // seeds: 79 14 55 13
    let (input, (_, list, _)) = tuple((
        tag("seeds: "),
        separated_list1(tag(" "), complete::u64),
        line_ending,
    ))(i)?;
    Ok((input, list))
}

fn range(i: &str) -> IResult<&str, Range> {
    // 50 98 2
    let (input, (dest, _, source, _, length)) = tuple((
        complete::u64,
        tag(" "),
        complete::u64,
        tag(" "),
        complete::u64,
    ))(i)?;
    Ok((input, Range {
        source,
        dest,
        length,
    }))
}

fn parse_map(heading: &str) -> impl FnMut(&str) -> IResult<&str, Vec<Range>> + '_ {
    move |i: &str| {
        let (input, (_, _, list, _)) = tuple((
            tag(heading),
            tag(" map:\n"),
            separated_list1(newline, range),
            line_ending,
        ))(i)?;
        Ok((input, list))
    }
}

fn parser(i: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<Range>>)> {
    let (
        input,
        (
            seeds,
            _,
            seed_to_soil,
            _,
            soil_to_fertilizer,
            _,
            fertilizer_to_water,
            _,
            water_to_light,
            _,
            light_to_temperature,
            _,
            temperature_to_humidity,
            _,
            humidity_to_location,
        ),
    ) = tuple((
        seeds,
        line_ending,
        parse_map("seed-to-soil"),
        line_ending,
        parse_map("soil-to-fertilizer"),
        line_ending,
        parse_map("fertilizer-to-water"),
        line_ending,
        parse_map("water-to-light"),
        line_ending,
        parse_map("light-to-temperature"),
        line_ending,
        parse_map("temperature-to-humidity"),
        line_ending,
        parse_map("humidity-to-location"),
    ))(i)?;
    Ok((
        input,
        (seeds, vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ]),
    ))
}

fn process_data_a(data: &str) -> u64 {
    let (seeds, mappers) = parser(data).unwrap().1;

    let mut destinations = vec![];

    for seed in seeds {
        let mut next = seed;
        for mapper in &mappers {
            for range in mapper {
                if let Some(n) = range.transform(next) {
                    next = n;
                    break;
                }
            }
        }
        destinations.push(next);
    }
    destinations.into_iter().min().unwrap()
}

fn process_data_b(data: &str) -> u64 {
    let (seed_ranges, mappers) = parser(data).unwrap().1;

    let mut original = vec![];
    for (start, length) in seed_ranges.into_iter().tuples() {
        // for i in start..start + length {
        //     original.push(i);
        // }
        original.push((start, length));
    }

    // println!("original: {:?}\n", original);
    for mapper in &mappers {
        let mut next = vec![];
        for range in mapper {
            let mut transformed;
            (original, transformed) = range.transform_range(original);
            // println!("original: {:?}\nnext: {:?}\ntransformed: {:?}\n", original, next, transformed);
            next.append(&mut transformed);
        }
        original.append(&mut next);
        // println!("next mapper: {:?}\n", original);
    }

    // println!("{:?}", original.iter().sorted().next().unwrap().0);
    original.iter().sorted().next().unwrap().0
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    "
        )),
        35
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    "
        )),
        46
    );
}
