use std::convert::TryInto;

use permutohedron::heap_recursive;

#[derive(Debug, Eq, PartialEq)]
enum Address {
    Immediate,
    Position,
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
    // TODO: I think I should go back to three addresses. It's just easier
}
impl Instruction {
    fn parse(n: i32) -> Instruction {
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
            99 => Instruction::Halt,
            _ => panic!("Invalid Opcode: {}", n),
        }
    }
}


#[derive(Debug, PartialEq)]
struct ProgState {
    output: Vec<i32>,
    input: Vec<i32>,
    memory: Vec<i32>,
    instruction_pointer: usize,
}

impl ProgState {

    fn new(memory: Vec<i32>, input: Vec<i32>) -> ProgState {
        ProgState {
            memory: memory,
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

fn int_to_address(n: &u8) -> Address {
    if *n == 1_u8 {
        Address::Immediate
    } else {
        Address::Position
    }
}

fn main() {
    let max_thruster_signal = amplifier_circuit(computer_memory());
    println!("max_thruster_signal: {}", max_thruster_signal);

    println!("max feedback signal: {}", feedback_amplifier_circuit(computer_memory()));
}

fn execute_program(mut memory: Vec<i32>, input: Vec<i32>, existing_output: Vec<i32>, ip: usize) -> ProgResult {
    let mut ip: usize = ip.clone();
    let mut output = existing_output.clone();
    let mut input_iter: _ = input.into_iter();

    loop {
        match Instruction::parse(memory[ip]) {
            // Parameters that an instruction writes to will never be in immediate mode.
            Instruction::Add(a1, a2) => {
                let param1 = mem_lookup(&memory, &a1, &(ip + 1));
                let param2 = mem_lookup(&memory, &a2, &(ip + 2));
                let result_addr: usize = memory[ip + 3].try_into().unwrap();

                memory[result_addr] = param1 + param2;
                ip += 4
            }
            Instruction::Multiply(a1, a2) => {
                let param1 = mem_lookup(&memory, &a1, &(ip + 1));
                let param2 = mem_lookup(&memory, &a2, &(ip + 2));
                let result_addr: usize = memory[ip + 3].try_into().unwrap();

                memory[result_addr] = param1 * param2;
                ip += 4
            }
            Instruction::Input => {
                let result_addr: usize = memory[ip + 1].try_into().unwrap();
                match input_iter.next() {
                    Some(result) => {
                        memory[result_addr] = result;
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
                let code = mem_lookup(&memory, &a, &(ip + 1));
                output.push(code);
                ip += 2
            }
            Instruction::JumpIfTrue(a1, a2) => {
                let param1 = mem_lookup(&memory, &a1, &(ip + 1));
                if param1 != 0 {
                    ip = mem_lookup(&memory, &a2, &(ip + 2)).try_into().unwrap();
                } else {
                    ip += 3
                }
            }
            Instruction::JumpIfFalse(a1, a2) => {
                let param1 = mem_lookup(&memory, &a1, &(ip + 1));
                if param1 == 0 {
                    ip = mem_lookup(&memory, &a2, &(ip + 2)).try_into().unwrap();
                } else {
                    ip += 3
                }
            }
            Instruction::LessThan(a1, a2, a3) => {
                let param1 = mem_lookup(&memory, &a1, &(ip + 1));
                let param2 = mem_lookup(&memory, &a2, &(ip + 2));
                // Tests fail if we use positional values here and computer spins forever...
                let result_addr: usize = mem_lookup(&memory, &Address::Immediate, &(ip + 3))
                    .try_into()
                    .unwrap();

                memory[result_addr] = if param1 < param2 { 1 } else { 0 };
                ip += 4
            }
            Instruction::Equals(a1, a2, a3) => {
                let param1 = mem_lookup(&memory, &a1, &(ip + 1));
                let param2 = mem_lookup(&memory, &a2, &(ip + 2));
                let result_addr: usize = mem_lookup(&memory, &Address::Immediate, &(ip + 3))
                    .try_into()
                    .unwrap();

                memory[result_addr] = if param1 == param2 { 1 } else { 0 };
                ip += 4
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

fn amplifier_circuit(memory: Vec<i32>) -> i32 {
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

fn feedback_amplifier_circuit(memory: Vec<i32>) -> i32 {
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

fn feedback_amplifier_circuit_for_phase(memory: Vec<i32>, phase_setting: Vec<i32>) -> i32 {
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

fn mem_lookup(memory: &Vec<i32>, addr_type: &Address, instruction_pointer: &usize) -> i32 {
    match addr_type {
        Address::Immediate => memory[*instruction_pointer],
        Address::Position => {
            let ref_address: usize = memory[*instruction_pointer].try_into().unwrap();
            memory[ref_address]
        }
    }
}

fn num_to_digits_rev(n: i32) -> Vec<u8> {
    let mut ds: Vec<u8> = Vec::new();
    let mut res = n;

    while res > 0 {
        let digit = res % 10;
        ds.push(digit.try_into().unwrap());
        res = (res - digit) / 10;
    }

    ds
}

fn computer_memory() -> Vec<i32> {
    vec![
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
    ]
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
            execute_program(vec![1, 0, 0, 0, 99], vec![1], Vec::new(), 0).unwrap_halt().memory
        );
        assert_eq!(vec![99], execute_program(vec![99], vec![1], Vec::new(), 0).unwrap_halt().memory);
        assert_eq!(
            vec![99, 1, 0, 0, 0],
            execute_program(vec![99, 1, 0, 0, 0], vec![1], Vec::new(), 0).unwrap_halt().memory
        );
        assert_eq!(
            vec![2, 3, 0, 6, 99],
            execute_program(vec![2, 3, 0, 3, 99], vec![1], Vec::new(), 0).unwrap_halt().memory
        );
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            execute_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![1], Vec::new(), 0).unwrap_halt().memory
        );
    }

    #[test]
    fn test_execute_program_with_immediate_values() {
        let r = execute_program(vec![3, 0, 4, 0, 99], vec![1], Vec::new(), 0).unwrap_halt();
        assert_eq!(
            vec![1, 0, 4, 0, 99],
            r.memory
        );
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example1() {
        assert_eq!(
            vec![1101, 100, -1, 4, 99],
            execute_program(vec![1101, 100, -1, 4, 0], vec![1], Vec::new(), 0).unwrap_halt().memory
        );
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example2() {
        assert_eq!(
            vec![1002, 4, 3, 4, 99],
            execute_program(vec![1002, 4, 3, 4, 33], vec![1], Vec::new(), 0).unwrap_halt().memory
        );
    }

    #[test]
    fn test_execute_program_position_input_is_8() {
        let r = execute_program(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_not_8() {
        let r = execute_program(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_less_than_8() {
        let r = execute_program(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![7], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_input_is_greater_than_8() {
        let r = execute_program(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_equal_8() {
        let r = execute_program(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_not_equal_8() {
        let r = execute_program(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r. output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_less_than_8() {
        let r = execute_program(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![7], Vec::new(), 0).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_immediate_input_is_greater_than_8() {
        let r = execute_program(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![9], Vec::new(), 0).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_jump_test_input_nonzero() {
        let r = execute_program(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![9],
            Vec::new(),
            0
        ).unwrap_halt();
        assert_eq!(1, r.output[0]);
    }

    #[test]
    fn test_execute_program_position_jump_test_input_zero() {
        let r = execute_program(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![0],
            Vec::new(),
            0
        ).unwrap_halt();
        assert_eq!(0, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_less_than_8() {
        let r = execute_program(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![5],
            Vec::new(),
            0
        ).unwrap_halt();
        assert_eq!(999, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_equal_8() {
        let r = execute_program(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![8],
            Vec::new(),
            0,
        ).unwrap_halt();
        assert_eq!(1000, r.output[0]);
    }

    #[test]
    fn test_execute_program_largest_example_more_than_8() {
        let r = execute_program(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
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
            amplifier_circuit(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ])
        );
    }

    #[test]
    fn test_amplifier_circuit_example_2() {
        // max signal from sequence 0,1,2,3,4
        assert_eq!(
            54321,
            amplifier_circuit(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ])
        );
    }

    #[test]
    fn test_amplifier_circuit_example_3() {
        // max signal from sequence 1,0,4,3,2
        assert_eq!(
            65210,
            amplifier_circuit(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ])
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
        let s = execute_program(vec![3, 0, 99], Vec::new(), Vec::new(), 0).unwrap_suspend();

        let expected = execute_program(s.memory.clone(), vec![1], s.output.clone(), s.instruction_pointer).unwrap_halt();

        assert_eq!(vec![1, 0, 99], expected.memory);
        assert_eq!(2, expected.instruction_pointer);
    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_with_suspend_example_00() {
        assert_eq!(
            54321,
            feedback_amplifier_circuit_for_phase(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ], vec![0, 1, 2, 3, 4])
        );

    }

    #[test]
    fn test_feedback_amplifier_circuit_for_phase_with_suspend_example_01() {
        assert_eq!(
            139629729,
            feedback_amplifier_circuit_for_phase(vec![
                3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
            ], vec![9, 8, 7, 6, 5])
        );

    }

    #[test]
    fn test_feedback_amplifier_circuit_with_real_input() {
        assert_eq!(
            84088865,
            feedback_amplifier_circuit(computer_memory())
        );
    }
}
