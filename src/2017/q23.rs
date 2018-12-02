//-----------------------------------------------------
// Setup.

use aoc::Day;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

static INPUT: &'static str = include_str!("data/q23.data");

#[derive(Clone, Debug)]
enum Instruction {
    SetReg(char, char),
    SetLit(char, i64),
    SubReg(char, char),
    SubLit(char, i64),
    MulReg(char, char),
    MulLit(char, i64),
    JumpRegReg(char, char),
    JumpRegLit(char, i64),
    JumpLitReg(i64, char),
    JumpLitLit(i64, i64),
}

impl Instruction {
    fn execute(&self, state: &State) -> State {
        let mut rv = state.clone();
        match (*self).clone() {
            Instruction::SetReg(dst, src) => {
                rv.pc += 1;
                rv.registers.insert(dst, state.registers[&src]);
            }
            Instruction::SetLit(reg, lit) => {
                rv.pc += 1;
                rv.registers.insert(reg, lit);
            }
            Instruction::SubReg(dst, src) => {
                rv.pc += 1;
                rv.registers
                    .insert(dst, state.registers[&dst] - state.registers[&src]);
            }
            Instruction::SubLit(reg, lit) => {
                rv.pc += 1;
                rv.registers.insert(reg, state.registers[&reg] - lit);
            }
            Instruction::MulReg(dst, src) => {
                rv.pc += 1;
                rv.registers
                    .insert(dst, state.registers[&dst] * state.registers[&src]);
            }
            Instruction::MulLit(reg, lit) => {
                rv.pc += 1;
                rv.registers.insert(reg, state.registers[&reg] * lit);
            }
            Instruction::JumpRegReg(reg_test, reg_offset) => {
                if rv.registers[&reg_test] != 0 {
                    rv.pc += rv.registers[&reg_offset];
                } else {
                    rv.pc += 1;
                }
            }
            Instruction::JumpRegLit(reg, offset) => {
                if rv.registers[&reg] != 0 {
                    rv.pc += offset;
                } else {
                    rv.pc += 1;
                }
            }
            Instruction::JumpLitReg(test, reg) => {
                if test != 0 {
                    rv.pc += rv.registers[&reg];
                } else {
                    rv.pc += 1;
                }
            }
            Instruction::JumpLitLit(test, offset) => {
                if test != 0 {
                    rv.pc += offset;
                } else {
                    rv.pc += 1;
                }
            }
        }
        rv
    }

    fn registers(&self) -> Vec<char> {
        match (*self).clone() {
            Instruction::SetLit(reg, _)
            | Instruction::SubLit(reg, _)
            | Instruction::MulLit(reg, _)
            | Instruction::JumpRegLit(reg, _)
            | Instruction::JumpLitReg(_, reg) => vec![reg],
            Instruction::JumpLitLit(_, _) => vec![],
            Instruction::SetReg(a, b)
            | Instruction::SubReg(a, b)
            | Instruction::MulReg(a, b)
            | Instruction::JumpRegReg(a, b) => vec![a, b],
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        lazy_static! {
            static ref SET_REG_RE: Regex = Regex::new(r"^ *set ([a-z]) ([a-z])( *=>.*)?$").unwrap();
            static ref SET_LIT_RE: Regex = Regex::new(r"^ *set ([a-z]) (-?\d+)( *=>.*)?$").unwrap();
            static ref SUB_REG_RE: Regex = Regex::new(r"^ *sub ([a-z]) ([a-z])( *=>.*)?$").unwrap();
            static ref SUB_LIT_RE: Regex = Regex::new(r"^ *sub ([a-z]) (-?\d+)( *=>.*)?$").unwrap();
            static ref MUL_REG_RE: Regex = Regex::new(r"^ *mul ([a-z]) ([a-z])( *=>.*)?$").unwrap();
            static ref MUL_LIT_RE: Regex = Regex::new(r"^ *mul ([a-z]) (-?\d+)( *=>.*)?$").unwrap();
            static ref JUMP_LITLIT_RE: Regex =
                Regex::new(r"^ *jnz (-?[0-9]+) (-?[0-9]+)( *=>.*)?$").unwrap();
            static ref JUMP_LITREG_RE: Regex =
                Regex::new(r"^ *jnz (-?[0-9]+) ([a-z])( *=>.*)?$").unwrap();
            static ref JUMP_REGLIT_RE: Regex =
                Regex::new(r"^ *jnz ([a-z]) (-?[0-9]+)( *=>.*)?$").unwrap();
            static ref JUMP_REGREG_RE: Regex =
                Regex::new(r"^ *jnz ([a-z]) ([a-z])( *=>.*)?$").unwrap();
        }

        if let Some(cap) = SET_REG_RE.captures(s) {
            return Ok(Instruction::SetReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = SET_LIT_RE.captures(s) {
            return Ok(Instruction::SetLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = SUB_REG_RE.captures(s) {
            return Ok(Instruction::SubReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = SUB_LIT_RE.captures(s) {
            return Ok(Instruction::SubLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = MUL_REG_RE.captures(s) {
            return Ok(Instruction::MulReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = MUL_LIT_RE.captures(s) {
            return Ok(Instruction::MulLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = JUMP_REGREG_RE.captures(s) {
            return Ok(Instruction::JumpRegReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = JUMP_REGLIT_RE.captures(s) {
            return Ok(Instruction::JumpRegLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = JUMP_LITREG_RE.captures(s) {
            return Ok(Instruction::JumpLitReg(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        if let Some(cap) = JUMP_LITLIT_RE.captures(s) {
            return Ok(Instruction::JumpLitLit(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
            ));
        }

        println!("Unknown instruction! '{}'", s);
        Err(())
    }
}

#[derive(Clone, Debug)]
struct State {
    registers: HashMap<char, i64>,
    pc: i64,
    incoming: Vec<i64>,
    outgoing: Vec<i64>,
    instructions: Vec<Instruction>,
    waiting: bool,
}

impl State {
    pub fn new(instructions: Vec<Instruction>, registers: HashMap<char, i64>) -> State {
        State {
            registers,
            pc: 0,
            incoming: Vec::new(),
            outgoing: Vec::new(),
            instructions,
            waiting: false,
        }
    }

    fn execute(&self) -> State {
        let instruction = &self.instructions[self.pc as usize];
        instruction.execute(self)
    }
}

fn process_data_a(data: &str) -> i64 {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut registers: HashMap<char, i64> = HashMap::new();
    for line in data.lines() {
        let instruction: Instruction = line.parse().unwrap();
        instructions.push(instruction.clone());
        for reg in instruction.registers() {
            registers.insert(reg, 0);
        }
    }
    // println!("{:?}, {:?}", registers, instructions);
    let mut state = State::new(instructions, registers);
    let mut value = 0;
    while (state.pc as usize) < state.instructions.len() {
        value += match state.instructions[state.pc as usize] {
            Instruction::MulLit(_, _) | Instruction::MulReg(_, _) => 1,
            _ => 0,
        };
        state = state.execute();
    }
    value
}

fn process_data_b() -> i64 {
    let primes = vec![
        105_701, 105_727, 105_733, 105_751, 105_761, 105_767, 105_769, 105_817, 105_829, 105_863,
        105_871, 105_883, 105_899, 105_907, 105_913, 105_929, 105_943, 105_953, 105_967, 105_971,
        105_977, 105_983, 105_997, 106_013, 106_019, 106_031, 106_033, 106_087, 106_103, 106_109,
        106_121, 106_123, 106_129, 106_163, 106_181, 106_187, 106_189, 106_207, 106_213, 106_217,
        106_219, 106_243, 106_261, 106_273, 106_277, 106_279, 106_291, 106_297, 106_303, 106_307,
        106_319, 106_321, 106_331, 106_349, 106_357, 106_363, 106_367, 106_373, 106_391, 106_397,
        106_411, 106_417, 106_427, 106_433, 106_441, 106_451, 106_453, 106_487, 106_501, 106_531,
        106_537, 106_541, 106_543, 106_591, 106_619, 106_621, 106_627, 106_637, 106_649, 106_657,
        106_661, 106_663, 106_669, 106_681, 106_693, 106_699, 106_703, 106_721, 106_727, 106_739,
        106_747, 106_751, 106_753, 106_759, 106_781, 106_783, 106_787, 106_801, 106_823, 106_853,
        106_859, 106_861, 106_867, 106_871, 106_877, 106_903, 106_907, 106_921, 106_937, 106_949,
        106_957, 106_961, 106_963, 106_979, 106_993, 107_021, 107_033, 107_053, 107_057, 107_069,
        107_071, 107_077, 107_089, 107_099, 107_101, 107_119, 107_123, 107_137, 107_171, 107_183,
        107_197, 107_201, 107_209, 107_227, 107_243, 107_251, 107_269, 107_273, 107_279, 107_309,
        107_323, 107_339, 107_347, 107_351, 107_357, 107_377, 107_441, 107_449, 107_453, 107_467,
        107_473, 107_507, 107_509, 107_563, 107_581, 107_599, 107_603, 107_609, 107_621, 107_641,
        107_647, 107_671, 107_687, 107_693, 107_699, 107_713, 107_717, 107_719, 107_741, 107_747,
        107_761, 107_773, 107_777, 107_791, 107_827, 107_837, 107_839, 107_843, 107_857, 107_867,
        107_873, 107_881, 107_897, 107_903, 107_923, 107_927, 107_941, 107_951, 107_971, 107_981,
        107_999, 108_007, 108_011, 108_013, 108_023, 108_037, 108_041, 108_061, 108_079, 108_089,
        108_107, 108_109, 108_127, 108_131, 108_139, 108_161, 108_179, 108_187, 108_191, 108_193,
        108_203, 108_211, 108_217, 108_223, 108_233, 108_247, 108_263, 108_271, 108_287, 108_289,
        108_293, 108_301, 108_343, 108_347, 108_359, 108_377, 108_379, 108_401, 108_413, 108_421,
        108_439, 108_457, 108_461, 108_463, 108_497, 108_499, 108_503, 108_517, 108_529, 108_533,
        108_541, 108_553, 108_557, 108_571, 108_587, 108_631, 108_637, 108_643, 108_649, 108_677,
        108_707, 108_709, 108_727, 108_739, 108_751, 108_761, 108_769, 108_791, 108_793, 108_799,
        108_803, 108_821, 108_827, 108_863, 108_869, 108_877, 108_881, 108_883, 108_887, 108_893,
        108_907, 108_917, 108_923, 108_929, 108_943, 108_947, 108_949, 108_959, 108_961, 108_967,
        108_971, 108_991, 109_001, 109_013, 109_037, 109_049, 109_063, 109_073, 109_097, 109_103,
        109_111, 109_121, 109_133, 109_139, 109_141, 109_147, 109_159, 109_169, 109_171, 109_199,
        109_201, 109_211, 109_229, 109_253, 109_267, 109_279, 109_297, 109_303, 109_313, 109_321,
        109_331, 109_357, 109_363, 109_367, 109_379, 109_387, 109_391, 109_397, 109_423, 109_433,
        109_441, 109_451, 109_453, 109_469, 109_471, 109_481, 109_507, 109_517, 109_519, 109_537,
        109_541, 109_547, 109_567, 109_579, 109_583, 109_589, 109_597, 109_609, 109_619, 109_621,
        109_639, 109_661, 109_663, 109_673, 109_717, 109_721, 109_741, 109_751, 109_789, 109_793,
        109_807, 109_819, 109_829, 109_831, 109_841, 109_843, 109_847, 109_849, 109_859, 109_873,
        109_883, 109_891, 109_897, 109_903, 109_913, 109_919, 109_937, 109_943, 109_961, 109_987,
        110_017, 110_023, 110_039, 110_051, 110_059, 110_063, 110_069, 110_083, 110_119, 110_129,
        110_161, 110_183, 110_221, 110_233, 110_237, 110_251, 110_261, 110_269, 110_273, 110_281,
        110_291, 110_311, 110_321, 110_323, 110_339, 110_359, 110_419, 110_431, 110_437, 110_441,
        110_459, 110_477, 110_479, 110_491, 110_501, 110_503, 110_527, 110_533, 110_543, 110_557,
        110_563, 110_567, 110_569, 110_573, 110_581, 110_587, 110_597, 110_603, 110_609, 110_623,
        110_629, 110_641, 110_647, 110_651, 110_681, 110_711, 110_729, 110_731, 110_749, 110_753,
        110_771, 110_777, 110_807, 110_813, 110_819, 110_821, 110_849, 110_863, 110_879, 110_881,
        110_899, 110_909, 110_917, 110_921, 110_923, 110_927, 110_933, 110_939, 110_947, 110_951,
        110_969, 110_977, 110_989, 111_029, 111_031, 111_043, 111_049, 111_053, 111_091, 111_103,
        111_109, 111_119, 111_121, 111_127, 111_143, 111_149, 111_187, 111_191, 111_211, 111_217,
        111_227, 111_229, 111_253, 111_263, 111_269, 111_271, 111_301, 111_317, 111_323, 111_337,
        111_341, 111_347, 111_373, 111_409, 111_427, 111_431, 111_439, 111_443, 111_467, 111_487,
        111_491, 111_493, 111_497, 111_509, 111_521, 111_533, 111_539, 111_577, 111_581, 111_593,
        111_599, 111_611, 111_623, 111_637, 111_641, 111_653, 111_659, 111_667, 111_697, 111_721,
        111_731, 111_733, 111_751, 111_767, 111_773, 111_779, 111_781, 111_791, 111_799, 111_821,
        111_827, 111_829, 111_833, 111_847, 111_857, 111_863, 111_869, 111_871, 111_893, 111_913,
        111_919, 111_949, 111_953, 111_959, 111_973, 111_977, 111_997, 112_019, 112_031, 112_061,
        112_067, 112_069, 112_087, 112_097, 112_103, 112_111, 112_121, 112_129, 112_139, 112_153,
        112_163, 112_181, 112_199, 112_207, 112_213, 112_223, 112_237, 112_241, 112_247, 112_249,
        112_253, 112_261, 112_279, 112_289, 112_291, 112_297, 112_303, 112_327, 112_331, 112_337,
        112_339, 112_349, 112_361, 112_363, 112_397, 112_403, 112_429, 112_459, 112_481, 112_501,
        112_507, 112_543, 112_559, 112_571, 112_573, 112_577, 112_583, 112_589, 112_601, 112_603,
        112_621, 112_643, 112_657, 112_663, 112_687, 112_691, 112_741, 112_757, 112_759, 112_771,
        112_787, 112_799, 112_807, 112_831, 112_843, 112_859, 112_877, 112_901, 112_909, 112_913,
        112_919, 112_921, 112_927, 112_939, 112_951, 112_967, 112_979, 112_997, 113_011, 113_017,
        113_021, 113_023, 113_027, 113_039, 113_041, 113_051, 113_063, 113_081, 113_083, 113_089,
        113_093, 113_111, 113_117, 113_123, 113_131, 113_143, 113_147, 113_149, 113_153, 113_159,
        113_161, 113_167, 113_171, 113_173, 113_177, 113_189, 113_209, 113_213, 113_227, 113_233,
        113_279, 113_287, 113_327, 113_329, 113_341, 113_357, 113_359, 113_363, 113_371, 113_381,
        113_383, 113_417, 113_437, 113_453, 113_467, 113_489, 113_497, 113_501, 113_513, 113_537,
        113_539, 113_557, 113_567, 113_591, 113_621, 113_623, 113_647, 113_657, 113_683, 113_717,
        113_719, 113_723, 113_731, 113_749, 113_759, 113_761, 113_777, 113_779, 113_783, 113_797,
        113_809, 113_819, 113_837, 113_843, 113_891, 113_899, 113_903, 113_909, 113_921, 113_933,
        113_947, 113_957, 113_963, 113_969, 113_983, 113_989, 114_001, 114_013, 114_031, 114_041,
        114_043, 114_067, 114_073, 114_077, 114_083, 114_089, 114_113, 114_143, 114_157, 114_161,
        114_167, 114_193, 114_197, 114_199, 114_203, 114_217, 114_221, 114_229, 114_259, 114_269,
        114_277, 114_281, 114_299, 114_311, 114_319, 114_329, 114_343, 114_371, 114_377, 114_407,
        114_419, 114_451, 114_467, 114_473, 114_479, 114_487, 114_493, 114_547, 114_553, 114_571,
        114_577, 114_593, 114_599, 114_601, 114_613, 114_617, 114_641, 114_643, 114_649, 114_659,
        114_661, 114_671, 114_679, 114_689, 114_691, 114_713, 114_743, 114_749, 114_757, 114_761,
        114_769, 114_773, 114_781, 114_797, 114_799, 114_809, 114_827, 114_833, 114_847, 114_859,
        114_883, 114_889, 114_901, 114_913, 114_941, 114_967, 114_973, 114_997, 115_001, 115_013,
        115_019, 115_021, 115_057, 115_061, 115_067, 115_079, 115_099, 115_117, 115_123, 115_127,
        115_133, 115_151, 115_153, 115_163, 115_183, 115_201, 115_211, 115_223, 115_237, 115_249,
        115_259, 115_279, 115_301, 115_303, 115_309, 115_319, 115_321, 115_327, 115_331, 115_337,
        115_343, 115_361, 115_363, 115_399, 115_421, 115_429, 115_459, 115_469, 115_471, 115_499,
        115_513, 115_523, 115_547, 115_553, 115_561, 115_571, 115_589, 115_597, 115_601, 115_603,
        115_613, 115_631, 115_637, 115_657, 115_663, 115_679, 115_693, 115_727, 115_733, 115_741,
        115_751, 115_757, 115_763, 115_769, 115_771, 115_777, 115_781, 115_783, 115_793, 115_807,
        115_811, 115_823, 115_831, 115_837, 115_849, 115_853, 115_859, 115_861, 115_873, 115_877,
        115_879, 115_883, 115_891, 115_901, 115_903, 115_931, 115_933, 115_963, 115_979, 115_981,
        115_987, 116_009, 116_027, 116_041, 116_047, 116_089, 116_099, 116_101, 116_107, 116_113,
        116_131, 116_141, 116_159, 116_167, 116_177, 116_189, 116_191, 116_201, 116_239, 116_243,
        116_257, 116_269, 116_273, 116_279, 116_293, 116_329, 116_341, 116_351, 116_359, 116_371,
        116_381, 116_387, 116_411, 116_423, 116_437, 116_443, 116_447, 116_461, 116_471, 116_483,
        116_491, 116_507, 116_531, 116_533, 116_537, 116_539, 116_549, 116_579, 116_593, 116_639,
        116_657, 116_663, 116_681, 116_687, 116_689, 116_707, 116_719, 116_731, 116_741, 116_747,
        116_789, 116_791, 116_797, 116_803, 116_819, 116_827, 116_833, 116_849, 116_867, 116_881,
        116_903, 116_911, 116_923, 116_927, 116_929, 116_933, 116_953, 116_959, 116_969, 116_981,
        116_989, 116_993, 117_017, 117_023, 117_037, 117_041, 117_043, 117_053, 117_071, 117_101,
        117_109, 117_119, 117_127, 117_133, 117_163, 117_167, 117_191, 117_193, 117_203, 117_209,
        117_223, 117_239, 117_241, 117_251, 117_259, 117_269, 117_281, 117_307, 117_319, 117_329,
        117_331, 117_353, 117_361, 117_371, 117_373, 117_389, 117_413, 117_427, 117_431, 117_437,
        117_443, 117_497, 117_499, 117_503, 117_511, 117_517, 117_529, 117_539, 117_541, 117_563,
        117_571, 117_577, 117_617, 117_619, 117_643, 117_659, 117_671, 117_673, 117_679, 117_701,
        117_703, 117_709, 117_721, 117_727, 117_731, 117_751, 117_757, 117_763, 117_773, 117_779,
        117_787, 117_797, 117_809, 117_811, 117_833, 117_839, 117_841, 117_851, 117_877, 117_881,
        117_883, 117_889, 117_899, 117_911, 117_917, 117_937, 117_959, 117_973, 117_977, 117_979,
        117_989, 117_991, 118_033, 118_037, 118_043, 118_051, 118_057, 118_061, 118_081, 118_093,
        118_127, 118_147, 118_163, 118_169, 118_171, 118_189, 118_211, 118_213, 118_219, 118_247,
        118_249, 118_253, 118_259, 118_273, 118_277, 118_297, 118_343, 118_361, 118_369, 118_373,
        118_387, 118_399, 118_409, 118_411, 118_423, 118_429, 118_453, 118_457, 118_463, 118_471,
        118_493, 118_529, 118_543, 118_549, 118_571, 118_583, 118_589, 118_603, 118_619, 118_621,
        118_633, 118_661, 118_669, 118_673, 118_681, 118_687, 118_691, 118_709, 118_717, 118_739,
        118_747, 118_751, 118_757, 118_787, 118_799, 118_801, 118_819, 118_831, 118_843, 118_861,
        118_873, 118_891, 118_897, 118_901, 118_903, 118_907, 118_913, 118_927, 118_931, 118_967,
        118_973, 119_027, 119_033, 119_039, 119_047, 119_057, 119_069, 119_083, 119_087, 119_089,
        119_099, 119_101, 119_107, 119_129, 119_131, 119_159, 119_173, 119_179, 119_183, 119_191,
        119_227, 119_233, 119_237, 119_243, 119_267, 119_291, 119_293, 119_297, 119_299, 119_311,
        119_321, 119_359, 119_363, 119_389, 119_417, 119_419, 119_429, 119_447, 119_489, 119_503,
        119_513, 119_533, 119_549, 119_551, 119_557, 119_563, 119_569, 119_591, 119_611, 119_617,
        119_627, 119_633, 119_653, 119_657, 119_659, 119_671, 119_677, 119_687, 119_689, 119_699,
        119_701, 119_723, 119_737, 119_747, 119_759, 119_771, 119_773, 119_783, 119_797, 119_809,
        119_813, 119_827, 119_831, 119_839, 119_849, 119_851, 119_869, 119_881, 119_891, 119_921,
        119_923, 119_929, 119_953, 119_963, 119_971, 119_981, 119_983, 119_993, 120_011, 120_017,
        120_041, 120_047, 120_049, 120_067, 120_077, 120_079, 120_091, 120_097, 120_103, 120_121,
        120_157, 120_163, 120_167, 120_181, 120_193, 120_199, 120_209, 120_223, 120_233, 120_247,
        120_277, 120_283, 120_293, 120_299, 120_319, 120_331, 120_349, 120_371, 120_383, 120_391,
        120_397, 120_401, 120_413, 120_427, 120_431, 120_473, 120_503, 120_511, 120_539, 120_551,
        120_557, 120_563, 120_569, 120_577, 120_587, 120_607, 120_619, 120_623, 120_641, 120_647,
        120_661, 120_671, 120_677, 120_689, 120_691, 120_709, 120_713, 120_721, 120_737, 120_739,
        120_749, 120_763, 120_767, 120_779, 120_811, 120_817, 120_823, 120_829, 120_833, 120_847,
        120_851, 120_863, 120_871, 120_877, 120_889, 120_899, 120_907, 120_917, 120_919, 120_929,
        120_937, 120_941, 120_943, 120_947, 120_977, 120_997, 121_001, 121_007, 121_013, 121_019,
        121_021, 121_039, 121_061, 121_063, 121_067, 121_081, 121_123, 121_139, 121_151, 121_157,
        121_169, 121_171, 121_181, 121_189, 121_229, 121_259, 121_267, 121_271, 121_283, 121_291,
        121_309, 121_313, 121_321, 121_327, 121_333, 121_343, 121_349, 121_351, 121_357, 121_367,
        121_369, 121_379, 121_403, 121_421, 121_439, 121_441, 121_447, 121_453, 121_469, 121_487,
        121_493, 121_501, 121_507, 121_523, 121_531, 121_547, 121_553, 121_559, 121_571, 121_577,
        121_579, 121_591, 121_607, 121_609, 121_621, 121_631, 121_633, 121_637, 121_661, 121_687,
        121_697, 121_711, 121_721, 121_727, 121_763, 121_787, 121_789, 121_843, 121_853, 121_867,
        121_883, 121_889, 121_909, 121_921, 121_931, 121_937, 121_949, 121_951, 121_963, 121_967,
        121_993, 121_997, 122_011, 122_021, 122_027, 122_029, 122_033, 122_039, 122_041, 122_051,
        122_053, 122_069, 122_081, 122_099, 122_117, 122_131, 122_147, 122_149, 122_167, 122_173,
        122_201, 122_203, 122_207, 122_209, 122_219, 122_231, 122_251, 122_263, 122_267, 122_273,
        122_279, 122_299, 122_321, 122_323, 122_327, 122_347, 122_363, 122_387, 122_389, 122_393,
        122_399, 122_401, 122_443, 122_449, 122_453, 122_471, 122_477, 122_489, 122_497, 122_501,
        122_503, 122_509, 122_527, 122_533, 122_557, 122_561, 122_579, 122_597, 122_599, 122_609,
        122_611, 122_651, 122_653, 122_663, 122_693, 122_701,
    ];

    let mut rv = 0;
    for i in 0..1001 {
        let test = 105_700 + i * 17;
        if !primes.contains(&test) {
            rv += 1;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("23")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b();
        println!("Result = {}", result);
    }
}

#[test]
fn test_a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn test_b() {
    assert_eq!(process_data_b(), 915);
}
