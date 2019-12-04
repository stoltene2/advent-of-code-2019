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
        _ => None,
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
            }
            _ => w,
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

fn find_intersection(
    (s1p1, s1p2): ((i32, i32), (i32, i32)),
    (s2p1, s2p2): ((i32, i32), (i32, i32)),
) -> Option<(i32, i32)> {

    let ((s1_x1, s1_y1), (s1_x2, s1_y2)) = order_points(s1p1, s1p2);
    let ((s2_x1, s2_y1), (s2_x2, s2_y2)) = order_points(s2p1, s2p2);

    if (s2_x1 <= s1_x1 && s1_x1 <= s2_x2) && (s1_y1 <= s2_y1 && s2_y1 <= s1_y2) {
        Some((s1_x1, s2_y1))
    } else if (s1_x1 <= s2_x1 && s2_x1 <= s1_x2)  && (s1_y1 <= s1_y1 && s1_y1 <= s1_y2 ) {
        Some((s2_x1, s1_y1))
    }
    else {
        None
    }
}

fn order_points(p1: (i32, i32), p2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    if p1.0 != p2.0 {
        if p1.0 < p2.0 {
            (p1, p2)
        } else {
            (p2, p1)
        }
    } else {
        if p1.1 < p2.1 {
            (p1, p2)
        } else {
            (p2, p1)
        }
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

    #[test]
    fn test_gen_wire_coordinates() {
        assert_eq!(
            gen_wire_coordinates(vec!["R1", "U1", "L1", "D1"]),
            vec![(0, 0), (1, 0), (1, 1), (0, 1), (0, 0)]
        );
        assert_eq!(
            gen_wire_coordinates("R1,U1,L1,D1".split(",").collect()),
            vec![(0, 0), (1, 0), (1, 1), (0, 1), (0, 0)]
        );
    }

    #[test]
    fn test_gen_wire_segments() {
        assert_eq!(
            gen_wire_segments(vec![(0, 0), (0, 1)]),
            vec![((0, 0), (0, 1))]
        );
        assert_eq!(
            gen_wire_segments(vec![(0, 0), (0, 1), (1, 1)]),
            vec![((0, 0), (0, 1)), ((0, 1), (1, 1)),]
        );
    }

    #[test]
    fn test_order_points() {
        assert_eq!(order_points((0, 1), (1, 1)), ((0, 1), (1, 1)));
        assert_eq!(order_points((1, 1), (0, 1)), ((0, 1), (1, 1)));

        assert_eq!(order_points((0, 0), (0, 1)), ((0, 0), (0, 1)));
        assert_eq!(order_points((0, 1), (0, 0)), ((0, 0), (0, 1)));
    }

    #[test]
    fn test_wire_intersections() {
        assert_eq!(
            find_intersection(((0, 0), (0, 1)), ((1, 0), (1, 1))),
            None
        );

        // Intersects at right angle
        assert_eq!(
            find_intersection(((0, 0), (0, 1)), ((1, 1), (0, 1))),
            Some((0, 1))
        );

        assert_eq!(
            find_intersection(((1, 1), (0, 1)), ((0, 0), (0, 1))),
            Some((0, 1))
        );

        // Intersects at right angle
        assert_eq!(
            find_intersection(((1, 0), (0, 0)), ((1, 1), (1, 0))),
            Some((1, 0))
        );

        assert_eq!(
            find_intersection(((1, 1), (1, 0)), ((1, 0), (0, 0))),
            Some((1, 0))
        );

        // Intersects like a cross
        assert_eq!(
            find_intersection(((0, 2), (3, 2)), ((1, 0), (1, 3))),
            Some((1, 2))
        );

        assert_eq!(
            find_intersection(((1, 0), (1, 3)), ((0, 2), (3, 2))),
            Some((1, 2))
        );

        // Segments are on the same line and intersect at one endpoint
        assert_eq!(
            find_intersection(((0, 0), (1, 0)), ((1, 0), (2, 0))),
            Some((1, 0))
        )

    }
}
