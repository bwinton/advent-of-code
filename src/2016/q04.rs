//-----------------------------------------------------
// Setup.

use aoc::Day;

static INPUT: &'static str = include_str!("data/q04.data");
// static INPUT : &'static str = "aaaaa-bbb-z-y-x-123[abxyz]
// a-b-c-d-e-f-g-h-987[abcde]
// not-a-real-room-404[oarel]
// totally-real-room-200[decoy]
// qzmt-zixmtkozy-ivhz-343[abcde]";

use regex::Regex;
use std::iter::FromIterator;

#[derive(Debug)]
struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut chars = Vec::<(i32, char)>::new();
        for char in self.name.chars() {
            if char != '-' {
                let pos = chars.iter().position(|&r| r.1 == char);
                match pos {
                    None => chars.push((-1, char)),
                    Some(i) => chars[i].0 -= 1,
                }
            }
        }
        chars.sort();
        chars.truncate(5);
        let data = String::from_iter(chars.iter().map(|x| x.1));
        // println!("{}.{} => {:?}    {}", self.name, self.checksum, data, self.checksum == data);
        self.checksum == data
    }

    fn decrypt(&self) -> String {
        let shift = (self.sector % 26) as u8;
        let alphabet = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        // rot-n, where n is sector % 26…
        let name: String = self
            .name
            .chars()
            .map(|c| {
                *alphabet
                    .iter()
                    .chain(alphabet.iter())
                    .skip_while(|&x| *x != c)
                    .nth(usize::from(shift))
                    .unwrap_or(&c)
            })
            .collect();
        name.replace('-', " ")
    }
}

use std::str::FromStr;
impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Room, ()> {
        let room_re: Regex = Regex::new(r"^([a-z-]+)-([0-9]+)\[([a-z]{5})\]$").unwrap();
        let blank = String::from("");
        let mut rv = Room {
            name: blank.clone(),
            sector: 0,
            checksum: blank.clone(),
        };
        for cap in room_re.captures_iter(s) {
            rv.name = cap[1].to_string();
            rv.sector = cap[2].parse().unwrap();
            rv.checksum = cap[3].to_string();
        }
        Ok(rv)
        // on fail, Err(())
    }
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("4")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let mut sum: i32 = 0;
        for line in INPUT.lines() {
            let room: Room = line.parse().unwrap();
            if room.is_valid() {
                sum += room.sector;
            }
        }
        println!("Result = {}", sum);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        for line in INPUT.lines() {
            let room: Room = line.parse().unwrap();
            let name = room.decrypt();
            if name.find("northpole object storage") != None {
                println!("Result = \"{}\" {}", room.decrypt(), room.sector);
                return;
            }
        }
    }
}
