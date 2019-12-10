use std::collections::HashMap;
use std::convert::TryInto;

use permutohedron::heap_recursive;

#[derive(Debug, Eq, PartialEq)]
enum Address {
    Immediate,
    Position,
    Relative,
}

#[derive(Debug, Clone, PartialEq)]
struct Memory {
    main: Vec<i64>,
    ext: HashMap<u64, i64>,
    relative_base: i64,
}

// Memory can be moved to its own module
impl Memory {
    fn new() -> Memory {
        Memory {
            main: Vec::new(),
            ext: HashMap::new(),
            relative_base: 0,
        }
    }

    fn from(input: Vec<i64>) -> Memory {
        // Might as well just use one hashmap instead of doing these conversions
        Memory {
            main: input,
            ext: HashMap::new(),
            relative_base: 0,
        }
    }

    fn get(&self, address: u64) -> i64 {
        if (address < self.main.len().try_into().unwrap()) {
            let usize_offset: usize = address.try_into().unwrap();
            self.main[usize_offset].try_into().unwrap()
        } else {
            *self.ext.get(&address).unwrap_or(&0)
        }
    }

    fn set(&mut self, mode: Address, address: i64, value: i64) {
        let relative_base: i64 = self.relative_base.try_into().unwrap();

        let ref_address: u64 = match mode {
            Address::Immediate => address,
            Address::Position => address,
            Address::Relative => address + relative_base,
        }.try_into().unwrap();

        if (ref_address < self.main.len().try_into().unwrap()) {
            let ra: usize = ref_address.try_into().unwrap();
            self.main[ra] = value;
        } else {
            self.ext.insert(ref_address, value);
        }
    }

    // Like a get but uses the Address type
    fn lookup(&self, addr_type: &Address, offset: u64) -> i64 {
        match addr_type {
            Address::Immediate => self.get(offset),
            Address::Position => {
                let position_address: u64 = self.get(offset).try_into().unwrap();
                self.get(position_address)
            }
            Address::Relative => {
                let o = self.get(offset);
                let ref_address: u64 = (o + self.relative_base).try_into().unwrap();
                self.get(ref_address)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Halt,
    Add(Address, Address, Address),
    Multiply(Address, Address, Address),
    Input(Address),
    Output(Address),
    JumpIfTrue(Address, Address),
    JumpIfFalse(Address, Address),
    LessThan(Address, Address, Address),
    Equals(Address, Address, Address),
    AdjustRelativeBase(Address),
}

impl Instruction {
    fn parse(n: i64) -> Instruction {
        let digits = num_to_digits_rev(n);

        let intcode = digits.get(1).unwrap_or(&0) * 10 + digits.get(0).unwrap();
        let param1 = int_to_address(digits.get(2).unwrap_or(&0));
        let param2 = int_to_address(digits.get(3).unwrap_or(&0));
        let param3 = int_to_address(digits.get(4).unwrap_or(&0));

        match intcode {
            1 => Instruction::Add(param1, param2, param3),
            2 => Instruction::Multiply(param1, param2, param3),
            3 => Instruction::Input(param1),
            4 => Instruction::Output(param1),
            5 => Instruction::JumpIfTrue(param1, param2),
            6 => Instruction::JumpIfFalse(param1, param2),
            7 => Instruction::LessThan(param1, param2, param3),
            8 => Instruction::Equals(param1, param2, param3),
            9 => Instruction::AdjustRelativeBase(param1),
            99 => Instruction::Halt,
            _ => panic!("Invalid Opcode: {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ProgState {
    output: Vec<i64>,
    input: Vec<i64>,
    memory: Memory,
    instruction_pointer: u64,
}

impl ProgState {
    fn new(memory: Memory, input: Vec<i64>) -> ProgState {
        ProgState {
            memory,
            instruction_pointer: 0,
            output: Vec::new(),
            input: input,
        }
    }
}

#[derive(Debug, PartialEq)]
enum ProgResult {
    Halt(ProgState),
    Suspend(ProgState),
}

impl ProgResult {
    fn is_halt(&self) -> bool {
        match self {
            ProgResult::Halt(_) => true,
            _ => false,
        }
    }

    fn unwrap_halt(self) -> ProgState {
        match self {
            ProgResult::Halt(state) => state,
            _ => panic!("Found non-Halt value"),
        }
    }

    fn unwrap_suspend(self) -> ProgState {
        match self {
            ProgResult::Suspend(state) => state,
            _ => panic!("Found non-Suspend value"),
        }
    }

    fn unwrap(self) -> ProgState {
        match self {
            ProgResult::Suspend(state) => state,
            ProgResult::Halt(state) => state,
        }
    }
}

// TODO: Make a helper for this in impl Address
fn int_to_address(n: &u8) -> Address {
    match *n {
        0 => Address::Position,
        1 => Address::Immediate,
        2 => Address::Relative,
        _ => panic!("Unknow address type: {}", n),
    }
}

fn main() {
    println!("Program execution: {:?}", execute_program(computer_memory(), vec![1], Vec::new(), 0).unwrap_halt().output);
    println!("Program execution: {:?}", execute_program(computer_memory(), vec![2], Vec::new(), 0).unwrap_halt().output);
}

// Make this take ProgState
fn execute_program(
    mut memory: Memory,
    input: Vec<i64>,
    existing_output: Vec<i64>,
    ip: u64,
) -> ProgResult {
    let mut ip: u64 = ip.clone();
    let mut output = existing_output.clone();
    let mut input_iter: _ = input.into_iter();

    loop {
        match Instruction::parse(memory.get(ip)) {
            // Parameters that an instruction writes to will never be in immediate mode.
            Instruction::Add(a1, a2, a3) => {
                let param1 = memory.lookup(&a1, ip + 1);
                let param2 = memory.lookup(&a2, ip + 2);
                let result_addr: i64 = memory.get(ip + 3).try_into().unwrap();

                memory.set(a3, result_addr, param1 + param2);

                ip += 4
            }
            Instruction::Multiply(a1, a2, a3) => {
                let param1 = memory.lookup(&a1, ip + 1);
                let param2 = memory.lookup(&a2, ip + 2);
                let result_addr: i64 = memory.get(ip + 3).try_into().unwrap();

                memory.set(a3, result_addr, param1 * param2);
                ip += 4
            }
            Instruction::Input(a) => {
                // Get this in position mode and write with parameter mode
                let result_addr: i64 = memory.get(ip + 1).try_into().unwrap();

                match input_iter.next() {
                    Some(result) => {
                        memory.set(a, result_addr, result);
                        ip += 2
                    }
                    None => {
                        // Input not provided stop running program until it is available.
                        return ProgResult::Suspend(ProgState {
                            memory,
                            input: input_iter.collect(),
                            output: output.to_vec(),
                            instruction_pointer: ip,
                        });
                    }
                }
            }
            Instruction::Output(a) => {
                let code = memory.lookup(&a, ip + 1).try_into().unwrap();
                output.push(code);
                ip += 2
            }
            Instruction::JumpIfTrue(a1, a2) => {
                let param1 = memory.lookup(&a1, ip + 1);
                if param1 != 0 {
                    ip = memory.lookup(&a2, ip + 2).try_into().unwrap();
                } else {
                    ip += 3
                }
            }
            Instruction::JumpIfFalse(a1, a2) => {
                let param1 = memory.lookup(&a1, ip + 1);
                if param1 == 0 {
                    ip = memory.lookup(&a2, ip + 2).try_into().unwrap();
                } else {
                    ip += 3
                }
            }
            Instruction::LessThan(a1, a2, a3) => {
                let param1 = memory.lookup(&a1, ip + 1);
                let param2 = memory.lookup(&a2, ip + 2);

                let result_addr: i64 = memory.get(ip + 3).try_into().unwrap();

                memory.set(a3, result_addr, if param1 < param2 { 1 } else { 0 });
                ip += 4
            }
            Instruction::Equals(a1, a2, a3) => {
                let param1 = memory.lookup(&a1, ip + 1);
                let param2 = memory.lookup(&a2, ip + 2);

                let result_addr: i64 = memory.get(ip + 3).try_into().unwrap();

                memory.set(a3, result_addr, if param1 == param2 { 1 } else { 0 });
                ip += 4
            }
            Instruction::AdjustRelativeBase(a1) => {
                let relative_base = memory.lookup(&a1, ip + 1);
                memory.relative_base += relative_base;
                ip += 2
            }
            Instruction::Halt => {
                break;
            }
        }
    }

    ProgResult::Halt(ProgState {
        memory,
        input: input_iter.collect(), // This was the crux. Just
        // returning input will always save inputs even if they are stale.
        output: output.to_vec(),
        instruction_pointer: ip,
    })
}

fn amplifier_circuit(memory: Memory) -> i64 {
    let mut v = vec![0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    let mut max_thruster_signal = 0;

    heap_recursive(&mut v, |permutation| {
        permutations.push(permutation.to_vec())
    });

    for p in permutations {
        // execute program A
        let prog1_input = vec![p[0], 0];
        let prog1_output = Vec::new();
        let a = execute_program(memory.clone(), prog1_input, prog1_output, 0).unwrap_halt();

        // execute program B
        let prog2_input_signal = a.output[0];
        let prog2_input = vec![p[1], prog2_input_signal];
        let prog2_output = Vec::new();
        let b = execute_program(memory.clone(), prog2_input, prog2_output, 0).unwrap_halt();

        // execute program C
        let prog3_input_signal = b.output[0];
        let prog3_input = vec![p[2], prog3_input_signal];
        let prog3_output = Vec::new();
        let c = execute_program(memory.clone(), prog3_input, prog3_output, 0).unwrap_halt();

        // execute program D
        let prog4_input_signal = c.output[0];
        let prog4_input = vec![p[3], prog4_input_signal];
        let prog4_output = Vec::new();
        let d = execute_program(memory.clone(), prog4_input, prog4_output, 0).unwrap_halt();

        // execute program E
        let prog5_input_signal = d.output[0];
        let prog5_input = vec![p[4], prog5_input_signal];
        let prog5_output = Vec::new();
        let e = execute_program(memory.clone(), prog5_input, prog5_output, 0).unwrap_halt();

        // computer max thruster signal

        if e.output[0] > max_thruster_signal {
            max_thruster_signal = e.output[0]
        }
    }

    max_thruster_signal
}

fn feedback_amplifier_circuit(memory: Memory) -> i64 {
    let mut v = vec![5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    let mut max_thruster_signal = 0;

    heap_recursive(&mut v, |permutation| {
        permutations.push(permutation.to_vec())
    });

    for p in permutations {
        let result = feedback_amplifier_circuit_for_phase(memory.clone(), p);

        if result > max_thruster_signal {
            max_thruster_signal = result
        }
    }

    max_thruster_signal
}

fn feedback_amplifier_circuit_for_phase(memory: Memory, phase_setting: Vec<i64>) -> i64 {
    let mut a_state = ProgState::new(memory.clone(), vec![phase_setting[0], 0]);
    let mut b_state = ProgState::new(memory.clone(), vec![phase_setting[1]]);
    let mut c_state = ProgState::new(memory.clone(), vec![phase_setting[2]]);
    let mut d_state = ProgState::new(memory.clone(), vec![phase_setting[3]]);
    let mut e_state = ProgState::new(memory.clone(), vec![phase_setting[4]]);

    loop {
        a_state = execute_program(
            a_state.memory,
            a_state.input,
            a_state.output,
            a_state.instruction_pointer,
        )
        .unwrap();
        b_state.input.append(&mut a_state.output);

        b_state = execute_program(
            b_state.memory,
            b_state.input,
            b_state.output,
            b_state.instruction_pointer,
        )
        .unwrap();
        c_state.input.append(&mut b_state.output);

        c_state = execute_program(
            c_state.memory,
            c_state.input,
            c_state.output,
            c_state.instruction_pointer,
        )
        .unwrap();
        d_state.input.append(&mut c_state.output);

        d_state = execute_program(
            d_state.memory,
            d_state.input,
            d_state.output,
            d_state.instruction_pointer,
        )
        .unwrap();
        e_state.input.append(&mut d_state.output);

        let e_result = execute_program(
            e_state.memory,
            e_state.input,
            e_state.output,
            e_state.instruction_pointer,
        );

        if e_result.is_halt() {
            return e_result.unwrap_halt().output[0];
        }

        e_state = e_result.unwrap_suspend();
        a_state.input.append(&mut e_state.output);
    }
}

fn num_to_digits_rev(n: i64) -> Vec<u8> {
    let mut ds: Vec<u8> = Vec::new();
    let mut res = n;

    while res > 0 {
        let digit = res % 10;
        ds.push(digit.try_into().unwrap());
        res = (res - digit) / 10;
    }

    ds
}

fn computer_memory() -> Memory {
    let input = vec![
        1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1102, 1, 3, 1000, 109,
        988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000,
        2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99,
        4, 17, 104, 0, 99, 0, 0, 1101, 0, 493, 1024, 1102, 1, 38, 1015, 1101, 20, 0, 1011, 1101, 0,
        509, 1026, 1101, 0, 32, 1018, 1101, 0, 333, 1022, 1102, 1, 0, 1020, 1101, 326, 0, 1023,
        1101, 0, 33, 1010, 1101, 21, 0, 1016, 1101, 25, 0, 1004, 1102, 28, 1, 1008, 1102, 1, 506,
        1027, 1102, 488, 1, 1025, 1101, 0, 27, 1013, 1101, 1, 0, 1021, 1101, 0, 34, 1019, 1101,
        607, 0, 1028, 1102, 1, 23, 1003, 1102, 26, 1, 1007, 1102, 29, 1, 1009, 1101, 31, 0, 1000,
        1102, 37, 1, 1012, 1101, 30, 0, 1005, 1101, 602, 0, 1029, 1101, 36, 0, 1002, 1102, 1, 22,
        1001, 1102, 1, 35, 1014, 1102, 24, 1, 1006, 1102, 39, 1, 1017, 109, 4, 21102, 40, 1, 6,
        1008, 1010, 40, 63, 1005, 63, 203, 4, 187, 1106, 0, 207, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, 13, 1206, 3, 221, 4, 213, 1106, 0, 225, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -5,
        1208, -9, 22, 63, 1005, 63, 241, 1106, 0, 247, 4, 231, 1001, 64, 1, 64, 1002, 64, 2, 64,
        109, -5, 21107, 41, 40, 3, 1005, 1010, 263, 1106, 0, 269, 4, 253, 1001, 64, 1, 64, 1002,
        64, 2, 64, 109, -1, 1202, 3, 1, 63, 1008, 63, 29, 63, 1005, 63, 295, 4, 275, 1001, 64, 1,
        64, 1106, 0, 295, 1002, 64, 2, 64, 109, 16, 21108, 42, 42, -8, 1005, 1014, 313, 4, 301,
        1105, 1, 317, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -4, 2105, 1, 5, 1001, 64, 1, 64, 1105,
        1, 335, 4, 323, 1002, 64, 2, 64, 109, -5, 1207, -4, 28, 63, 1005, 63, 355, 1001, 64, 1, 64,
        1105, 1, 357, 4, 341, 1002, 64, 2, 64, 109, 2, 21102, 43, 1, -1, 1008, 1014, 45, 63, 1005,
        63, 377, 1106, 0, 383, 4, 363, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -10, 1208, -3, 36,
        63, 1005, 63, 401, 4, 389, 1106, 0, 405, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 6, 21107,
        44, 45, 1, 1005, 1012, 423, 4, 411, 1105, 1, 427, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 4,
        21101, 45, 0, 3, 1008, 1018, 45, 63, 1005, 63, 453, 4, 433, 1001, 64, 1, 64, 1105, 1, 453,
        1002, 64, 2, 64, 109, -23, 2101, 0, 10, 63, 1008, 63, 36, 63, 1005, 63, 475, 4, 459, 1106,
        0, 479, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 26, 2105, 1, 6, 4, 485, 1105, 1, 497, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 4, 2106, 0, 5, 1105, 1, 515, 4, 503, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, -25, 1201, 10, 0, 63, 1008, 63, 26, 63, 1005, 63, 537, 4, 521, 1105,
        1, 541, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 18, 21101, 46, 0, -1, 1008, 1014, 43, 63,
        1005, 63, 565, 1001, 64, 1, 64, 1106, 0, 567, 4, 547, 1002, 64, 2, 64, 109, -6, 1201, -4,
        0, 63, 1008, 63, 33, 63, 1005, 63, 587, 1105, 1, 593, 4, 573, 1001, 64, 1, 64, 1002, 64, 2,
        64, 109, 22, 2106, 0, -3, 4, 599, 1105, 1, 611, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -28,
        2102, 1, -2, 63, 1008, 63, 22, 63, 1005, 63, 633, 4, 617, 1105, 1, 637, 1001, 64, 1, 64,
        1002, 64, 2, 64, 109, -1, 21108, 47, 44, 9, 1005, 1011, 653, 1105, 1, 659, 4, 643, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 10, 2107, 24, -8, 63, 1005, 63, 681, 4, 665, 1001, 64, 1,
        64, 1105, 1, 681, 1002, 64, 2, 64, 109, -11, 2107, 31, 4, 63, 1005, 63, 697, 1106, 0, 703,
        4, 687, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 8, 2101, 0, -8, 63, 1008, 63, 23, 63, 1005,
        63, 727, 1001, 64, 1, 64, 1105, 1, 729, 4, 709, 1002, 64, 2, 64, 109, -16, 2108, 21, 10,
        63, 1005, 63, 749, 1001, 64, 1, 64, 1106, 0, 751, 4, 735, 1002, 64, 2, 64, 109, 17, 2108,
        36, -8, 63, 1005, 63, 769, 4, 757, 1105, 1, 773, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
        -10, 1207, 1, 23, 63, 1005, 63, 791, 4, 779, 1105, 1, 795, 1001, 64, 1, 64, 1002, 64, 2,
        64, 109, -3, 2102, 1, 6, 63, 1008, 63, 22, 63, 1005, 63, 815, 1106, 0, 821, 4, 801, 1001,
        64, 1, 64, 1002, 64, 2, 64, 109, 16, 1205, 7, 837, 1001, 64, 1, 64, 1105, 1, 839, 4, 827,
        1002, 64, 2, 64, 109, -5, 1202, 0, 1, 63, 1008, 63, 30, 63, 1005, 63, 863, 1001, 64, 1, 64,
        1106, 0, 865, 4, 845, 1002, 64, 2, 64, 109, 4, 1205, 9, 883, 4, 871, 1001, 64, 1, 64, 1106,
        0, 883, 1002, 64, 2, 64, 109, 16, 1206, -7, 899, 1001, 64, 1, 64, 1106, 0, 901, 4, 889, 4,
        64, 99, 21102, 1, 27, 1, 21101, 915, 0, 0, 1105, 1, 922, 21201, 1, 47633, 1, 204, 1, 99,
        109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102, 942, 1, 0, 1105, 1, 922,
        22102, 1, 1, -1, 21201, -2, -3, 1, 21101, 957, 0, 0, 1106, 0, 922, 22201, 1, -1, -2, 1105,
        1, 968, 22101, 0, -2, -2, 109, -3, 2106, 0, 0,
    ];

    Memory::from(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_instruction_parse() {
        assert_eq!(
            Instruction::parse(1),
            Instruction::Add(Address::Position, Address::Position, Address::Position)
        );

        assert_eq!(
            Instruction::parse(2),
            Instruction::Multiply(Address::Position, Address::Position, Address::Position)
        );
        assert_eq!(Instruction::parse(3), Instruction::Input(Address::Position));
        assert_eq!(
            Instruction::parse(4),
            Instruction::Output(Address::Position)
        );
        assert_eq!(Instruction::parse(99), Instruction::Halt);

        assert_eq!(
            Instruction::parse(11101),
            Instruction::Add(Address::Immediate, Address::Immediate, Address::Immediate)
        );

        assert_eq!(
            Instruction::parse(1101),
            Instruction::Add(Address::Immediate, Address::Immediate, Address::Position)
        );

        assert_eq!(
            Instruction::parse(11102),
            Instruction::Multiply(Address::Immediate, Address::Immediate, Address::Immediate)
        );
    }

    #[test]
    fn test_addition_instruction() {
        assert_eq!(
            vec![2, 0, 0, 0, 99],
            execute_program(Memory::from(vec![1, 0, 0, 0, 99]), vec![1], Vec::new(), 0)
                .unwrap_halt()
                .memory
                .main
        );
        assert_eq!(
            vec![99],
            execute_program(Memory::from(vec![99]), vec![1], Vec::new(), 0)
                .unwrap_halt()
                .memory
                .main
        );
        assert_eq!(
            vec![99, 1, 0, 0, 0],
            execute_program(Memory::from(vec![99, 1, 0, 0, 0]), vec![1], Vec::new(), 0)
                .unwrap_halt()
                .memory
                .main
        );
        assert_eq!(
            vec![2, 3, 0, 6, 99],
            execute_program(Memory::from(vec![2, 3, 0, 3, 99]), vec![1], Vec::new(), 0)
                .unwrap_halt()
                .memory
                .main
        );
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            execute_program(
                Memory::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
                vec![1],
                Vec::new(),
                0
            )
            .unwrap_halt()
            .memory
            .main
        );
    }

    #[test]
    fn test_execute_program_with_immediate_values() {
        let r = execute_program(Memory::from(vec![3, 0, 4, 0, 99]), vec![1], Vec::new(), 0)
            .unwrap_halt();
        assert_eq!(vec![1, 0, 4, 0, 99], r.memory.main);
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example1() {
        assert_eq!(
            vec![1101, 100, -1, 4, 99],
            execute_program(
                Memory::from(vec![1101, 100, -1, 4, 0]),
                vec![1],
                Vec::new(),
                0
            )
            .unwrap_halt()
            .memory
            .main
        );
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example2() {
        assert_eq!(
            vec![1002, 4, 3, 4, 99],
            execute_program(
                Memory::from(vec![1002, 4, 3, 4, 33]),
                vec![1],
                Vec::new(),
                0
            )
            .unwrap_halt()
            .memory
            .main
        );
    }

    #[test]
    fn test_execute_program_position_input_is_8() {
        let r = execute_program(
            Memory::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]),
            vec![8],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_not_8() {
        let r = execute_program(
            Memory::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]),
            vec![9],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_less_than_8() {
        let r = execute_program(
            Memory::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]),
            vec![7],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_greater_than_8() {
        let r = execute_program(
            Memory::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]),
            vec![9],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_equal_8() {
        let r = execute_program(
            Memory::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]),
            vec![8],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_not_equal_8() {
        let r = execute_program(
            Memory::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]),
            vec![9],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_less_than_8() {
        let r = execute_program(
            Memory::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]),
            vec![7],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_greater_than_8() {
        let r = execute_program(
            Memory::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]),
            vec![9],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_jump_test_input_nonzero() {
        let r = execute_program(
            Memory::from(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]),
            vec![9],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_jump_test_input_zero() {
        let r = execute_program(
            Memory::from(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ]),
            vec![0],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_less_than_8() {
        let r = execute_program(
            Memory::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            vec![5],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(999, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_equal_8() {
        let r = execute_program(
            Memory::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            vec![8],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(1000, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_more_than_8() {
        let r = execute_program(
            Memory::from(vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]),
            vec![10],
            Vec::new(),
            0,
        )
        .unwrap_halt();
        assert_eq!(1001, r.output[0]);
    }

    #[test]
    fn test_amplifier_circuit_example_1() {
        // max signal from sequence 4,3,2,1,0
        assert_eq!(
            43210,
            amplifier_circuit(Memory::from(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]))
        );
    }

    #[test]
    fn test_amplifier_circuit_example_2() {
        // max signal from sequence 0,1,2,3,4
        assert_eq!(
            54321,
            amplifier_circuit(Memory::from(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]))
        );
    }

    #[test]
    fn test_amplifier_circuit_example_3() {
        // max signal from sequence 1,0,4,3,2
        assert_eq!(
            65210,
            amplifier_circuit(Memory::from(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]))
        );
    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_suspend_waiting_for_input() {
        let s = execute_program(Memory::from(vec![3, 0, 99]), Vec::new(), Vec::new(), 0)
            .unwrap_suspend();

        let expected = execute_program(
            Memory::from(s.memory.main.clone()),
            vec![1],
            s.output.clone(),
            s.instruction_pointer,
        )
        .unwrap_halt();

        assert_eq!(vec![1, 0, 99], expected.memory.main);
        assert_eq!(2, expected.instruction_pointer);
    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_with_suspend_example_00() {
        assert_eq!(
            54321,
            feedback_amplifier_circuit_for_phase(
                Memory::from(vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ]),
                vec![0, 1, 2, 3, 4]
            )
        );
    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_with_suspend_example_01() {
        assert_eq!(
            139629729,
            feedback_amplifier_circuit_for_phase(
                Memory::from(vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ]),
                vec![9, 8, 7, 6, 5]
            )
        );
    }

    #[test]
    fn test_write_and_read_from_extended_memory() {
        // Writes 2 to address 1000 and then reads it back
        let mut mem = Memory::from(vec![101, 1, 1, 1000, 99]);

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(2, output_state.memory.get(1000));
    }

    #[test]
    fn test_read_from_extended_memory_returns_0_if_not_set() {
        let mut mem = Memory::from(vec![4, 10000, 99]);

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(0, output_state.output[0]);
    }

    #[test]
    fn test_relative_base_opcode_example_01() {
        // Quine example
        let input: Vec<i64> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut mem = Memory::from(input.clone());

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(input, output_state.output);
    }

    // This test is the reason I need to use 64 bit signed integers
    #[test]
    fn test_relative_base_opcode_example_02_outputs_16_digit_number() {
        let input: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut mem = Memory::from(input.clone());

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(1219070632396864, output_state.output[0]);
    }

    #[test]
    fn test_relative_base_opcode_example_03_outputs_16_digit_number_from_second_param() {
        let input: Vec<i64> = vec![104, 1125899906842624, 99];
        let mut mem = Memory::from(input.clone());

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(input[1], output_state.output[0]);
    }

    #[test]
    fn test_part_1_of_day_09() {
        assert_eq!(3345854957, execute_program(computer_memory(), vec![1], Vec::new(), 0).unwrap_halt().output[0]);
    }

}
