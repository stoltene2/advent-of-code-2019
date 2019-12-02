fn main() {
    println!("Hello, world!");
}

fn execute_program(mut p: Vec<usize>) -> Vec<usize> {
    let intcode = p[0];
    let fst_ptr = p[1];
    let snd_ptr = p[2];
    let mem_ptr = p[3];
    p[mem_ptr] = p[fst_ptr] + p[snd_ptr];
    p
}

fn input_program() -> Vec<usize> {
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
    fn test_addition_intcode() {
        assert_eq!(execute_program(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(execute_program(vec![99]), vec![99]);
        assert_eq!(execute_program(vec![99, 1, 0, 0, 0]), vec![99, 1, 0, 0, 0]);
        assert_eq!(execute_program(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(execute_program(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}
