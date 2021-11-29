// Note 2,2 -> 3,4 is not found for some reason


extern crate num;
use std::convert::TryInto;
use std::collections::HashMap;
use std::cmp::Ordering;
use num::FromPrimitive;
use num::rational::*;


#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
        Point { x, y,}
    }

    fn euclidean_metric(&self, p: &Point) -> i16 {
        (self.x - p.x).pow(2) + (self.y - p.y).pow(2)
    }

    fn angle_metric(&self, p: &Point) -> f32 {
        use std::f32::consts::PI;

        // let rel_x: f32 = (p.x - self.x).into();
        // let rel_y: f32 = (p.y - self.y).into();

        let rel_y: f32 = (p.x - self.x).into();
        let rel_x: f32 = (p.y - self.y).into();

        let pi_2: f32 = PI/2.0;

        let quadrant_offset: f32 = if rel_x < 0.0 && rel_y < 0.0 {
            //Quad 2
            pi_2
        } else if rel_x <= 0.0 {
            //Quad 0
            //3.0*pi_2
            0_f32
        } else if rel_y <= 0.0 {
            //Quad ?
            3.0*pi_2
        } else {
            2.0*pi_2
            //Quad ?
        };

        let ratio: f32 = (rel_y / rel_x).abs();

        quadrant_offset + ratio.atan()

    }

    fn cmp(&self, other: &Point) -> Ordering {
        // I think I can use some of the convenience methods here
        // https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then_with
        // I don't know how to implement this Trait correctly

        if self.x == other.x && self.y == other.y {
            return Ordering::Equal;
        }

        if self.y <= other.y {
            if self.x < other.x {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    }

}

#[derive(Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}


impl Segment {

    fn new(p1: Point, p2: Point) -> Segment {
        Segment{p1, p2}
    }

    fn contains(&self, p: &Point) -> bool {
        if self.p1 == *p || self.p2 == *p {
            return true;
        }

        // Verify that p is in the bounding box of self
        let min_x = self.p1.x.min(self.p2.x);
        let max_x = self.p1.x.max(self.p2.x);
        let min_y = self.p1.y.min(self.p2.y);
        let max_y = self.p1.y.max(self.p2.y);
        if p.x < min_x || max_x < p.x || p.y < min_y || max_y < p.y {
            // Bail out if we are not in the segment
            return false;
        }

        if self.p1.x == self.p2.x {
            // Vertical segment
            if self.p1.y < p.y && p.y < self.p2.y && self.p1.x == p.x {
                return true;
            } else {
                return false;
            }
        } else if self.p1.y == self.p2.y {
            // Horizontal segment
            if self.p1.x < p.x && p.x < self.p2.x && self.p1.y == p.y {
                return true;
            } else {
                return false;
            }
        } else {
            // diagonal - non-zero or 1 slope
            // y = mx + b

            // I'm choosing to use Ratios for the accuracy here.
            // Maybe I should use ratios for all the numbers so I
            // never need to cast and unwrap.
            let p1_x: Ratio<i16> = FromPrimitive::from_i16(self.p1.x).unwrap();
            let p1_y: Ratio<i16> = FromPrimitive::from_i16(self.p1.y).unwrap();
            let p_x: Ratio<i16> = FromPrimitive::from_i16(p.x).unwrap();

            let delta_y: Ratio<i16> = FromPrimitive::from_i16(self.p2.y - self.p1.y).unwrap();
            let delta_x: Ratio<i16> = FromPrimitive::from_i16(self.p2.x - self.p1.x).unwrap();

            let slope: Ratio<i16> = delta_y / delta_x;
            let b: Ratio<i16> = (p1_y - (slope * p1_x));
            let y: Ratio<i16> = slope*p_x + b;

            if y.is_integer() {
                return y.to_integer() == p.y;
            }
        }

        false
    }

    // Starts with the first point in the Segment
    fn line_of_sight(&self, points: &Vec<&Point>) -> Point {
        let mut ps = points.clone();
        if ps.is_empty() {
            return self.p2;
        }

        ps.sort_by(|p1, p2| {
            let d1 = self.p1.euclidean_metric(p1);
            let d2 = self.p1.euclidean_metric(p2);
            d1.cmp(&d2)
        });

        let r = ps.iter()
            .filter(|&&&p| p != self.p1).collect::<Vec<&&Point>>()[0];

        **r
    }

    fn points_on_segment<'a>(&self, points: &'a Vec<Point>) -> Vec<&'a Point> {
        points.iter().filter(|&p| {
            self.contains(p) && self.p1 != *p
        }).collect()
    }
}

fn input_to_points(input: &Vec<String>) -> Vec<Point> {
    let row_len = input[0].as_bytes().len();
    let mut asteroids: Vec<Point> = Vec::with_capacity(row_len*input.len());

    for y in 0..input.len() {
        for x in 0..row_len {
            let val = input[y].as_bytes()[x];

            // Take any non dot as an asteroid. It helps make good
            // examples for tests using A and a.
            if val != b'.' {
                asteroids.push(Point::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }

    asteroids.sort_by(|p1, p2| p1.cmp(&p2));
    asteroids
}

fn all_pairs<'a, T>(v: &'a Vec<T>) -> Vec<(&'a T, &'a T)> {
    let mut result = Vec::new();

    for (i, p) in v.iter().enumerate() {
        for j in i+1..v.len() {
            result.push((&v[i], &v[j]))
        }
    }

    result
}
fn find_asteroid_counts(asteroids: &Vec<Point>) -> HashMap<Point, u32> {

    let mut h: HashMap<Point, u32> = HashMap::new();

    for (i, p) in asteroids.iter().enumerate() {
        for j in i+1..asteroids.len() {
            // if *p1 == Point::new(31, 0) {
            //     panic!("I should not be here");
            // }

            let p1 = p;
            let p2 = asteroids[j];

            let s = Segment::new(*p1, p2);

            // let asteroids_to_check: Vec<_> = asteroids.iter().skip(i).map(|p| *p).collect();
            // let points_to_check = s.points_on_segment(&asteroids_to_check);
            let points_to_check = s.points_on_segment(&asteroids);

            let closest_point = s.line_of_sight(&points_to_check);

            // If closest point and end point are equal
            if closest_point.cmp(&p2) == Ordering::Equal {
                if let Some(count) = h.get(&p1) {
                    h.insert(*p1, count+1);
                } else {
                    h.insert(*p1, 1);
                }

                if let Some(count) = h.get(&p2) {
                    h.insert(p2, count+1);
                } else {
                    h.insert(p2, 1);
                }
            }
        }
    }

    h
}

fn find_best_asteroid(input: Vec<String>) -> (Point, u32) {
    let row_len = input[0].len();

    let asteroids = input_to_points(&input);

    let is_sorted = asteroids.iter()
        .zip(asteroids.iter().skip(1))
        .all(|(p1, p2)| {
            p1.y <= p2.y && p1.x <= p2.y
        });

    // assert!(is_sorted);

    let h = find_asteroid_counts(&asteroids);
    let (p, max) = h.iter().max_by(|(_, v1), (_, v2)| v1.cmp(&v2)).unwrap();

    (*p, *max)
}

fn main() {
    let (p, max) = find_best_asteroid(program_input());
    println!("{:?}, {}", p, max);
}

fn program_input() -> Vec<String> {
    vec![
        String::from(".#..#..##.#...###.#............#."),
        String::from(".....#..........##..#..#####.#..#"),
        String::from("#....#...#..#.......#...........#"),
        String::from(".#....#....#....#.#...#.#.#.#...."),
        String::from("..#..#.....#.......###.#.#.##...."),
        String::from("...#.##.###..#....#........#..#.#"),
        String::from("..#.##..#.#.#...##..........#...#"),
        String::from("..#..#.......................#..#"),
        String::from("...#..#.#...##.#...#.#..#.#......"),
        String::from("......#......#.....#............."),
        String::from(".###..#.#..#...#..#.#.......##..#"),
        String::from(".#...#.................###......#"),
        String::from("#.#.......#..####.#..##.###.....#"),
        String::from(".#.#..#.#...##.#.#..#..##.#.#.#.."),
        String::from("##...#....#...#....##....#.#....#"),
        String::from("......#..#......#.#.....##..#.#.."),
        String::from("##.###.....#.#.###.#..#..#..###.."),
        String::from("#...........#.#..#..#..#....#...."),
        String::from("..........#.#.#..#.###...#.....#."),
        String::from("...#.###........##..#..##........"),
        String::from(".###.....#.#.###...##.........#.."),
        String::from("#.#...##.....#.#.........#..#.###"),
        String::from("..##..##........#........#......#"),
        String::from("..####......#...#..........#.#..."),
        String::from("......##...##.#........#...##.##."),
        String::from(".#..###...#.......#........#....#"),
        String::from("...##...#..#...#..#..#.#.#...#..."),
        String::from("....#......#.#............##....."),
        String::from("#......####...#.....#...#......#."),
        String::from("...#............#...#..#.#.#..#.#"),
        String::from(".#...#....###.####....#.#........"),
        String::from("#.#...##...#.##...#....#.#..##.#."),
        String::from(".#....#.###..#..##.#.##...#.#..##"),
    ]

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vertical_segment_contains_first_endpoint() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(0,3);
        let interior_point = Point::new(0, 0);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }

    #[test]
    fn test_vertical_segment_contains_last_endpoint() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(0,3);
        let interior_point = Point::new(0, 3);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }


    #[test]
    fn test_vertical_segment_contains_point() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(0,3);
        let interior_point = Point::new(0, 1);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }

    #[test]
    fn test_vertical_segment_does_not_include_exterior_point() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(0,3);
        let exterior = Point::new(0, 10);
        let seg = Segment::new(p1, p2);

        assert!(!seg.contains(&exterior));
    }

    #[test]
    fn test_horizontal_segment_contains_point() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(3,0);
        let interior_point = Point::new(1, 0);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }

    #[test]
    fn test_horizontal_segment_does_not_contain_point() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(3,0);
        let exterior = Point::new(4, 0);
        let seg = Segment::new(p1, p2);

        assert!(!seg.contains(&exterior));
    }

    #[test]
    fn test_horizontal_segment_contains_first_endpoint() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(3,0);
        let interior_point = Point::new(0, 0);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }

    #[test]
    fn test_horizontal_segment_contains_last_endpoint() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(3,0);
        let interior_point = Point::new(3, 0);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }

    // Find points on any line
    #[test]
    fn test_any_segment_contains_first_endpoint() {
        let p1 = Point::new(0,0);
        let p2 = Point::new(7,1);
        let interior_point = Point::new(0, 0);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }


    #[test]
    fn test_any_segment_contains_last_endpoint() {
        // #.........
        // ..........
        // ......A...
        let p1 = Point::new(0,0);
        let p2 = Point::new(7,1);
        let interior_point = Point::new(7, 1);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&interior_point));
    }

    #[test]
    fn test_any_segment_contains_last_endpoint_right_to_left_diagonal() {
        // ...#
        // ....
        // #...
        let p1 = Point::new(3,0);
        let p2 = Point::new(0,2);
        let seg = Segment::new(p1, p2);

        assert!(seg.contains(&p2));
    }

    #[test]
    fn test_any_segment_does_not_contain_exterior_point() {
        // #...#.....
        // ..........
        // ......A...

        let p1 = Point::new(0,0);
        let p2 = Point::new(7,1);
        let interior_point = Point::new(4, 0);
        let seg = Segment::new(p1, p2);

        assert!(!seg.contains(&interior_point));
    }

    #[test]
    fn test_any_segment_contains_point_on_segment() {
        // #.........
        // ...A......
        // ......a...

        let p1 = Point::new(0,0);
        let interior_point = Point::new(3, 1);
        let p2 = Point::new(6,2);

        let seg = Segment::new(p1, p2);
        assert!(seg.contains(&interior_point));
    }

    #[test]
    fn test_any_segment_contains_point_on_diagonal() {
        // ..........
        // ..........
        // ..#.......
        // ..........
        // ...#......

        let p1 = Point::new(2,2);
        let p2 = Point::new(3,4);

        let seg = Segment::new(p1, p2);
        assert!(seg.contains(&p2));
    }

    #[test]
    fn test_horizontal_segment_with_some_point() {
        // #...#.....
        // ..........
        // ..........

        let p1 = Point::new(0,0);
        let interior_point = Point::new(2, 2);
        let p2 = Point::new(4,0);

        let seg = Segment::new(p1, p2);
        assert!(!seg.contains(&interior_point));
    }

    #[test]
    fn test_vertical_segment_with_some_point() {
        // #.........
        // ..........
        // #.........

        let p1 = Point::new(0,0);
        let interior_point = Point::new(2, 2);
        let p2 = Point::new(0,2);

        let seg = Segment::new(p1, p2);
        assert!(!seg.contains(&interior_point));
    }

    #[test]
    fn test_find_line_of_sight_trivial() {
        // #.........
        // ..........
        // ......A...
        let p1 = Point::new(0,0);
        let p2 = Point::new(7,1);
        let seg = Segment::new(p1, p2);

        assert_eq!(p2, seg.line_of_sight(&vec![]));
    }

    #[test]
    fn test_find_line_of_sight_finds_closest_point_on_line() {
        // #.........
        // ...b......
        // ......a...

        let p1 = Point::new(0,0);
        let b = Point::new(3, 1);
        let a = Point::new(6,2);

        let seg = Segment::new(p1, a);
        assert_eq!(b, seg.line_of_sight(&vec![&a, &b]));
    }

    #[test]
    fn test_find_points_on_segment() {
        let p1 = Point::new(2,2);
        let p2 = Point::new(3,4);
        let bad = Point::new(1, 0);

        let s = Segment::new(p1, p2);

        assert!(!s.contains(&bad));
        assert_eq!(vec![&p2], s.points_on_segment(&vec![bad, p1, p2]));
    }

    #[test]
    fn test_find_points_on_segment_negative_slope() {
        // ...2
        // ..3.
        // .2..
        let p1 = Point::new(3,0);
        let good = Point::new(2, 1);
        let p2 = Point::new(1,2);

        let s = Segment::new(p1, p2);

        assert!(s.contains(&good));
        assert_eq!(vec![&good, &p2], s.points_on_segment(&vec![good, p1, p2]));
    }


    #[test]
    fn test_paths_from_example() {
        let asteroids = input_to_points(&vec![
            String::from("#........."),
            String::from("...A......"),
            String::from("...B..a..."),
            String::from(".EDCG....a"),
            String::from("..F.c.b..."),
            String::from(".....c...."),
            String::from("..efd.c.gb"),
            String::from(".......c.."),
            String::from("....f...c."),
            String::from("...e..d..c"),
        ]);

        // A (0,0) -> (9,3) = (3,1)
        let p1 = Point::new(0,0);
        let p2 = Point::new(9,3);
        let s = Segment::new(p1,p2);
        let ps = s.points_on_segment(&asteroids);
        assert_eq!(Point::new(3,1), s.line_of_sight(&ps));


        // C
        // (0,0) -> (9,9) = (3,3)
        let p1 = Point::new(0,0);
        let p2 = Point::new(9,9);
        let s = Segment::new(p1,p2);
        let ps = s.points_on_segment(&asteroids);
        assert_eq!(Point::new(3,3), s.line_of_sight(&ps));

        // # should see 7
        assert_eq!(Some(&7), find_asteroid_counts(&asteroids).get(&p1));
    }

    #[test]
    fn test_paths_from_simple_example() {
        let asteroids = input_to_points(&vec![
            String::from("...2"),
            String::from("..3."),
            String::from(".23."),
        ]);

        // A (0,0) -> (9,3) = (3,1)
        let p1 = Point::new(3,0);
        let p2 = Point::new(2,1);
        let p3 = Point::new(1,2);
        let p4 = Point::new(2,2);
        assert_eq!(Some(&2), find_asteroid_counts(&asteroids).get(&p1));
        assert_eq!(Some(&3), find_asteroid_counts(&asteroids).get(&p2));
        assert_eq!(Some(&2), find_asteroid_counts(&asteroids).get(&p3));
        assert_eq!(Some(&3), find_asteroid_counts(&asteroids).get(&p4));
    }

    // TODO: Most below here are failing

    #[test]
    fn test_example_1() {
        let input = vec![
            String::from("......#.#."),
            String::from("#..#.#...."),
            String::from("..#######."),
            String::from(".#.#.###.."),
            String::from(".#..#....."),
            String::from("..#....#.#"),
            String::from("#..#....#."),
            String::from(".##.#..###"),
            String::from("##...#..#."),
            String::from(".#....####"),
        ];

        let p = Point::new(5, 8);
        assert_eq!((p, 33), find_best_asteroid(input));
    }

    #[test]
    fn test_example_2() {
        let input = vec![
            String::from("#.#...#.#."),
            String::from(".###....#."),
            String::from(".#....#..."),
            String::from("##.#.#.#.#"),
            String::from("....#.#.#."),
            String::from(".##..###.#"),
            String::from("..#...##.."),
            String::from("..##....##"),
            String::from("......#..."),
            String::from(".####.###."),
        ];

        let p = Point::new(1, 2);
        assert_eq!((p, 35), find_best_asteroid(input));

    }

    #[test]
    fn test_example_3() {
        let input = vec![
            String::from(".#..#..###"),
            String::from("####.###.#"),
            String::from("....###.#."),
            String::from("..###.##.#"),
            String::from("##.##.#.#."),
            String::from("....###..#"),
            String::from("..#.#..#.#"),
            String::from("#..#.#.###"),
            String::from(".##...##.#"),
            String::from(".....#.#.."),
        ];

        let p = Point::new(6, 3);
        assert_eq!((p, 41), find_best_asteroid(input));
    }

    #[test]
    fn test_example_4() {
        let input = vec![
            String::from(".#..##.###...#######"),
            String::from("##.############..##."),
            String::from(".#.######.########.#"),
            String::from(".###.#######.####.#."),
            String::from("#####.##.#.##.###.##"),
            String::from("..#####..#.#########"),
            String::from("####################"),
            String::from("#.####....###.#.#.##"),
            String::from("##.#################"),
            String::from("#####.##.###..####.."),
            String::from("..######..##.#######"),
            String::from("####.##.####...##..#"),
            String::from(".#####..#.######.###"),
            String::from("##...#.##########..."),
            String::from("#.##########.#######"),
            String::from(".####.#.###.###.#.##"),
            String::from("....##.##.###..#####"),
            String::from(".#.#.###########.###"),
            String::from("#.#.#.#####.####.###"),
            String::from("###.##.####.##.#..##"),
        ];

        let p = Point::new(11, 13);
        assert_eq!((p, 210), find_best_asteroid(input));
    }

    #[test]
    fn test_angle_metric() {

        // .#....###24...#..
        // ##...##.13#67..9#
        // ##...#...5.8####.
        // ..#.....X...###..
        // ..#.#.....#....##
        let X = Point::new(8,3);
        let p_1 = Point::new(8, 1);
        let p_2 = Point::new(9, 0);
        let p_3 = Point::new(9, 1);
        let p_4 = Point::new(10, 0);
        let p_5 = Point::new(9, 2);
        let p_6 = Point::new(11, 1);
        let p_7 = Point::new(12, 1);

        // Next quadrant
        let p_8 = Point::new(12, 3);
        let p_9 = Point::new(10, 4);

        // Next quadrant
        let p_10 = Point::new(4, 4);

        // Last quadrant
        let p_11 = Point::new(7, 0);

        println!("{}", X.angle_metric(&p_1));
        println!("{}", X.angle_metric(&p_2));
        println!("{}", X.angle_metric(&p_3));
        println!("{}", X.angle_metric(&p_4));
        println!("{}", X.angle_metric(&p_5));
        println!("{}", X.angle_metric(&p_6));
        println!("{}", X.angle_metric(&p_7));
        println!("{}", X.angle_metric(&p_8));

        println!("{}", X.angle_metric(&p_9));
        println!("{}", X.angle_metric(&p_10));
        println!("{}", X.angle_metric(&p_11));
        assert!(false);
    }


}
