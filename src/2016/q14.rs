//-----------------------------------------------------
// Setup.

use aoc::Day;
use crypto::{digest::Digest, md5::Md5};
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, mem};

// static INPUT : &'static str = "abc";
static INPUT: &str = "zpqevtbw";

#[derive(Debug, Eq, PartialEq)]
enum Key {
    Potential(usize),
    Confirmed(usize, String),
}

impl Ord for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        match *self {
            Key::Potential(ref me) | Key::Confirmed(ref me, _) => match *other {
                Key::Potential(ref them) | Key::Confirmed(ref them, _) => me.cmp(them),
            },
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Keys = Vec<Key>;
type KeysRef<'a> = &'a [Key];

#[derive(Clone, Debug)]
struct Quintuple {
    _key: String,
    regex: Regex,
    indices: Vec<usize>,
}

impl Quintuple {
    pub fn new(key: &str) -> Quintuple {
        Quintuple {
            _key: key.to_string(),
            regex: Regex::new(&key.repeat(5)).unwrap(),
            indices: Vec::new(),
        }
    }
}

type Quintuples = HashMap<String, Quintuple>;

pub fn to_hex_string(bytes: &[u8; 16]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    strs.join("")
}

pub fn get_triple(input: &str) -> Option<String> {
    let re: &Regex = regex!("(0){3}|(1){3}|(2){3}|(3){3}|(4){3}|(5){3}|(6){3}|(7){3}|(8){3}|(9){3}|(a){3}|(b){3}|(c){3}|(d){3}|(e){3}|(f){3}");
    re.captures(input)
        .map(|key| key[0].chars().next().unwrap().to_string())
}

fn add_quintuple(i: usize, key: &str, quintuples: &mut Quintuples) {
    let quintuple = quintuples
        .entry(key.to_string())
        .or_insert_with(|| Quintuple::new(key));
    quintuple.indices.push(i);
}

fn remove_keys(count: usize, keys: &mut Keys, quintuples: &mut Quintuples) {
    quintuples.retain(|_, quintuple| {
        quintuple.indices.retain(|i| i + 1001 > count);
        !quintuple.indices.is_empty()
    });
    keys.retain(|key| match *key {
        Key::Confirmed(_, _) => true,
        Key::Potential(i) => i + 1001 >= count,
    });
}

fn get_quintuple(input: &str, keys: &mut Keys, quintuples: &mut Quintuples, count: usize) -> usize {
    let mut rv = 0;
    for (key, quintuple) in &mut quintuples.clone() {
        if quintuple.regex.is_match(input) {
            let len = quintuple.indices.len();
            for index in quintuple.indices.clone() {
                if index == count {
                    continue;
                }
                let new_key = Key::Confirmed(index, input.to_string());
                let key_index = keys.binary_search(&new_key).unwrap();
                let _ = mem::replace(&mut keys[key_index], new_key);
            }
            quintuple.indices.retain(|i| *i == count);

            if quintuple.indices.is_empty() {
                quintuples.remove(key);
            }
            rv += len;
        }
    }
    rv
}

fn is_winning(keys: KeysRef) -> bool {
    if keys.len() < 64 {
        return false;
    }
    for key in &keys[0..64] {
        if let Key::Potential(_) = *key {
            return false;
        }
    }
    true
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("14")
    }

    fn a(&self) {
        print!("{}A: ", self.number());

        let mut keys: Keys = Vec::new();
        let mut quintuples = HashMap::new();

        let mut hasher = Md5::new();
        let mut i: usize = 0;
        while !is_winning(&keys) {
            // while i < 818 {
            hasher.input(INPUT.as_bytes());
            hasher.input(i.to_string().as_bytes());
            let mut output = [0; 16]; // An MD5 is 16 bytes
            hasher.result(&mut output);
            hasher.reset();
            // to_hex_string(&output);
            let out_string = to_hex_string(&output);
            match get_triple(&out_string) {
                None => {}
                Some(triple) => {
                    keys.push(Key::Potential(i));
                    add_quintuple(i, &triple, &mut quintuples);
                }
            }
            remove_keys(i, &mut keys, &mut quintuples);
            get_quintuple(&out_string, &mut keys, &mut quintuples, i);
            i += 1;
        }
        println!("Keys:");
        for key in &keys[0..64] {
            println!(" {:?}", key);
        }
        println!("Result = {:?}", keys[63]);
    }

    fn b(&self) {
        print!("{}B: ", self.number());

        let mut keys: Keys = Vec::new();
        let mut quintuples = HashMap::new();

        let mut hasher = Md5::new();
        let mut i: usize = 0;
        while !is_winning(&keys) {
            let mut out_string = INPUT.to_string() + &i.to_string();
            for _ in 0..2017 {
                hasher.input(out_string.as_bytes());
                let mut output = [0; 16]; // An MD5 is 16 bytes
                hasher.result(&mut output);
                hasher.reset();
                out_string = to_hex_string(&output);
            }
            match get_triple(&out_string) {
                None => {}
                Some(triple) => {
                    keys.push(Key::Potential(i));
                    add_quintuple(i, &triple, &mut quintuples);
                }
            }
            remove_keys(i, &mut keys, &mut quintuples);
            get_quintuple(&out_string, &mut keys, &mut quintuples, i);
            i += 1;
        }
        println!("Keys:");
        for key in &keys[0..64] {
            println!(" {:?}", key);
        }
        println!("Result = {:?}", keys[63]);
    }
}
