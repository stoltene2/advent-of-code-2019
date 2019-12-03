fn main() {
    println!("Hello, world!");
}

fn find_coordinate(point: (i32, i32), dir: &str) -> Option<(i32, i32)> {
    let direction = dir.get(0..1).unwrap_or("Unknown");
    let points: i32 = dir.get(1..).unwrap_or("0").parse().unwrap();

    match direction {
        "U" => Some((point.0, point.1 + points)),
        "D" => Some((point.0, point.1 - points)),
        "L" => Some((point.0 - points, point.1)),
        "R" => Some((point.0 + points, point.1)),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_generate_coordinate() {
        assert_eq!(find_coordinate((0, 0), "R1"), Some((1, 0)));
        assert_eq!(find_coordinate((0, 0), "U1"), Some((0, 1)));
        assert_eq!(find_coordinate((0, 0), "L1"), Some((-1, 0)));
        assert_eq!(find_coordinate((0, 0), "D1"), Some((0, -1)));

        assert_eq!(find_coordinate((0, 1), "R1"), Some((1, 1)));
        assert_eq!(find_coordinate((1, 0), "U1"), Some((1, 1)));
        assert_eq!(find_coordinate((3, -1), "L1"), Some((2, -1)));
        assert_eq!(find_coordinate((-10, -10), "D1"), Some((-10, -11)));
    }

    }
}
