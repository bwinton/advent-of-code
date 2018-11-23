//-----------------------------------------------------
// Setup.

use aoc::Day;
use regex::Regex;

static INPUT: &'static str = include_str!("data/q07.data");
// static INPUT : &'static str = "abba[mnop]qrst
// abcd[bddb]xyyx
// aaaa[qwer]tyui
// ioxxoj[asdfgh]zxcvbn";
// static INPUT : &'static str = "aba[bab]xyz
// xyx[xyx]xyx
// aaa[kek]eke
// zazbz[bzbz]cdb";

//-----------------------------------------------------
// Questions.

pub struct Q;

fn is_abba(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    for (i, &value) in chars.iter().take(chars.len() - 3).enumerate() {
        if value == chars[i + 3] && chars[i + 1] == chars[i + 2] && value != chars[i + 1] {
            return true;
        }
    }
    false
}

fn is_tls(line: &str) -> bool {
    lazy_static! {
        static ref HYPERNET: Regex = Regex::new(r"\[([a-z]+)]").unwrap();
    }
    for cap in HYPERNET.captures_iter(line) {
        if is_abba(&cap[1]) {
            return false;
        }
    }
    let non_hypernet = HYPERNET.replace_all(line, "|");
    is_abba(&non_hypernet)
}

fn get_babs(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut rv: Vec<String> = Vec::new();
    for (i, &value) in chars.iter().take(chars.len() - 2).enumerate() {
        if value == chars[i + 2] && value != chars[i + 1] {
            let mut bab = String::new();
            bab.push(chars[i + 1]);
            bab.push(value);
            bab.push(chars[i + 1]);
            rv.push(bab.clone());
        }
    }
    rv
}

fn is_ssl(line: &str) -> bool {
    lazy_static! {
        static ref HYPERNET: Regex = Regex::new(r"\[([a-z]+)]").unwrap();
    }
    for cap in HYPERNET.captures_iter(line) {
        let babs = get_babs(&cap[1]);
        let non_hypernet = HYPERNET.replace_all(line, "|");
        for bab in babs {
            if non_hypernet.contains(&bab[..]) {
                return true;
            }
        }
    }
    false
}

impl Day for Q {
    fn number(&self) -> String {
        String::from("7")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let mut result = 0;
        for line in INPUT.lines() {
            if is_tls(line) {
                result += 1;
            }
        }
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let mut result = 0;
        for line in INPUT.lines() {
            if is_ssl(line) {
                result += 1;
            }
        }
        println!("Result = {}", result);
    }
}
