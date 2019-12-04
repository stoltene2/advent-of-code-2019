use core::convert::TryInto;

fn main() {

    let res = (245182..790572).fold(0, |mut total, pw| {
        if meets_criteria_pt1(&num_to_digits(pw)) {
            total += 1;
        }

        total
    });

    println!("Total pws are: {}", res);

    validate_part2();
}

fn validate_part2() {

    println!("==========Part 2============");
    let res = (245182..790572).fold(0, |mut total, pw| {
        if meets_criteria_pt2(&num_to_digits(pw)) {
            total += 1;
        }

        total
    });

    println!("Total pws are: {}", res);
}

fn meets_criteria_pt1(digits: &Vec<u8>) -> bool {
    all_digits_monotone_increasing_p(digits) && has_two_adjacent_digits(digits)
}

fn meets_criteria_pt2(digits: &Vec<u8>) -> bool {
    all_digits_monotone_increasing_p(digits) && has_pair_in_group_of_two_only(digits)
}

fn all_digits_monotone_increasing_p(digits: &Vec<u8>) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(a, b)| a <= b)
}

fn has_two_adjacent_digits(digits: &Vec<u8>) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .any(|(a, b)| a == b)
}

fn has_one_pair_of_adjacent_digits_in_group_of_two_only(digits: &Vec<u8>) -> bool {
    let mut num: u8 = 0;
    let mut count: u8 = 0;

    for n in digits {
        if num == *n {
            count += 1;
        } else {
            if count == 2 {
                return true;
            } else {
                num = *n;
                count = 1;
            }
        }
    }

    // Check if the last digit is part of a pair
    count == 2
}

fn num_to_digits(n: u32) -> Vec<u8> {
    let mut ds: Vec<u8> = Vec::new();
    let mut res = n;

    while res > 0 {
        let digit = res % 10;
        ds.push(digit.try_into().unwrap());
        res = (res - digit)/10;
    }

    ds.reverse();
    ds
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_all_increasing() {
        let digits = vec![1, 2, 3];
        assert_eq!(all_digits_monotone_increasing_p(&digits), true);
    }

    #[test]
    fn test_not_increasing_digit() {
        let digits = vec![1, 2, 1];
        assert_eq!(all_digits_monotone_increasing_p(&digits), false);
    }


    #[test]
    fn test_double_digit() {
        let digits = vec![1, 1, 3];
        assert_eq!(has_two_adjacent_digits(&digits), true);
    }

    #[test]
    fn test_no_double_digit() {
        let digits = vec![1, 2, 3];
        assert_eq!(has_two_adjacent_digits(&digits), false);
    }

    #[test]
    fn test_does_not_meet_criteria() {
        assert_eq!(meets_criteria_pt1(&vec![1, 2, 3]), false);
        assert_eq!(meets_criteria_pt1(&vec![9, 1, 2]), false);
        assert_eq!(meets_criteria_pt1(&vec![9, 1, 1]), false);
    }

    #[test]
    fn test_meets_criteria_pt1() {
        assert_eq!(meets_criteria_pt1(&vec![1, 2, 3, 3]), true);
        assert_eq!(meets_criteria_pt1(&vec![4, 4, 9]), true);
        assert_eq!(meets_criteria_pt1(&vec![1, 2, 3, 9, 9]), true);
    }

    #[test]
    fn test_num_to_digits() {
        assert_eq!(num_to_digits(12345), vec![1, 2, 3, 4, 5]);
        assert_eq!(num_to_digits(11111), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_has_one_pair_of_adjacent_digits_in_group_of_two_only() {
        assert_eq!(has_one_pair_of_adjacent_digits_in_group_of_two_only(&vec![1, 1, 2, 3]), true);
        assert_eq!(has_one_pair_of_adjacent_digits_in_group_of_two_only(&vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test_meets_criteria_pt2() {
        assert_eq!(meets_criteria_pt2(&num_to_digits(112233)), true);
        assert_eq!(meets_criteria_pt2(&num_to_digits(111122)), true);
        assert_eq!(meets_criteria_pt2(&num_to_digits(123444)), false);
        assert_eq!(meets_criteria_pt2(&num_to_digits(444567)), false);
    }

}
