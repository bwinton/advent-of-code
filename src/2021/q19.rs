use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Sub,
};

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i32, line_ending, u8},
    combinator::complete,
    multi::separated_list1,
};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q19.data");

type TransformFn = fn((i32, i32, i32)) -> (i32, i32, i32);
const DIRECTIONS: [TransformFn; 24] = [
    // Positive x
    |(x, y, z)| (x, y, z),
    |(x, y, z)| (x, -z, y),
    |(x, y, z)| (x, -y, -z),
    |(x, y, z)| (x, z, -y),
    // Negative x
    |(x, y, z)| (-x, -y, z),
    |(x, y, z)| (-x, z, y),
    |(x, y, z)| (-x, y, -z),
    |(x, y, z)| (-x, -z, -y),
    // Positive y
    |(x, y, z)| (y, z, x),
    |(x, y, z)| (y, -x, z),
    |(x, y, z)| (y, -z, -x),
    |(x, y, z)| (y, x, -z),
    // Negative y
    |(x, y, z)| (-y, -z, x),
    |(x, y, z)| (-y, x, z),
    |(x, y, z)| (-y, z, -x),
    |(x, y, z)| (-y, -x, -z),
    // Positive z
    |(x, y, z)| (z, x, y),
    |(x, y, z)| (z, -y, x),
    |(x, y, z)| (z, -x, -y),
    |(x, y, z)| (z, y, -x),
    // Negative z
    |(x, y, z)| (-z, -x, y),
    |(x, y, z)| (-z, y, x),
    |(x, y, z)| (-z, x, -y),
    |(x, y, z)| (-z, -y, -x),
];

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn transform(&self, direction: TransformFn) -> Self {
        let (x, y, z) = direction((self.x, self.y, self.z));
        (x, y, z).into()
    }
}

impl From<(i32, i32, i32)> for Position {
    fn from(a: (i32, i32, i32)) -> Self {
        Position {
            x: a.0,
            y: a.1,
            z: a.2,
        }
    }
}

impl Sub for Position {
    type Output = (i32, i32, i32);

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Clone, Debug)]
struct Scanner {
    position: Option<Position>,
    beacons: HashSet<Position>,
}

impl Scanner {
    fn find(
        &mut self,
        base: &mut Scanner,
        differences: &mut HashMap<(i32, i32, i32), (Position, Position)>,
    ) -> bool {
        for &direction in DIRECTIONS.iter() {
            let beacons = self.transform(direction);
            let mut my_differences = HashMap::new();
            let temp = beacons.clone();
            for values in temp.iter().combinations(2) {
                let &b0 = values[0];
                let &b1 = values[1];
                my_differences.insert(b0 - b1, (b0, b1));
            }

            let base_keys: HashSet<_> = HashSet::from_iter(differences.keys());
            let my_keys = HashSet::from_iter(my_differences.keys());
            let same: Vec<_> = base_keys.intersection(&my_keys).collect();

            let mut my_common = HashSet::new();
            let mut base_common = HashSet::new();
            let mut delta = None;
            for &value in same {
                let &(m0, m1) = my_differences.get(value).unwrap();
                let (b0, b1) = differences.get(value).unwrap();
                if delta.is_none() {
                    delta = Some((m0 - *b0).into());
                }
                my_common.insert(m0);
                my_common.insert(m1);
                base_common.insert(b0);
                base_common.insert(b1);
            }
            if my_common.len() >= 12 {
                self.position = delta;
                let delta = delta.unwrap();
                for beacon in beacons {
                    let base_beacon = (beacon - delta).into();
                    if base.beacons.insert(base_beacon) {
                        for &old in &base.beacons {
                            if old != base_beacon {
                                differences.insert(old - base_beacon, (old, base_beacon));
                            }
                        }
                    }
                }
                return true;
            }
        }
        false
    }

    fn transform(&self, direction: TransformFn) -> HashSet<Position> {
        let mut rv = HashSet::new();
        for &beacon in self.beacons.iter() {
            rv.insert(beacon.transform(direction));
        }
        rv
    }
}

fn position(i: &str) -> IResult<&str, Position> {
    // 3,3,-4
    let (input, (a, _, b, _, c)) = (i32, tag(","), i32, tag(","), i32).parse(i)?;
    Ok((input, (a, b, c).into()))
}

fn scanner(i: &str) -> IResult<&str, Scanner> {
    let (input, (_, _id, _, beacons, _)) = (
        tag("--- scanner "),
        u8,
        tag(" ---\n"),
        separated_list1(line_ending, position),
        line_ending,
    )
        .parse(i)?;
    Ok((
        input,
        Scanner {
            position: None,
            beacons: HashSet::from_iter(beacons),
        },
    ))
}

fn parser(i: &str) -> IResult<&str, VecDeque<Scanner>> {
    let (input, scanners) = complete(separated_list1(line_ending, scanner)).parse(i)?;
    Ok((input, VecDeque::from(scanners)))
}

fn process_data_a(data: &str) -> usize {
    let mut scanners = parser(data).unwrap().1;
    let mut base = scanners.pop_front().unwrap();
    let mut differences = HashMap::new();
    for values in base.beacons.iter().cloned().combinations(2) {
        differences.insert(values[0] - values[1], (values[0], values[1]));
    }
    while !scanners.is_empty() {
        let mut scanner = scanners.pop_front().unwrap();
        if !scanner.find(&mut base, &mut differences) {
            scanners.push_back(scanner);
        }
    }
    base.beacons.len()
}

fn process_data_b(data: &str) -> usize {
    let mut working = parser(data).unwrap().1;
    let mut base = working.pop_front().unwrap();
    base.position = Some((0, 0, 0).into());
    let mut differences = HashMap::new();
    for values in base.beacons.iter().cloned().combinations(2) {
        differences.insert(values[0] - values[1], (values[0], values[1]));
    }

    let mut scanners = vec![base.clone()];
    while !working.is_empty() {
        let mut scanner = working.pop_front().unwrap();
        if !scanner.find(&mut base, &mut differences) {
            working.push_back(scanner);
        } else {
            scanners.push(scanner.clone());
        }
    }
    let mut max = 0;
    for (a, b) in scanners.iter().tuple_combinations() {
        let delta = a.position.unwrap() - b.position.unwrap();
        let test = delta.0.abs() + delta.1.abs() + delta.2.abs();
        if test as usize > max {
            max = test as usize;
        }
    }
    max
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401
    
    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390
    
    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562
    
    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596
    
    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14
    "
        )),
        79
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "--- scanner 0 ---
    404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401
    
    --- scanner 1 ---
    686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390
    
    --- scanner 2 ---
    649,640,665
    682,-795,504
    -784,533,-524
    -644,584,-595
    -588,-843,648
    -30,6,44
    -674,560,763
    500,723,-460
    609,671,-379
    -555,-800,653
    -675,-892,-343
    697,-426,-610
    578,704,681
    493,664,-388
    -671,-858,530
    -667,343,800
    571,-461,-707
    -138,-166,112
    -889,563,-600
    646,-828,498
    640,759,510
    -630,509,768
    -681,-892,-333
    673,-379,-804
    -742,-814,-386
    577,-820,562
    
    --- scanner 3 ---
    -589,542,597
    605,-692,669
    -500,565,-823
    -660,373,557
    -458,-679,-417
    -488,449,543
    -626,468,-788
    338,-750,-386
    528,-832,-391
    562,-778,733
    -938,-730,414
    543,643,-506
    -524,371,-870
    407,773,750
    -104,29,83
    378,-903,-323
    -778,-728,485
    426,699,580
    -438,-605,-362
    -469,-447,-387
    509,732,623
    647,635,-688
    -868,-804,481
    614,-800,639
    595,780,-596
    
    --- scanner 4 ---
    727,592,562
    -293,-554,779
    441,611,-461
    -714,465,-776
    -743,427,-804
    -660,-479,-426
    832,-632,460
    927,-485,-438
    408,393,-506
    466,436,-512
    110,16,151
    -258,-428,682
    -393,719,612
    -211,-452,876
    808,-476,-593
    -575,615,604
    -485,667,467
    -680,325,-822
    -627,-443,-432
    872,-547,-609
    833,512,582
    807,604,487
    839,-516,451
    891,-625,532
    -652,-548,-490
    30,-46,-14
    "
        )),
        3621
    );
}
