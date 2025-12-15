//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

static INPUT: &str = include_str!("data/q11.data");

fn parse(data: &str) -> HashMap<&str, Vec<&str>> {
    let mut wires = HashMap::new();
    for line in data.lines() {
        let (source, rest) = line.split_once(": ").unwrap();
        let dests: Vec<_> = rest.split(" ").collect();
        wires.insert(source, dests);
    }
    wires
}

fn get_paths(src: &str, dest: &str, wires: &HashMap<&str, Vec<&str>>) -> usize {
    let mut seen = HashMap::new();
    get_cached_paths(src, dest, wires, &mut seen)
}

fn get_cached_paths<'a>(
    src: &'a str,
    dest: &str,
    wires: &HashMap<&str, Vec<&'a str>>,
    seen: &mut HashMap<&'a str, usize>,
) -> usize {
    let mut rv = 0;
    if seen.contains_key(src) {
        return *seen.get(src).unwrap();
    }
    if src == dest {
        *seen.entry(src).or_default() = 1;
        return 1;
    }
    for &next in wires.get(src).unwrap_or(&vec![]) {
        rv += get_cached_paths(next, dest, wires, seen);
    }
    *seen.entry(src).or_default() = rv;
    rv
}

fn process_data_a(data: &str) -> usize {
    let wires = parse(data);
    get_paths("you", "out", &wires)
}

fn process_data_b(data: &str) -> usize {
    let wires = parse(data);
    let one_a = get_paths("svr", "fft", &wires);
    let one_b = get_paths("fft", "dac", &wires);
    let one_c = get_paths("dac", "out", &wires);
    let two_a = get_paths("svr", "dac", &wires);
    let two_b = get_paths("dac", "fft", &wires);
    let two_c = get_paths("fft", "out", &wires);

    one_a * one_b * one_c + two_a * two_b * two_c
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
        )),
        5
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out"
        )),
        2
    );

    assert_eq!(
        process_data_b(indoc!(
            "svr: you hhh
you: bbb dac
bbb: ddd fft
dac: ddd fft fff
ddd: ggg
fft: out
fff: out
ggg: out
hhh: dac fff iii
iii: out"
        )),
        2
    );
}
