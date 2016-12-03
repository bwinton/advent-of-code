//-----------------------------------------------------
// Common Header.

pub fn select(arg: String) {
  match arg.as_ref() {
    "01a" => a(),
    "01b" => b(),
    "01" => {a(); b()},
    _ => ()
  }
}

//-----------------------------------------------------
// Setup.

static INPUT : &'static str = "L5, R1, R4, L5, L4, R3, R1, L1, R4, R5, L1, L3, R4, L2, L4, R2, L4, L1, R3, R1, R1, L1, R1, L5, R5, R2, L5, R2, R1, L2, L4, L4, R191, R2, R5, R1, L1, L2, R5, L2, L3, R4, L1, L1, R1, R50, L1, R1, R76, R5, R4, R2, L5, L3, L5, R2, R1, L1, R2, L3, R4, R2, L1, L1, R4, L1, L1, R185, R1, L5, L4, L5, L3, R2, R3, R1, L5, R1, L3, L2, L2, R5, L1, L1, L3, R1, R4, L2, L1, L1, L3, L4, R5, L2, R3, R5, R1, L4, R5, L3, R3, R3, R1, R1, R5, R2, L2, R5, L5, L4, R4, R3, R5, R1, L3, R1, L2, L2, R3, R4, L1, R4, L1, R4, R3, L1, L4, L1, L5, L2, R2, L1, R1, L5, L3, R4, L1, R5, L5, L5, L1, L3, R1, R5, L2, L4, L5, L1, L1, L2, R5, R5, L4, R3, L2, L1, L3, L4, L5, L5, L2, R4, R3, L5, R4, R2, R1, L5";
// static INPUT : &'static str = "R2, L3";

#[derive(Debug)]
enum Heading {
  North, East, South, West
}

type Pos = [i32; 2];

fn next_heading(heading: &mut Heading, dir: &str) {
  match *heading {
    Heading::North => if dir == "R" {
      *heading = Heading::East;
    } else {
      *heading = Heading::West;
    },
    Heading::East => if dir == "R" {
      *heading = Heading::South;
    } else {
      *heading = Heading::North;
    },
    Heading::South => if dir == "R" {
      *heading = Heading::West;
    } else {
      *heading = Heading::East;
    },
    Heading::West => if dir == "R" {
      *heading = Heading::North;
    } else {
      *heading = Heading::South;
    }
  }
  // println!("{}, {:?}", dir, heading);

}

fn handle_turn(turn: String, heading: &mut Heading) -> i32 {
  let (dir, len_str) = turn.split_at(1);
  let length : i32 = len_str.parse().expect("Wanted a number");
  next_heading(heading, dir);
  return length;
}


//-----------------------------------------------------
// Questions.

fn a() {
  let mut heading:Heading = Heading::North;
  let mut pos : Pos = [0, 0];

  fn run_turn(pos : &mut Pos, heading: & Heading, length: i32) {
    // length = headings[heading].map(x => x * length);
    // pos = pos.map((x, i) => x + length[i]);
    match *heading {
      Heading::North => pos[0] += length,
      Heading::East => pos[1] += length,
      Heading::South => pos[0] -= length,
      Heading::West => pos[1] -= length
    }
    // println!("{:?}, {} {:?}", pos, length, heading);
  }

  println!("A:");
  for data in INPUT.split(", ") {
    let length = handle_turn(String::from(data), &mut heading);
    run_turn(&mut pos, &heading, length);
  }
  println!("Result: {}", pos[0].abs() + pos[1].abs());
}

fn b() {
  let mut heading:Heading = Heading::North;
  let mut pos : Pos = [0, 0];

  fn run_turn(pos : &mut Pos, heading: & Heading, length: i32) {
    // length = headings[heading].map(x => x * length);
    // pos = pos.map((x, i) => x + length[i]);
    match *heading {
      Heading::North => pos[0] += length,
      Heading::East => pos[1] += length,
      Heading::South => pos[0] -= length,
      Heading::West => pos[1] -= length
    }
    // println!("{:?}, {} {:?}", pos, length, heading);
  }

  println!("B:");
  for data in INPUT.split(", ") {
    let length = handle_turn(String::from(data), &mut heading);
    run_turn(&mut pos, &heading, length);
  }
  println!("Result: {}", pos[0].abs() + pos[1].abs());
}
