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

fn gen_wire_coordinates(diagram: Vec<&str>) -> Vec<(i32, i32)> {
    let mut wires: Vec<(i32, i32)> = Vec::new();
    wires.push((0, 0));

    diagram.iter().fold(wires, |mut w, dir| {
        let coord = w.last().unwrap_or(&(0, 0));
        match find_coordinate(*coord, dir) {
            Some(next) => {
                w.push(next);
                w
            },
            _ => w
        }
    })
}

// TODO: I might be able to just return an iterator here
fn gen_wire_segments(coordinates: Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    let c2 = coordinates.clone();

    coordinates
        .into_iter()
        .zip(c2.into_iter().skip(1))
        .collect()
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

    #[test]
    fn test_gen_wire_coordinates() {
        assert_eq!(gen_wire_coordinates(vec!["R1", "U1", "L1", "D1"]), vec![(0, 0), (1, 0), (1, 1), (0, 1), (0, 0)]);
        assert_eq!(gen_wire_coordinates("R1,U1,L1,D1".split(",").collect()), vec![(0, 0), (1, 0), (1, 1), (0, 1), (0, 0)]);
    }

    #[test]
    fn test_gen_wire_segments() {
        assert_eq!(gen_wire_segments(vec![(0, 0), (0, 1)]), vec![((0, 0), (0, 1))]);
        assert_eq!(gen_wire_segments(vec![(0, 0), (0, 1), (1, 1)]),
                   vec![
                       ((0, 0), (0, 1)),
                       ((0, 1), (1, 1)),
                   ]);
    }
}
