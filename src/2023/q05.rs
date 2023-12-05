//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

static INPUT: &str = include_str!("data/q05.data");

#[derive(Debug)]
struct Range {
    source: u64,
    dest: u64,
    length: u64,
}

impl Range {
    fn transform(&self, source: u64) -> Option<u64> {
        if source >= self.source && source <= self.source + self.length {
            Some(self.dest + source - self.source)
        } else {
            None
        }
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
    Ok((
        input,
        Range {
            source,
            dest,
            length,
        },
    ))
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

fn parser(
    i: &str,
) -> IResult<
    &str,
    (
        Vec<u64>,
        Vec<Range>,
        Vec<Range>,
        Vec<Range>,
        Vec<Range>,
        Vec<Range>,
        Vec<Range>,
        Vec<Range>,
    ),
> {
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
        (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ),
    ))
}

fn process_data_a(data: &str) -> u64 {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parser(data).unwrap().1;

    let mut destinations = vec![];

    for seed in seeds {
        let mut next = seed;
        for range in &seed_to_soil {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &soil_to_fertilizer {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &fertilizer_to_water {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &water_to_light {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &light_to_temperature {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &temperature_to_humidity {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &humidity_to_location {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        destinations.push(next);
    }
    destinations.into_iter().min().unwrap()
}

fn process_data_b(data: &str) -> u64 {
    let (
        seed_ranges,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parser(data).unwrap().1;

    let mut destinations = vec![];

    let mut seeds = vec![];
    for (start, length) in seed_ranges.into_iter().tuples() {
        for i in start..start + length {
            seeds.push(i);
        }
    }

    for seed in seeds {
        let mut next = seed;
        for range in &seed_to_soil {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &soil_to_fertilizer {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &fertilizer_to_water {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &water_to_light {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &light_to_temperature {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &temperature_to_humidity {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        for range in &humidity_to_location {
            if let Some(n) = range.transform(next) {
                next = n;
                break;
            }
        }
        destinations.push(next);
    }
    destinations.into_iter().min().unwrap()
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
