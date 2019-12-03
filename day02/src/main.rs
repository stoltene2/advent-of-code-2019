fn main() {
    let mut memory = computer_memory();

    // Repair ship according to instructions
    memory[1] = 12;
    memory[2] = 2;

    let first_intcode = execute_program(memory)[0];
    println!("Resulting intcode: {}", first_intcode);

    println!("Part 2 intcode: {}", find_magic());
}

fn find_magic() -> usize {
    const DESIRED_OUTPUT: usize = 19_690_720;
    let mut result: usize = 0;

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = computer_memory();

            memory[1] = noun.into();
            memory[2] = verb.into();

            let first_intcode = execute_program(memory)[0];

            if first_intcode == DESIRED_OUTPUT {
                result = 100 * noun + verb;
                break 'outer;
            }
        }
    }

    result
}

fn execute_program(mut memory: Vec<usize>) -> Vec<usize> {
    const HALT: usize = 99;
    const ADD: usize = 1;
    const MUL: usize = 2;

    for instruction_ptr in (0..memory.len()).step_by(4) {
        let instruction = memory[instruction_ptr];

        if instruction == HALT { break; }

        let param_1_address = memory[instruction_ptr + 1];
        let param_2_address = memory[instruction_ptr + 2];
        let result_address  = memory[instruction_ptr + 3];

        match instruction {
            ADD => {
                memory[result_address] = memory[param_1_address] + memory[param_2_address];
            },
            MUL => {
                memory[result_address] = memory[param_1_address] * memory[param_2_address];
            },
            _ => ()
        }
    }

    memory
}

fn computer_memory() -> Vec<usize> {
    vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 5, 19, 23, 1, 23, 6, 27, 1,
        5, 27, 31, 1, 31, 6, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 43, 6, 47, 2, 6, 47, 51, 1, 5, 51,
        55, 1, 55, 13, 59, 1, 59, 10, 63, 2, 10, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75, 79,
        2, 79, 13, 83, 1, 83, 5, 87, 1, 87, 9, 91, 1, 5, 91, 95, 1, 5, 95, 99, 1, 99, 13, 103, 1,
        10, 103, 107, 1, 107, 9, 111, 1, 6, 111, 115, 2, 115, 13, 119, 1, 10, 119, 123, 2, 123, 6,
        127, 1, 5, 127, 131, 1, 5, 131, 135, 1, 135, 6, 139, 2, 139, 10, 143, 2, 143, 9, 147, 1,
        147, 6, 151, 1, 151, 13, 155, 2, 155, 9, 159, 1, 6, 159, 163, 1, 5, 163, 167, 1, 5, 167,
        171, 1, 10, 171, 175, 1, 13, 175, 179, 1, 179, 2, 183, 1, 9, 183, 0, 99, 2, 14, 0, 0,
    ]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_vector_operations() {
        assert_eq!(vec![1, 2, 3], vec![1, 2, 3]);

        let mut v = vec![1, 2, 3];
        v[0] = 0;

        assert_eq!(v, vec![0, 2, 3]);
    }

    #[test]
    fn test_vec_of_usize() {
        let mut v: Vec<usize> = vec![1, 2, 3];
        let p = v[0];
        v[1 + p] = 0;
        assert_eq!(v, vec![1, 2, 0]);
    }

    #[test]
    fn test_addition_instruction() {
        assert_eq!(vec![2,0,0,0,99], execute_program(vec![1,0,0,0,99]));
        assert_eq!(vec![99], execute_program(vec![99]));
        assert_eq!(vec![99, 1, 0, 0, 0], execute_program(vec![99, 1, 0, 0, 0]));
        assert_eq!(vec![2,3,0,6,99], execute_program(vec![2,3,0,3,99]));
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], execute_program(vec![1,1,1,4,99,5,6,0,99]));
    }

    #[test]
    fn test_break_out_of_double_loop() {
        let mut x = 0;
        let mut y = 0;

        'outer: for i in 1..10 {
            for j in 1..10 {
                if i == 5 && j == 5 {
                    x = 5;
                    y = 5;
                    break 'outer;
                }
            }
        }

        assert_eq!((x, y), (5, 5));
    }
}
