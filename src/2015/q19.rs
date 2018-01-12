//-----------------------------------------------------
// Setup.

use aoc::Day;

use nom::alpha;
use regex::Regex;
use std::collections::HashSet;

static INPUT : &'static str = "Al => ThF
Al => ThRnFAr
B => BCa
B => TiB
B => TiRnFAr
Ca => CaCa
Ca => PB
Ca => PRnFAr
Ca => SiRnFYFAr
Ca => SiRnMgAr
Ca => SiTh
F => CaF
F => PMg
F => SiAl
H => CRnAlAr
H => CRnFYFYFAr
H => CRnFYMgAr
H => CRnMgYFAr
H => HCa
H => NRnFYFAr
H => NRnMgAr
H => NTh
H => OB
H => ORnFAr
Mg => BF
Mg => TiMg
N => CRnFAr
N => HSi
O => CRnFYFAr
O => CRnMgAr
O => HP
O => NRnFAr
O => OTi
P => CaP
P => PTi
P => SiRnFAr
Si => CaSi
Th => ThCa
Ti => BP
Ti => TiTi
e => HF
e => NAl
e => OMg

CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";

#[derive(Clone)]
#[derive(Debug)]
struct Rule {
  source: Regex,
  dest: String
}

impl Rule {
  fn match_all(self, start: &str) -> Vec<String> {
    let mut rv = Vec::new();
    for found in self.source.find_iter(start) {
      let mut dest = start.to_owned();
      dest.splice(found.0 .. found.1, &self.dest);
      rv.push(dest);
    }
    rv
  }
}

named!(rule_parser<&str, Rule>, do_parse!(
  source: alpha >>
  tag!(" => ") >>
  dest: alpha >>
  tag!("\n") >>
  (Rule { source: Regex::new(source).unwrap(), dest: dest.to_owned() })
));

named!(parser<&str, (Vec<Rule>, String)>, complete!(do_parse!(
  rules: many1!(rule_parser) >>
  tag!("\n") >>
  start: alpha >>
  (rules.to_vec(), start.to_owned())
)));

fn process_data_a(data: &str) -> usize {
  let (rules, start) = parser(data).unwrap().1;
  let mut rv = HashSet::new();
  for rule in rules {
    let matches = rule.match_all(&start);
    rv.extend(matches);
  }
  rv.len()
}

fn process_data_b(data: &str) -> usize {
  let (_, goal) = parser(data).unwrap().1;
  let tokens: Vec<String> = Regex::new("[A-Z][a-z]?").unwrap().captures_iter(&goal).map(|x| x[0].to_owned()).collect();
  let brackets: Vec<&String> = tokens.iter().filter(|&x| *x == "Rn" || *x == "Ar").collect();
  let commas: Vec<&String> = tokens.iter().filter(|&x| *x == "Y").collect();
  tokens.len() - brackets.len() - 2 * commas.len() - 1
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
  fn number(&self) -> String {
    String::from("19")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(process_data_a("H => HO
H => OH
O => HH

HOH"), 4);
assert_eq!(process_data_a("H => HO
H => OH
O => HH

HOHOHO"), 7);
}

#[test]
fn b() {
}
