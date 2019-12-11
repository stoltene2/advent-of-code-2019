// Note 2,2 -> 3,4 is not found for some reason


extern crate num;
use std::convert::TryInto;
use std::collections::HashMap;
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
            if p.x < self.p1.x || self.p2.x < p.x || p.y < self.p1.y || self.p2.y < p.y {
                // Bail out if we are not in the segment
                return false;
            }

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

    // Starts with the first point in the vector
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

            if val == b'#' {
                asteroids.push(Point::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }

    // println!("Asteroid Field\n{:#?}", &asteroids);
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

fn find_best_asteroid(input: Vec<String>) -> (Point, u32) {
    let row_len = input[0].len();

    let asteroids = input_to_points(&input);
    let mut h: HashMap<Point, u32> = HashMap::new();

//    for (i, (p1, p2)) in all_pairs(&asteroids).into_iter().enumerate() {

    for (i, p) in asteroids.iter().enumerate() {
        for j in i+1..asteroids.len() {
            // if *p1 == Point::new(31, 0) {
            //     panic!("I should not be here");
            // }

            let p1 = p;
            let p2 = asteroids[j];

            let s = Segment::new(*p1, p2);

            // This includes too many points to check. It should only
            // consider points ahead of p1, not _all_ asteroids That's
            // what this line below does I think i need to put this into
            // one long vector to check and do the double loop.  This
            // approach I used below mixes two concepts. Using an index
            // from all pairs into the asteroids vec. Bad
            // let asteroids_to_check: Vec<_> = asteroids.iter().skip(i).map(|p| *p).collect();
            // let points_to_check = s.points_on_segment(&asteroids_to_check);
            let points_to_check = s.points_on_segment(&asteroids);

            let closest_point = s.line_of_sight(&points_to_check);
            if closest_point == p2 {
                if p1.x == 31 && p1.y == 0 {
                    panic!("How did I get here? {:?} - {:?}", p1, p2);
                }

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

        let p = Point::new(6, 3);
        assert_eq!((p, 41), find_best_asteroid(input));
    }



}
