use core::fmt;
use std::collections::{HashMap, VecDeque};

use custom_error::custom_error;

custom_error! {
    #[derive(PartialEq,PartialOrd)]
    pub IntcodeError
        ModeNotFound {flag: i128} = "Mode {flag} not understood.",
        OpcodeNotFound {opcode: i128} = "Opcode {opcode} not understood",
        MachineHalted = "Attempting to run halted machine.",
        MissingValue = "Missing value.",
        MachineNotWaiting = "Machine should be waiting for input, but isn't",
        InvalidPosition {position: usize, len: usize} = "Invalid position {position}, should be less than {len}",
        MachineExceededLimit {limit: usize}= "Machine exceeded run length limit of {limit}",
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Ready,
    WaitingForInput,
    Halted,
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

impl Mode {
    fn get_mode(flag: i128) -> Result<Mode, IntcodeError> {
        match flag % 10 {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(IntcodeError::ModeNotFound { flag }),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add {
        mode: (Mode, Mode, Mode),
        a: i128,
        b: i128,
        dest: i128,
    },
    Multiply {
        mode: (Mode, Mode, Mode),
        a: i128,
        b: i128,
        dest: i128,
    },
    Input {
        mode: Mode,
        dest: i128,
    },
    Output {
        mode: Mode,
        src: i128,
    },
    JumpIfTrue {
        mode: (Mode, Mode),
        test: i128,
        pos: i128,
    },
    JumpIfFalse {
        mode: (Mode, Mode),
        test: i128,
        pos: i128,
    },
    LessThan {
        mode: (Mode, Mode, Mode),
        a: i128,
        b: i128,
        dest: i128,
    },
    Equals {
        mode: (Mode, Mode, Mode),
        a: i128,
        b: i128,
        dest: i128,
    },
    SetBase {
        mode: Mode,
        base: i128,
    },
    Halt,
}

impl Opcode {
    fn get_size(&self) -> usize {
        match self {
            Opcode::Add { .. } => 4,
            Opcode::Multiply { .. } => 4,
            Opcode::Input { .. } => 2,
            Opcode::Output { .. } => 2,
            Opcode::JumpIfTrue { .. } => 3,
            Opcode::JumpIfFalse { .. } => 3,
            Opcode::LessThan { .. } => 4,
            Opcode::Equals { .. } => 4,
            Opcode::SetBase { .. } => 2,
            Opcode::Halt => 0,
        }
    }
    fn get_opcode(memory: &HashMap<usize, i128>, position: usize) -> Result<Opcode, IntcodeError> {
        let code = memory[&position];
        match code % 100 {
            1 => {
                let mode = (
                    Mode::get_mode(code / 100)?,
                    Mode::get_mode(code / 1000)?,
                    Mode::get_mode(code / 10_000)?,
                );
                let a = memory[&(position + 1)];
                let b = memory[&(position + 2)];
                let dest = memory[&(position + 3)];
                Ok(Opcode::Add { mode, a, b, dest })
            }
            2 => {
                let mode = (
                    Mode::get_mode(code / 100)?,
                    Mode::get_mode(code / 1000)?,
                    Mode::get_mode(code / 10_000)?,
                );
                let a = memory[&(position + 1)];
                let b = memory[&(position + 2)];
                let dest = memory[&(position + 3)];
                Ok(Opcode::Multiply { mode, a, b, dest })
            }
            3 => {
                let mode = Mode::get_mode(code / 100)?;
                let dest = memory[&(position + 1)];
                Ok(Opcode::Input { mode, dest })
            }
            4 => {
                let mode = Mode::get_mode(code / 100)?;
                let src = memory[&(position + 1)];
                Ok(Opcode::Output { mode, src })
            }
            5 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let test = memory[&(position + 1)];
                let pos = memory[&(position + 2)];
                Ok(Opcode::JumpIfTrue { mode, test, pos })
            }
            6 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let test = memory[&(position + 1)];
                let pos = memory[&(position + 2)];
                Ok(Opcode::JumpIfFalse { mode, test, pos })
            }
            7 => {
                let mode = (
                    Mode::get_mode(code / 100)?,
                    Mode::get_mode(code / 1000)?,
                    Mode::get_mode(code / 10_000)?,
                );
                let a = memory[&(position + 1)];
                let b = memory[&(position + 2)];
                let dest = memory[&(position + 3)];
                Ok(Opcode::LessThan { mode, a, b, dest })
            }
            8 => {
                let mode = (
                    Mode::get_mode(code / 100)?,
                    Mode::get_mode(code / 1000)?,
                    Mode::get_mode(code / 10_000)?,
                );
                let a = memory[&(position + 1)];
                let b = memory[&(position + 2)];
                let dest = memory[&(position + 3)];
                Ok(Opcode::Equals { mode, a, b, dest })
            }
            9 => {
                let mode = Mode::get_mode(code / 100)?;
                let base = memory[&(position + 1)];
                Ok(Opcode::SetBase { mode, base })
            }
            99 => Ok(Opcode::Halt),
            _ => Err(IntcodeError::OpcodeNotFound {
                opcode: memory[&position],
            }),
        }
    }
}

#[derive(Clone)]
pub struct Intcode {
    position: usize,
    state: State,
    base: i128,
    pub memory: HashMap<usize, i128>,
    pub inputs: VecDeque<i128>,
    pub outputs: VecDeque<i128>,
}

impl fmt::Debug for Intcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Intcode {{ position: {}, state: {:?}, inputs: {:?}, outputs: {:?} }}",
            self.position, self.state, self.inputs, self.outputs
        )
    }
}

impl Intcode {
    pub fn new(memory: Vec<i128>, inputs: Vec<i128>) -> Self {
        Intcode {
            position: 0,
            state: State::Ready,
            base: 0,
            memory: memory.into_iter().enumerate().collect(),
            inputs: VecDeque::from(inputs),
            outputs: VecDeque::new(),
        }
    }

    fn get_value(&mut self, mode: Mode, value: i128) -> i128 {
        match mode {
            Mode::Immediate => value,
            Mode::Position => *self.memory.get(&(value as usize)).unwrap_or(&0),
            Mode::Relative => *self
                .memory
                .get(&((value + self.base) as usize))
                .unwrap_or(&0),
        }
    }

    fn get_location(&mut self, mode: Mode, value: i128) -> usize {
        match mode {
            Mode::Immediate => panic!("ERROR!!!! Immediate mode for location!!!"),
            Mode::Position => value as usize,
            Mode::Relative => (value + self.base) as usize,
        }
    }

    pub fn run_step(&mut self) -> Result<State, IntcodeError> {
        let opcode = Opcode::get_opcode(&self.memory, self.position)?;
        let mut jumped = false;
        // println!("Executing {:?} b:{}", opcode, self.base);
        match &opcode {
            Opcode::Add { mode, a, b, dest } => {
                let a = self.get_value(mode.0, *a);
                let b = self.get_value(mode.1, *b);
                let dest = self.get_location(mode.2, *dest);
                self.memory.insert(dest, a + b);
            }
            Opcode::Multiply { mode, a, b, dest } => {
                let a = self.get_value(mode.0, *a);
                let b = self.get_value(mode.1, *b);
                let dest = self.get_location(mode.2, *dest);
                self.memory.insert(dest, a * b);
            }
            Opcode::Input { mode, dest } => match self.inputs.pop_front() {
                Some(value) => {
                    let dest = self.get_location(*mode, *dest);
                    self.memory.insert(dest, value);
                    self.state = State::Ready;
                }
                None => {
                    jumped = true;
                    self.state = State::WaitingForInput;
                }
            },
            Opcode::Output { mode, src } => {
                let value = self.get_value(*mode, *src);
                self.outputs.push_front(value);
            }
            Opcode::JumpIfTrue { mode, test, pos } => {
                if self.get_value(mode.0, *test) != 0 {
                    jumped = true;
                    self.position = self.get_value(mode.1, *pos) as usize;
                }
            }
            Opcode::JumpIfFalse { mode, test, pos } => {
                if self.get_value(mode.0, *test) == 0 {
                    jumped = true;
                    self.position = self.get_value(mode.1, *pos) as usize;
                }
            }
            Opcode::LessThan { mode, a, b, dest } => {
                let a = self.get_value(mode.0, *a);
                let b = self.get_value(mode.1, *b);
                let dest = self.get_location(mode.2, *dest);
                self.memory.insert(dest, if a < b { 1 } else { 0 });
            }
            Opcode::Equals { mode, a, b, dest } => {
                let a = self.get_value(mode.0, *a);
                let b = self.get_value(mode.1, *b);
                let dest = self.get_location(mode.2, *dest);
                self.memory.insert(dest, if a == b { 1 } else { 0 });
            }
            Opcode::SetBase { mode, base } => {
                self.base += self.get_value(*mode, *base);
            }
            Opcode::Halt => {
                self.state = State::Halted;
            }
        }
        if !jumped {
            self.position += opcode.get_size();
        }
        if self.position >= self.memory.len() {
            Err(IntcodeError::InvalidPosition {
                position: self.position,
                len: self.memory.len(),
            })
        } else {
            Ok(self.state)
        }
    }

    pub fn run_tape(&mut self) -> Result<State, IntcodeError> {
        if self.state == State::Halted {
            return Err(IntcodeError::MachineHalted);
        }
        loop {
            match self.run_step()? {
                State::WaitingForInput | State::Halted => {
                    break;
                }
                _ => {}
            }
        }
        Ok(self.state)
    }

    pub fn run_tape_until(&mut self, limit: usize) -> Result<State, IntcodeError> {
        if self.state == State::Halted {
            return Err(IntcodeError::MachineHalted);
        }
        let mut i = 0;
        loop {
            i += 1;
            match self.run_step()? {
                State::WaitingForInput | State::Halted => {
                    break;
                }
                _ => {}
            }
            if i > limit {
                return Err(IntcodeError::MachineExceededLimit { limit });
            }
        }
        Ok(self.state)
    }
}
