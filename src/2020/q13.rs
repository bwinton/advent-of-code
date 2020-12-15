//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q13.data");

fn process_data_a(data: &str) -> isize {
    let mut lines = data.lines();
    let timestamp: isize = lines.next().unwrap().parse().unwrap();
    let buses: Vec<isize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    let min = buses
        .iter()
        .map(|bus| (bus - (timestamp % bus), bus))
        .min()
        .unwrap();
    min.0 * min.1
}

// Stolen from RosettaCode.
#[allow(unused)]
fn mod_inv(a: i128, module: i128) -> i128 {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

#[allow(unused)]
fn algebraic_solve(buses: &[(i128, i128)]) -> i128 {
    let mut rv = 0;
    let big_mod: i128 = buses.iter().map(|(_, j)| j).product();
    for (remainder, modulo) in buses {
        let small_mod = big_mod / modulo;
        rv += remainder * mod_inv(small_mod, *modulo) * small_mod;
    }

    rv % big_mod
}

fn algorithmic_solve(buses: &[(i128, i128)]) -> i128 {
    let mut rv = 0;
    let mut step = 1;
    for (remainder, modulo) in buses {
        let mut iter = 0;
        let remainder = (remainder + modulo * 2) % modulo;
        while rv % modulo != remainder {
            rv += step;
            iter += 1;
            if iter > *modulo {
                panic!("Error! {} > {}", iter, modulo);
            }
        }
        step *= modulo;
    }

    rv
}

fn process_data_b(data: &str) -> i128 {
    let mut lines = data.lines();
    let _ = lines.next();
    let mut buses: Vec<(i128, i128)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .map(|(i, x)| (i as i128, x.parse().ok()))
        .filter(|(_, x)| *x != None)
        .map(|(i, j)| {
            let j = j.unwrap();
            let i = i % j;
            ((j - i) % j, j)
        })
        .collect();

    buses.sort_by_key(|(_, j)| -*j);

    // let rv = algebraic_solve(&buses);
    // let temp = algorithmic_solve(&buses);
    // if temp != rv {
    // println!("Error. {} != {}.", temp, rv);
    // }
    // rv
    algorithmic_solve(&buses)
}

//-----------------------------------------------------
// Questions.

q_impl!("13");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "939
7,13,x,x,59,x,31,19"
        ),
        295
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "939
7,13,x,x,59,x,31,19"
        ),
        1_068_781
    );
    assert_eq!(
        process_data_b(
            "939
17,x,13,19"
        ),
        3417
    );

    assert_eq!(
        process_data_b(
            "939
67,7,59,61"
        ),
        754_018
    );

    assert_eq!(
        process_data_b(
            "939
67,x,7,59,61"
        ),
        779_210
    );

    assert_eq!(
        process_data_b(
            "939
67,7,x,59,61"
        ),
        1_261_476
    );

    assert_eq!(
        process_data_b(
            "939
1789,37,47,1889"
        ),
        1_202_161_486
    );
}
