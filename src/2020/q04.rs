//-----------------------------------------------------
// Setup.

use once_cell::sync::Lazy;
use regex::Regex;

static INPUT: &str = include_str!("data/q04.data");

static HCL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[0-9a-z]{6}$").unwrap());
static HGT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0-9a-z]+)(cm|in)$").unwrap());
static PID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]{9}$").unwrap());

#[derive(Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl EyeColor {
    fn from(color: &str) -> Option<Self> {
        match color {
            "amb" => Some(Self::Amb),
            "blu" => Some(Self::Blu),
            "brn" => Some(Self::Brn),
            "gry" => Some(Self::Gry),
            "grn" => Some(Self::Grn),
            "hzl" => Some(Self::Hzl),
            "oth" => Some(Self::Oth),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct PassportBuilder {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportBuilder {
    fn new() -> Self {
        PassportBuilder {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn add_field(&mut self, key: &str, value: &str) -> bool {
        match key {
            "byr" => {
                self.byr = Some(value.to_owned());
                true
            }
            "iyr" => {
                self.iyr = Some(value.to_owned());
                true
            }
            "eyr" => {
                self.eyr = Some(value.to_owned());
                true
            }
            "hgt" => {
                self.hgt = Some(value.to_owned());
                true
            }
            "hcl" => {
                self.hcl = Some(value.to_owned());
                true
            }
            "ecl" => {
                self.ecl = Some(value.to_owned());
                true
            }
            "pid" => {
                self.pid = Some(value.to_owned());
                true
            }
            "cid" => {
                self.cid = Some(value.to_owned());
                true
            }
            _ => false,
        }
    }

    fn complete(&self) -> bool {
        self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
    }

    fn build(self, debug: bool) -> Option<Passport> {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if self.byr == None {
            if debug {
                println!("Missing field byr!");
            }
            return None;
        }
        let byr: Option<usize> = self.byr.unwrap().parse().ok();
        if byr == None {
            if debug {
                println!("Field byr not an int!");
            }
            return None;
        }
        let byr = byr.unwrap();

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if self.iyr == None {
            if debug {
                println!("Missing field iyr!");
            }
            return None;
        }
        let iyr: Option<usize> = self.iyr.unwrap().parse().ok();
        if iyr == None {
            if debug {
                println!("Field iyr not an int!");
            }
            return None;
        }
        let iyr = iyr.unwrap();

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if self.eyr == None {
            if debug {
                println!("Missing field eyr!");
            }
            return None;
        }
        let eyr: Option<usize> = self.eyr.unwrap().parse().ok();
        if eyr == None {
            if debug {
                println!("Field eyr not an int!");
            }
            return None;
        }
        let eyr = eyr.unwrap();

        // hgt (Height) - a number followed by either cm or in:
        if self.hgt == None {
            if debug {
                println!("Missing field hgt!");
            }
            return None;
        }
        let temp = &self.hgt.unwrap();
        let hgt = if let Some(captures) = HGT_RE.captures(temp) {
            let value: Option<usize> = captures[1].parse().ok();
            if value == None {
                if debug {
                    println!("Field hgt not an int!");
                }
                return None;
            }
            let value = value.unwrap();

            let units = &captures[2];
            match units {
                "cm" | "in" => (value, units.to_string()),
                _ => {
                    if debug {
                        println!("HGT {} unknown", &temp);
                    }
                    return None;
                }
            }
        } else {
            if debug {
                println!("HGT {} invalid", &temp);
            }
            return None;
        };

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if self.hcl == None {
            if debug {
                println!("Missing field hcl!");
            }
            return None;
        }
        let hcl = self.hcl.unwrap();

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if self.ecl == None {
            if debug {
                println!("Missing field ecl!");
            }
            return None;
        }
        let ecl = EyeColor::from(&self.ecl.clone().unwrap());
        if ecl.is_none() {
            if debug {
                println!("ECL {} invalid", &self.ecl.unwrap());
            }
            return None;
        }
        let _ecl = ecl.unwrap();

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if self.pid == None {
            if debug {
                println!("Missing field ecl!");
            }
            return None;
        }
        let pid = self.pid.unwrap();

        let _cid = self.cid;
        // cid (Country ID) - ignored, missing or not.
        Some(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            _ecl,
            pid,
            _cid,
        })
    }
}

#[derive(Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: (usize, String),
    hcl: String,
    _ecl: EyeColor,
    pid: String,
    _cid: Option<String>,
}

impl Passport {
    fn is_valid(&self, debug: bool) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if !(1920..=2002).contains(&self.byr) {
            if debug {
                println!("BYR {} out of range 1920-2002", &self.byr);
            }
            return false;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if !(2010..=2020).contains(&self.iyr) {
            if debug {
                println!("IYR {} out of range 2010-2020", &self.iyr);
            }
            return false;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if !(2020..=2030).contains(&self.eyr) {
            if debug {
                println!("EYR {} out of range 2020-2030", &self.eyr);
            }
            return false;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        match self.hgt.1.as_str() {
            "cm" => {
                if !(150..=193).contains(&self.hgt.0) {
                    if debug {
                        println!("HGT {}cm out of range 150-193", &self.hgt.0);
                    }
                    return false;
                }
            }
            "in" => {
                if !(59..=76).contains(&self.hgt.0) {
                    if debug {
                        println!("HGT {}in out of range 59-76", &self.hgt.0);
                    }
                    return false;
                }
            }
            _ => {
                unreachable!()
            }
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if !HCL_RE.is_match(&self.hcl) {
            if debug {
                println!("HCL {} unknown", &self.hcl);
            }
            return false;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // Done in the builder.

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if !PID_RE.is_match(&self.pid) {
            if debug {
                println!("PID {} unknown", &self.pid);
            }
            return false;
        }

        // cid (Country ID) - ignored, missing or not.

        true
    }
}

fn process_data_a(data: &str) -> usize {
    let mut rv = vec![];
    let mut curr = PassportBuilder::new();
    for line in data.lines() {
        if line.is_empty() {
            if curr.complete() {
                rv.push(curr);
            }
            curr = PassportBuilder::new();
            continue;
        }
        for value in line.split(' ') {
            let items: Vec<_> = value.splitn(2, ':').collect();
            if !curr.add_field(items[0], items[1]) {
                println!("Error! Unknown ID: {}", &value);
            }
        }
    }
    if curr.complete() {
        rv.push(curr);
    }

    rv.len()
}

fn process_data_b(data: &str) -> usize {
    let mut rv = vec![];
    let mut curr = PassportBuilder::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            if let Some(passport) = curr.build(false) {
                if passport.is_valid(false) {
                    // println!("Adding {}, of {:?}", rv.len(), &curr);
                    rv.push(passport);
                }
            }
            curr = PassportBuilder::new();
            continue;
        }
        for value in line.split(' ') {
            let items: Vec<_> = value.splitn(2, ':').collect();
            if !curr.add_field(items[0], items[1]) {
                println!("Error! Unknown ID: {}", &value);
            }
        }
    }
    // Handle the last one.
    if let Some(passport) = curr.build(false) {
        if passport.is_valid(false) {
            // println!("Adding {}, of {:?}", rv.len(), &curr);
            rv.push(passport);
        }
    }

    rv.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"
        ),
        2
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"
        ),
        0
    );

    assert_eq!(
        process_data_b(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"
        ),
        4
    );

    // println!("{:?}", HCL_RE.find("#a97842"));
    // println!("{:?}", HCL_RE.is_match("#a97842"));
    // println!("{:?}", !HCL_RE.is_match("#a97842"));
    // assert!(false);
}
