use std::convert::TryInto;
use std::collections::HashMap;

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
    ext: HashMap<usize, i64>,
    relative_base: i64, // Store this on the memory instead of the prog state
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
        Memory {
            main: input,
            ext: HashMap::new(),
            relative_base: 0,
        }
    }

    // TODO: Should have a variant for getting a usize back
    fn get(&self, address: usize) -> i64 {
        if (address < self.main.len()) {
            self.main[address].try_into().unwrap()
        } else {
            *self.ext.get(&address).unwrap_or(&0)
        }
    }

    fn set(&mut self, address: usize, value: i64) {
        if (address < self.main.len()) {
            self.main[address] = value;
        } else {
            self.ext.insert(address, value);
        }
    }

    // Like a get but uses the Address type
    fn lookup(&self, addr_type: &Address, offset: usize) -> i64 {
        match addr_type {
            Address::Immediate => self.get(offset),
            Address::Position => {
                let position_address: usize = self.get(offset).try_into().unwrap();
                self.get(position_address)
            },
            Address::Relative => {
                let o = self.get(offset);
                let ref_address: usize = (o + self.relative_base).try_into().unwrap();
                self.get(ref_address)
            }
        }
    }

}

#[derive(Debug, PartialEq)]
enum Instruction {
    Halt,
    Add(Address, Address),
    Multiply(Address, Address),
    Input,
    Output(Address),
    JumpIfTrue(Address, Address),
    JumpIfFalse(Address, Address),
    LessThan(Address, Address, Address),
    Equals(Address, Address, Address),
    AdjustRelativeBase(Address),
    // TODO: I think I should go back to three addresses. It's just easier
}

impl Instruction {
    fn parse(n: i64) -> Instruction {
        let digits = num_to_digits_rev(n);

        let intcode = digits.get(1).unwrap_or(&0) * 10 + digits.get(0).unwrap();
        let param1 = int_to_address(digits.get(2).unwrap_or(&0));
        let param2 = int_to_address(digits.get(3).unwrap_or(&0));
        let param3 = int_to_address(digits.get(4).unwrap_or(&0));

        match intcode {
            1 => Instruction::Add(param1, param2),
            2 => Instruction::Multiply(param1, param2),
            3 => Instruction::Input,
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
    instruction_pointer: usize,
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
        _ => panic!("Unknow address type: {}", n)
    }
}

fn main() {
    let max_thruster_signal = amplifier_circuit(computer_memory());
    println!("max_thruster_signal: {}", max_thruster_signal);

    println!("max feedback signal: {}", feedback_amplifier_circuit(computer_memory()));
}

// Make this take ProgState
fn execute_program(mut memory: Memory, input: Vec<i64>, existing_output: Vec<i64>, ip: usize) -> ProgResult {
    let mut ip: usize = ip.clone();
    let mut output = existing_output.clone();
    let mut input_iter: _ = input.into_iter();

    loop {
        match Instruction::parse(memory.get(ip)) {
            // Parameters that an instruction writes to will never be in immediate mode.
            Instruction::Add(a1, a2) => {
                let param1 = memory.lookup(&a1, ip + 1);
                let param2 = memory.lookup(&a2, ip + 2);
                let result_addr: usize = memory.get(ip + 3).try_into().unwrap();

                memory.set(result_addr, param1 + param2);

                ip += 4
            }
            Instruction::Multiply(a1, a2) => {
                let param1 = memory.lookup(&a1, ip + 1);
                let param2 = memory.lookup(&a2, ip + 2);
                let result_addr: usize = memory.get(ip + 3).try_into().unwrap();

                memory.set(result_addr, param1 * param2);
                ip += 4
            }
            Instruction::Input => {
                let result_addr: usize = memory.get(ip + 1).try_into().unwrap();
                match input_iter.next() {
                    Some(result) => {
                        memory.set(result_addr, result) ;
                        ip += 2
                    },
                    None => {
                        // Input not provided stop running program until it is available.
                        return ProgResult::Suspend( ProgState {
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
                // Tests fail if we use positional values here and computer spins forever...
                let result_addr: usize = memory.get(ip + 3).try_into().unwrap();

                memory.set(result_addr, if param1 < param2 { 1 } else { 0 });
                ip += 4
            }
            Instruction::Equals(a1, a2, a3) => {
                let param1 = memory.lookup(&a1, ip + 1);
                let param2 = memory.lookup(&a2, ip + 2);
                let result_addr: usize = memory.get(ip + 3).try_into().unwrap();

                memory.set(result_addr, if param1 == param2 { 1 } else { 0 });
                ip += 4
            }
            Instruction::AdjustRelativeBase(a1) => {
                // Assume for now that it just has a value and it isn't using other input types...
                let relative_base = memory.lookup(&a1, ip + 1);
                memory.relative_base += relative_base;
                ip += 2
            }
            Instruction::Halt => {
                break;
            }
        }
    }

    ProgResult::Halt( ProgState {
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
        a_state = execute_program(a_state.memory, a_state.input, a_state.output, a_state.instruction_pointer).unwrap();
        b_state.input.append(&mut a_state.output);

        b_state = execute_program(b_state.memory, b_state.input, b_state.output, b_state.instruction_pointer).unwrap();
        c_state.input.append(&mut b_state.output);

        c_state = execute_program(c_state.memory, c_state.input, c_state.output, c_state.instruction_pointer).unwrap();
        d_state.input.append(&mut c_state.output);

        d_state = execute_program(d_state.memory, d_state.input, d_state.output, d_state.instruction_pointer).unwrap();
        e_state.input.append(&mut d_state.output);

        let e_result = execute_program(e_state.memory, e_state.input, e_state.output, e_state.instruction_pointer);

        if e_result.is_halt() {
            return e_result.unwrap_halt().output[0]
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
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 55, 68, 85, 106, 187, 268, 349, 430, 99999, 3,
        9, 1001, 9, 5, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 1001, 9, 2, 9, 1002, 9, 5,
        9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 102, 3, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 5,
        9, 101, 3, 9, 9, 102, 5, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 4, 9, 1001, 9, 2, 9, 102, 3, 9, 9,
        101, 3, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1,
        9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
        3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001,
        9, 1, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9,
        4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2,
        9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
        99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9,
        4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
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
            Instruction::Add(Address::Position, Address::Position)
        );

        assert_eq!(
            Instruction::parse(2),
            Instruction::Multiply(Address::Position, Address::Position)
        );
        assert_eq!(Instruction::parse(3), Instruction::Input);
        assert_eq!(Instruction::parse(4), Instruction::Output(Address::Position));
        assert_eq!(Instruction::parse(99), Instruction::Halt);

        assert_eq!(
            Instruction::parse(11101),
            Instruction::Add(Address::Immediate, Address::Immediate)
        );

        assert_eq!(
            Instruction::parse(1101),
            Instruction::Add(Address::Immediate, Address::Immediate)
        );

        assert_eq!(
            Instruction::parse(11102),
            Instruction::Multiply(Address::Immediate, Address::Immediate)
        );
    }

    #[test]
    fn test_addition_instruction() {
        assert_eq!(
            vec![2, 0, 0, 0, 99],
            execute_program(Memory::from(vec![1, 0, 0, 0, 99]), vec![1], Vec::new(), 0).unwrap_halt().memory.main
        );
        assert_eq!(vec![99], execute_program(Memory::from(vec![99]), vec![1], Vec::new(), 0).unwrap_halt().memory.main);
        assert_eq!(
            vec![99, 1, 0, 0, 0],
            execute_program(Memory::from(vec![99, 1, 0, 0, 0]), vec![1], Vec::new(), 0).unwrap_halt().memory.main
        );
        assert_eq!(
            vec![2, 3, 0, 6, 99],
            execute_program(Memory::from(vec![2, 3, 0, 3, 99]), vec![1], Vec::new(), 0).unwrap_halt().memory.main
        );
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            execute_program(Memory::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), vec![1], Vec::new(), 0).unwrap_halt().memory.main
        );
    }

    #[test]
    fn test_execute_program_with_immediate_values() {
        let r = execute_program(Memory::from(vec![3, 0, 4, 0, 99]), vec![1], Vec::new(), 0).unwrap_halt();
        assert_eq!(
            vec![1, 0, 4, 0, 99],
            r.memory.main
        );
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example1() {
        assert_eq!(
            vec![1101, 100, -1, 4, 99],
            execute_program(Memory::from(vec![1101, 100, -1, 4, 0]), vec![1], Vec::new(), 0).unwrap_halt().memory.main
        );
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example2() {
        assert_eq!(
            vec![1002, 4, 3, 4, 99],
            execute_program(Memory::from(vec![1002, 4, 3, 4, 33]), vec![1], Vec::new(), 0).unwrap_halt().memory.main
        );
    }

    #[test]
    fn test_execute_program_position_input_is_8() {
        let r = execute_program(Memory::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]), vec![8], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_not_8() {
        let r = execute_program(Memory::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]), vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_less_than_8() {
        let r = execute_program(Memory::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]), vec![7], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_greater_than_8() {
        let r = execute_program(Memory::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]), vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_equal_8() {
        let r = execute_program(Memory::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]), vec![8], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_not_equal_8() {
        let r = execute_program(Memory::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]), vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r. output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_less_than_8() {
        let r = execute_program(Memory::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]), vec![7], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_greater_than_8() {
        let r = execute_program(Memory::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]), vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_jump_test_input_nonzero() {
        let r = execute_program(
            Memory::from(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
            ),
            vec![9],
            Vec::new(),
            0
        ).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_jump_test_input_zero() {
        let r = execute_program(
            Memory::from(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
            ),
            vec![0],
            Vec::new(),
            0
        ).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_less_than_8() {
        let r = execute_program(
            Memory::from(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                    98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                    1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ]),
            vec![5],
            Vec::new(),
            0
        ).unwrap_halt();
        assert_eq!(999, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_equal_8() {
        let r = execute_program(
            Memory::from(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                    98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                    1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ]),
            vec![8],
            Vec::new(),
            0,
        ).unwrap_halt();
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
            0
        ).unwrap_halt();
        assert_eq!(1001, r.output[0]);
    }

    #[test]
    fn test_amplifier_circuit_example_1() {
        // max signal from sequence 4,3,2,1,0
        assert_eq!(
            43210,
            amplifier_circuit(Memory::from(
                vec![
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
                ])
            )
        );
    }

    #[test]
    fn test_amplifier_circuit_example_2() {
        // max signal from sequence 0,1,2,3,4
        assert_eq!(
            54321,
            amplifier_circuit(Memory::from(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                    23, 99, 0, 0
                ])
            )
        );
    }

    #[test]
    fn test_amplifier_circuit_example_3() {
        // max signal from sequence 1,0,4,3,2
        assert_eq!(
            65210,
            amplifier_circuit(Memory::from(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                    1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ])
            )
        );
    }

    #[test]
    fn test_amplifier_circuit_with_real_input() {
        assert_eq!(
            255840,
            amplifier_circuit(computer_memory())
        );
    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_suspend_waiting_for_input() {
        let s = execute_program(Memory::from(vec![3, 0, 99]), Vec::new(), Vec::new(), 0).unwrap_suspend();

        let expected = execute_program(Memory::from(s.memory.main.clone()), vec![1], s.output.clone(), s.instruction_pointer).unwrap_halt();

        assert_eq!(vec![1, 0, 99], expected.memory.main);
        assert_eq!(2, expected.instruction_pointer);
    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_with_suspend_example_00() {
        assert_eq!(
            54321,
            feedback_amplifier_circuit_for_phase(
                Memory::from(
                    vec![
                        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                        23, 99, 0, 0
                    ]),
                vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_with_suspend_example_01() {
        assert_eq!(
            139629729,
            feedback_amplifier_circuit_for_phase(
                Memory::from(vec![
                    3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
                ]),
                vec![9, 8, 7, 6, 5])
        );

    }

    #[test]
    fn test_feedback_amplifier_circuit_with_real_input() {
        assert_eq!(
            84088865,
            feedback_amplifier_circuit(computer_memory())
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
        let input: Vec<i64> = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut mem = Memory::from(input.clone());

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(input, output_state.output);
    }

    // This test is the reason I need to use 64 bit signed integers
    #[test]
    fn test_relative_base_opcode_example_02_outputs_16_digit_number() {
        let input: Vec<i64> = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut mem = Memory::from(input.clone());

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(1219070632396864, output_state.output[0]);
    }


    #[test]
    fn test_relative_base_opcode_example_03_outputs_16_digit_number_from_second_param() {
        let input: Vec<i64> = vec![104,1125899906842624,99];
        let mut mem = Memory::from(input.clone());

        let output_state = execute_program(mem, Vec::new(), Vec::new(), 0).unwrap_halt();

        assert_eq!(input[1], output_state.output[0]);
    }

}
