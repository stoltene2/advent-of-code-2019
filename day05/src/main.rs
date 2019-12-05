use std::convert::TryInto;

fn main() {
    println!("TODO: Diagnostic Code: {:?}", 0);
}

fn execute_program(mut memory: Vec<i32>) -> Vec<i32> {
    const HALT: u8 = 99;
    const ADD: u8 = 1;
    const MUL: u8 = 2;

    for instruction_ptr in (0..memory.len()).step_by(4) {
        let ip: usize = instruction_ptr.try_into().unwrap();
        let instruction = memory[instruction_ptr].try_into().unwrap();

        if instruction == HALT {
            break;
        }

        let param_1_address: usize = memory[ip + 1].try_into().unwrap();
        let param_2_address: usize = memory[ip + 2].try_into().unwrap();
        let result_address: usize = memory[ip + 3].try_into().unwrap();

        match instruction {
            ADD => {
                memory[result_address] = memory[param_1_address] + memory[param_2_address];
            }
            MUL => {
                memory[result_address] = memory[param_1_address] * memory[param_2_address];
            }
            _ => (),
        }
    }

    memory
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
    fn test_addition_instruction() {
        assert_eq!(vec![2, 0, 0, 0, 99], execute_program(vec![1, 0, 0, 0, 99]));
        assert_eq!(vec![99], execute_program(vec![99]));
        assert_eq!(vec![99, 1, 0, 0, 0], execute_program(vec![99, 1, 0, 0, 0]));
        assert_eq!(vec![2, 3, 0, 6, 99], execute_program(vec![2, 3, 0, 3, 99]));
        assert_eq!(
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            execute_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])
        );
    }

    #[test]
    fn test_cast_i32_to_usize() {
        let i: i32 = 10;
        let mut u: usize = i.try_into().unwrap();
    }
}
