use std::convert::TryInto;

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
    Output,
}

impl Instruction {
    fn parse(n: i32) -> Instruction {

        let digits = num_to_digits_rev(n);

        let intcode = digits.get(1).unwrap_or(&0)*10 + digits.get(0).unwrap();
        let param1 = int_to_address(digits.get(2).unwrap_or(&0));
        let param2 = int_to_address(digits.get(3).unwrap_or(&0));

        match intcode {
            1 => Instruction::Add(param1, param2),
            2 => Instruction::Multiply(param1, param2),
            3 => Instruction::Input,
            4 => Instruction::Output,
            99 => Instruction::Halt,
            _ => panic!("Invalid Opcode: {}", n),
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

fn execute_instruction(memory: &mut Vec<i32>, instruction_pointer: &usize, output: &mut Vec<i32>) -> usize {
    let ip = *instruction_pointer;

    match Instruction::parse(memory[*instruction_pointer]) {
        // Parameters that an instruction writes to will never be in immediate mode.
        Instruction::Add(a1, a2) => {
            let param1 = mem_lookup(memory, &a1, &(ip + 1));
            let param2 = mem_lookup(memory, &a2, &(ip + 2));
            let result_addr: usize = memory[ip + 3].try_into().unwrap();

            memory[result_addr] = param1 + param2;
            4
        },
        Instruction::Multiply(a1, a2) => {
            let param1 = mem_lookup(memory, &a1, &(ip + 1));
            let param2 = mem_lookup(memory, &a2, &(ip + 2));
            let result_addr: usize = memory[ip + 3].try_into().unwrap();

            memory[result_addr] = param1 * param2;
            4
        },
        Instruction::Input => {
            let input = 1;
            let result_addr: usize = memory[ip + 1].try_into().unwrap();
            memory[result_addr] = input;
            2
        },
        Instruction::Output => {
            let code = mem_lookup(memory, &Address::Position, &(ip + 1));
            output.push(code);
            2
        },
        Instruction::Halt => {
            0
        },
    }
}

fn mem_lookup(memory: &Vec<i32>, addr_type: &Address, instruction_pointer: &usize) -> i32 {
    match addr_type {
        Address::Immediate => memory[*instruction_pointer],
        Address::Position  => {
            let ref_address: usize = memory[*instruction_pointer].try_into().unwrap();
            memory[ref_address]
        }
    }
}

fn main() {
    let mut output: Vec<i32> = Vec::new();
    execute_program(computer_memory(), &mut output);
    println!("{:?}", output);
}

fn execute_program(mut memory: Vec<i32>, output: &mut Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer: usize = 0;

    while instruction_pointer < memory.len() && Instruction::parse(memory[instruction_pointer]) != Instruction::Halt {
        instruction_pointer += execute_instruction(&mut memory, &instruction_pointer, output);
    }

    memory
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
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 88, 66, 225, 101, 8, 125, 224, 101,
        -88, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1101, 87, 23,
        225, 1102, 17, 10, 224, 101, -170, 224, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1,
        223, 224, 223, 1101, 9, 65, 225, 1101, 57, 74, 225, 1101, 66, 73, 225, 1101, 22, 37, 224,
        101, -59, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 1, 224, 1, 223, 224, 223, 1102,
        79, 64, 225, 1001, 130, 82, 224, 101, -113, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224,
        7, 224, 1, 223, 224, 223, 1102, 80, 17, 225, 1101, 32, 31, 225, 1, 65, 40, 224, 1001, 224,
        -32, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 224, 223, 223, 2, 99, 69, 224,
        1001, 224, -4503, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 223, 224, 223, 1002,
        14, 92, 224, 1001, 224, -6072, 224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223,
        224, 223, 102, 33, 74, 224, 1001, 224, -2409, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224,
        224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0,
        99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999,
        1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1,
        99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1,
        99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1,
        99999, 107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 329, 101, 1, 223, 223, 108, 677,
        677, 224, 1002, 223, 2, 223, 1005, 224, 344, 101, 1, 223, 223, 1007, 677, 677, 224, 1002,
        223, 2, 223, 1006, 224, 359, 101, 1, 223, 223, 1107, 226, 677, 224, 1002, 223, 2, 223,
        1006, 224, 374, 1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 389,
        101, 1, 223, 223, 1108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 1001, 223, 1,
        223, 7, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223, 1107, 677, 677,
        224, 1002, 223, 2, 223, 1005, 224, 434, 101, 1, 223, 223, 107, 226, 226, 224, 102, 2, 223,
        223, 1005, 224, 449, 101, 1, 223, 223, 107, 677, 226, 224, 1002, 223, 2, 223, 1006, 224,
        464, 1001, 223, 1, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 479, 1001, 223, 1,
        223, 108, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 1108, 677,
        226, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223, 1107, 677, 226, 224, 1002,
        223, 2, 223, 1005, 224, 524, 101, 1, 223, 223, 1008, 226, 226, 224, 1002, 223, 2, 223,
        1006, 224, 539, 101, 1, 223, 223, 1008, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 554,
        1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 569, 101, 1, 223, 223,
        1007, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 584, 1001, 223, 1, 223, 7, 677, 226,
        224, 102, 2, 223, 223, 1006, 224, 599, 101, 1, 223, 223, 1007, 226, 226, 224, 102, 2, 223,
        223, 1006, 224, 614, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224,
        629, 101, 1, 223, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 644, 101, 1, 223,
        223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 659, 101, 1, 223, 223, 8, 226, 226,
        224, 1002, 223, 2, 223, 1005, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
    ]
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryInto;

    #[test]
    fn test_vec_of_usize() {
        let mut v: Vec<usize> = vec![1, 2, 3];
        let p = v[0];
        v[1 + p] = 0;
        assert_eq!(v, vec![1, 2, 0]);
    }

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
        assert_eq!(Instruction::parse(4), Instruction::Output);
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
        let mut output: Vec<i32> = Vec::new();
        assert_eq!(vec![2, 0, 0, 0, 99], execute_program(vec![1, 0, 0, 0, 99], &mut output));
        assert_eq!(vec![99], execute_program(vec![99], &mut output));
        assert_eq!(vec![99, 1, 0, 0, 0], execute_program(vec![99, 1, 0, 0, 0], &mut output));
        assert_eq!(vec![2, 3, 0, 6, 99], execute_program(vec![2, 3, 0, 3, 99], &mut output));
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            execute_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &mut output)
        );
    }

    #[test]
    fn test_execute_program_with_immediate_values() {
        // Inputs hardcoded 1, this should output that 1
        let mut output: Vec<i32> = Vec::new();
        assert_eq!([1, 0, 4, 0, 99], *execute_program(vec![3, 0, 4, 0, 99], &mut output));
        assert_eq!(1, *output.get(0).unwrap());
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example1() {
        // Inputs hardcoded 1, this should output that 1
        let mut output: Vec<i32> = Vec::new();
        assert_eq!([1101, 100, -1, 4, 99], *execute_program(vec![1101, 100, -1, 4, 0], &mut output));
    }

    #[test]
    fn test_execute_program_with_immediate_values_from_example2() {
        // Inputs hardcoded 1, this should output that 1
        let mut output: Vec<i32> = Vec::new();
        assert_eq!([1002, 4, 3, 4, 99], *execute_program(vec![1002, 4, 3, 4, 33], &mut output));
    }


    #[test]
    fn test_execute_instruction_with_position_add() {
        let mut mem: Vec<i32> = vec![1, 0, 0, 0, 99];
        let mut output = Vec::new();

        let ip = execute_instruction(&mut mem, &0, &mut output);

        assert_eq!(ip, 4);
        assert_eq!(mem, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_execute_instruction_with_immediate_add() {
        let mut mem: Vec<i32> = vec![101, 2, 0, 0, 99];
        let mut output = Vec::new();

        let ip = execute_instruction(&mut mem, &0, &mut output);

        assert_eq!(ip, 4);
        assert_eq!(mem, vec![103, 2, 0, 0, 99]);
    }

    #[test]
    fn test_execute_instruction_with_position_multiply() {
        let mut mem: Vec<i32> = vec![2, 0, 0, 0, 99];
        let mut output = Vec::new();

        let ip = execute_instruction(&mut mem, &0, &mut output);

        assert_eq!(ip, 4);
        assert_eq!(mem, vec![4, 0, 0, 0, 99]);
    }

    #[test]
    fn test_execute_instruction_with_immediate_multiply() {
        let mut mem: Vec<i32> = vec![102, 2, 0, 0, 99];
        let mut output = Vec::new();

        let ip = execute_instruction(&mut mem, &0, &mut output);

        assert_eq!(ip, 4);
        assert_eq!(mem, vec![204, 2, 0, 0, 99]);
    }

    #[test]
    fn test_read_input() {
        let mut mem: Vec<i32> = vec![3, 0, 0, 0, 99];
        let mut output = Vec::new();

        let ip = execute_instruction(&mut mem, &0, &mut output);

        assert_eq!(ip, 2);
        assert_eq!(mem, vec![1, 0, 0, 0, 99]);
    }

    #[test]
    fn test_output() {
        let mut mem: Vec<i32> = vec![4, 2, 3, 0, 99];
        let mut output = Vec::new();

        let ip = execute_instruction(&mut mem, &0, &mut output);

        assert_eq!(ip, 2);
        assert_eq!(mem, vec![4, 2, 3, 0, 99]);
        assert_eq!(output, vec![3]);
    }
}
