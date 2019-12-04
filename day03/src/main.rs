fn main() {
    let w1 = wire_1().split(",").collect();
    let w2 = wire_2().split(",").collect();

    match find_min_intersections(w1, w2) {
        Some(min_point) => println!("Min intersection point: {}", min_point),
        _ => println!("No min found"),
    }
}

fn wire_1() -> &'static str {
    "R1002,D715,R356,D749,L255,U433,L558,D840,R933,U14,L285,U220,L88,D477,R36,U798,R373,U378,R305,D341,R959,D604,R717,D911,L224,D32,R481,D508,L203,U445,L856,U44,L518,U909,R580,U565,R484,D170,R356,U614,R278,U120,R540,D330,R124,D555,R890,U445,L876,D948,R956,D503,R391,U564,R624,D642,L821,U924,L921,U869,R104,U376,L693,U812,R758,U200,L515,U435,R505,U22,R707,U926,R261,D332,R535,D704,L561,U476,R225,U168,L784,D794,R311,D426,R813,U584,L831,D258,R241,D665,R550,D709,R261,U557,L670,D823,L297,U951,R634,D647,R699,U907,L219,U481,L583,D854,L898,U535,R648,U307,L870,D748,R768,D502,L15,U684,R476,D591,L531,D881,L466,U135,R445,U813,R950,D303,L590,U938,R630,D233,R567,U739,L446,U689,R585,D892,R741,U849,R629,D972,L625,D524,L715,D936,L328,U102,R864,U859,L827,U162,L886,D785,R359,D38,R51,U999,R560,U415,L840,U736,R552,D277,R722,D444,R164,U335,L129,D873,L499,U847,R84,U780,R104,U879,R938,D468,L575,D668,L143,U917,R86,D562,R595,U924,R807,U76,L44,D685,R936,U876,R570,U782,L139,D815,R89,D976,R84,U446,R238,U853,L603,U869,R312,U970,R387,U131,L647,D383,R161,D818,L765,U291,L423,D753,R277,U840,R23,U265,R298,U665,R522,D955,R26,D320,R347,U952,R743,U782,L780,D20,L393,U855,L279,D969,L923,D902,L818,U855,L927,D342,R769,U517,L485,U176,R14,U683,L632,U198,R656,U444,R41,D911,R99,U880,L363,D15,L894,D782,R612,D677,R469,D166,R61,U284,R474,U222,L687,D502,R690,U619,R536,D663,L54,D660,L804,D697,R67,U116,R842,D785,R277,U978,L920,D926,R681,D957,L582,U441,L593,U686,R829,U937,L924,U965,R727,D964,R468,U240,R934,D266,R416"
}

fn wire_2() -> &'static str {
    "L998,U258,R975,U197,R680,D56,R898,D710,R475,U909,L201,D579,L21,U743,R832,D448,R216,D136,R83,U413,R167,U138,R102,U122,L290,D49,L93,D941,L625,U709,R129,D340,L322,D27,R440,U692,R368,D687,L246,D425,R823,U287,L436,U999,R90,U663,R470,U177,R956,D981,L767,D780,R610,D644,R238,D416,R402,D327,L680,D367,L94,D776,L331,D745,R846,D559,R113,U158,R125,D627,L898,D212,L80,D184,L386,U943,R122,D614,L868,D600,R912,U501,R25,D887,R310,U872,L157,U865,L382,U959,R712,D248,L343,U819,L763,U886,R582,D631,L835,U443,L917,D934,L333,U470,R778,U142,R384,U589,R306,U933,L206,D199,L497,D406,L212,U439,L15,U985,R505,D502,R934,D966,R429,U810,R588,U367,L424,U804,R767,U703,R885,U568,R748,U209,L319,U305,L941,D184,R398,U681,L411,U414,L90,U711,L575,D368,L986,U29,R982,U361,L501,D970,R558,D887,L241,U506,R578,D932,R911,U621,L153,U200,L873,U711,L843,U549,R72,U377,R915,D79,L378,U66,L989,D589,L341,D350,L200,D78,R944,U876,L794,U643,R871,D909,L353,D54,R651,U338,R857,D938,R636,D301,R728,U318,R530,D589,L682,U784,L428,D879,L207,D247,L53,U312,L488,D534,L998,U512,L628,D957,L994,D747,L804,U399,L801,D500,R791,D980,R839,U564,L81,U461,R615,U863,R308,D564,R843,U579,R792,D472,R229,D153,L21,D647,R425,D54,L470,U330,R285,D81,L221,U168,R970,D624,R815,U189,L812,U195,L654,U108,R820,U786,L932,U657,L605,D164,L788,D393,L717,D49,R615,D81,L91,U322,L150,D368,R434,D861,L859,D911,R161,U576,L671,U992,L745,U585,R440,D731,R740,U584,L867,D906,R176,U72,L323,U329,L445,D667,R626,D111,L895,D170,R957,D488,R214,D354,L215,U486,L665,D266,L987"
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

    let res = if (s2_x1 <= s1_x1 && s1_x1 <= s2_x2) && (s1_y1 <= s2_y1 && s2_y1 <= s1_y2) {
        Some((s1_x1, s2_y1))
    } else if (s1_x1 <= s2_x1 && s2_x1 <= s1_x2) && (s2_y1 <= s1_y1 && s1_y1 <= s2_y2) {
        Some((s2_x1, s1_y1))
    } else {
        None
    };

    // Don't count intersections at the origin
    if res == Some((0, 0)) {
        None
    } else {
        res
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

fn find_min_intersections(w1: Vec<&str>, w2: Vec<&str>) -> Option<i32> {
    let w1_segs = gen_wire_segments(gen_wire_coordinates(w1));
    let w2_segs = gen_wire_segments(gen_wire_coordinates(w2));

    let mut results = Vec::new();

    for s1 in w1_segs {
        for s2 in w2_segs.iter() {
            match find_intersection(s1, *s2) {
                Some((x, y)) => {
                    results.push(x.abs() + y.abs());
                }
                None => (),
            }
        }
    }

    results.into_iter().min()
}

fn find_lowest_power_intersection(w1: Vec<&str>, w2: Vec<&str>) -> Option<i32> {
    let w1_segs = gen_wire_segments(gen_wire_coordinates(w1));
    let w2_segs = gen_wire_segments(gen_wire_coordinates(w2));

    let mut results = Vec::new();

    for s1 in w1_segs.iter() {
        for s2 in w2_segs.iter() {
            match find_intersection(*s1, *s2) {
                Some((x, y)) => {
                    println!("({}, {})", x, y);
                    println!("Dist to first {}", dist_to_point(&w1_segs, &(x,y)));
                    println!("Dist to second {}", dist_to_point(&w2_segs, &(x,y)));
                    println!("---------------");
                    results.push(dist_to_point(&w1_segs, &(x,y)) + dist_to_point(&w2_segs, &(x,y)))
                }
                None => (),
            }
        }
    }

    results.into_iter().min()

}

fn dist_to_point(w: &Vec<((i32, i32), (i32, i32))>, p: &(i32, i32)) -> i32 {
    let mut dist = 0;

  for ((x1, y1), (x2, y2)) in w {
   // for (p1, p2) in w {
   //     let ((x1, y1), (x2, y2)) = order_points(*p1, *p2);

      // if x1 <= p.0 && p.0 <= x2 && y1 <= p.1 && p.1 <= y2 {
      if x1 <= &p.0 && &p.0 <= x2 && y1 <= &p.1 && &p.1 <= y2 {
          // in segment

          // problem here is that I'm approaching from the right. This
          // is causing all totals to get messed up. Need to handle
          // two scenarios depending on if point is ordered or not.
          // there might be a bug with find_intersections swapping points
          println!("in seg, {}, p0={}, x1={}, p1={}, y1={}", (p.0 - x1).abs() + (p.1 - y1).abs(), p.0, x1, p.1, y1);
            //dist += (p.0).abs() - x1.abs() + (p.1).abs() - y1.abs();

          dist += (p.0 - x1).abs() + (p.1 - y1).abs();
//            dist += (p.0 - x2).abs() + (p.1 - y1).abs();
            break;
        } else {
            println!("not in seg, {}", (x2 - x1).abs() + (y2 - y1).abs());
            dist += (x2 - x1).abs() + (y2 - y1).abs();
        }
    }

    dist
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

        assert_eq!(
            gen_wire_coordinates("R8,U5,L5,D3".split(",").collect()),
            vec![(0, 0), (8, 0), (8, 5), (3, 5), (3, 2)]
        );

        assert_eq!(
            gen_wire_coordinates("U7,R6,D4,L4".split(",").collect()),
            vec![(0, 0), (0, 7), (6, 7), (6, 3), (2, 3)]
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
        assert_eq!(find_intersection(((0, 0), (0, 1)), ((1, 0), (1, 1))), None);

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
        );

        // From example
        assert_eq!(
            find_intersection(((2, 3), (6, 3)), ((3, 2), (3, 5))),
            Some((3, 3))
        );

        assert_eq!(
            find_intersection(((3, 2), (3, 5)), ((2, 3), (6, 3))),
            Some((3, 3))
        );

        // From example
        assert_eq!(
            find_intersection(((3, 5), (8, 5)), ((6, 3), (6, 7))),
            Some((6, 5))
        );

        assert_eq!(
            find_intersection(((6, 3), (6, 7)), ((3, 5), (8, 5))),
            Some((6, 5))
        );

        // From errors in my degugging
        assert_eq!(find_intersection(((0, 0), (8, 0)), ((6, 3), (6, 7))), None);
    }

    #[test]
    fn test_find_min_intersections_example1() {
        let w1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",").collect();
        let w2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(",").collect();
        assert_eq!(find_min_intersections(w1, w2), Some(159));
    }

    #[test]
    fn test_find_min_intersections_example2() {
        let w1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .split(",")
            .collect();
        let w2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",").collect();
        assert_eq!(find_min_intersections(w1, w2), Some(135));
    }

    #[test]
    fn test_find_min_intersections_text_example() {
        let w1 = "R8,U5,L5,D3".split(",").collect();
        let w2 = "U7,R6,D4,L4".split(",").collect();
        assert_eq!(find_min_intersections(w1, w2), Some(6));
    }

    #[test]
    fn test_dist_to_point() {
        assert_eq!(dist_to_point(&vec![((0,0), (0, 2)), ((0, 2), (2, 2))], &(1, 2)), 3);

        assert_eq!(dist_to_point(&vec![((0,0), (8, 0)), ((8, 0), (8, 5)), ((8, 5), (3, 5))], &(6, 5)), 15);
    }

    #[test]
    fn test_find_lowest_power_intersection_text_example1() {
        let w1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",").collect();
        let w2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(",").collect();
        assert_eq!(find_lowest_power_intersection(w1, w2), Some(610));
    }

    #[test]
    fn test_find_lowest_power_intersection_text_example() {
        let w1 = "R8,U5,L5,D3".split(",").collect();
        let w2 = "U7,R6,D4,L4".split(",").collect();
        assert_eq!(find_lowest_power_intersection(w1, w2), Some(30));
    }


}
