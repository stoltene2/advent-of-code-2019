use std::collections::HashSet;

type Point = (i32, i32);

fn main() {
    // increase every point if it isn't 9.
    //if point is 9 then add to `to_process` list

    // `to_process` until empty
    // Pop point, `p`
    // if p == 9
    //    Set p at (x,y) to 0 add all surrounding to `to_process`
    //    record +1
    // else if 0
    //    do nothing
    // else
    //    incease p+1

    let _test_input = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 9, 9, 9, 1],
        vec![1, 9, 1, 9, 1],
        vec![1, 9, 9, 9, 1],
        vec![1, 1, 1, 1, 1],
    ];

    let mut total: i32 = 0;

    let mut result = input().clone();
    for _i in 1..100 {
        let r = cycle(result);

        total += r.0;

        result = r.1;
    }

    assert_eq!(1697, total);

    println!("ocean: {:?}\ntotal: {}", &result, &total);

    // Iterate until all octopuses are 0
    let mut result = input().clone();
    for i in 1.. {
        let r = cycle(result);
        result = r.1;

        let all_zeros = result.iter().all(|rows| rows.iter().all(|x| *x == 0));

        if all_zeros {
            println!("Solution 2: {}", i);
            assert_eq!(344, i);
            break;
        }
    }
}

fn cycle(data: Vec<Vec<u8>>) -> (i32, Vec<Vec<u8>>) {
    let r = increase(data);
    let mut ocean = r.0;
    let mut next = r.1;

    let mut total: i32 = 0;

    while !next.is_empty() {
        let (x, y) = next.pop().unwrap();
        let val = ocean[y as usize][x as usize];

        if val == 9 {
            ocean[y as usize][x as usize] = 0;
            total += 1;

            let mut neighbors: Vec<_> = neighbors(&(x, y)).iter().cloned().collect();
            next.append(&mut neighbors);
        } else if val == 0 {
            continue;
        } else {
            ocean[y as usize][x as usize] += 1;
        }
    }
    (total, ocean)
}

fn increase(mut octopi: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, Vec<Point>) {
    let mut next: Vec<Point> = Vec::new();

    for (y, row) in octopi.iter_mut().enumerate() {
        for (x, val) in row.iter_mut().enumerate() {
            match *val {
                9 => {
                    next.push((x as i32, y as i32));
                    ()
                }
                _ => *val = *val + 1,
            }
        }
    }

    (octopi, next)
}

fn neighbors((x, y): &Point) -> HashSet<Point> {
    // Iterate over potential points and push to Set
    let mut ps = vec![
        (*x - 1, y - 1),
        (*x, y - 1),
        (x + 1, *y - 1),
        (x - 1, *y),
        (x + 1, *y),
        (x - 1, y + 1),
        (*x, y + 1),
        (x + 1, y + 1),
    ];

    ps = ps
        .iter()
        .filter(|(x, y)| x >= &0 && x < &10 && y >= &0 && y < &10)
        .cloned()
        .collect();

    ps.iter().fold(HashSet::new(), |mut result, p| {
        result.insert(*p);
        result
    })
}

fn input() -> Vec<Vec<u8>> {
    vec![
        vec![4, 3, 4, 1, 3, 4, 7, 6, 4, 3],
        vec![5, 4, 7, 7, 7, 2, 8, 4, 5, 1],
        vec![2, 3, 2, 2, 7, 3, 3, 8, 7, 8],
        vec![5, 4, 5, 3, 7, 6, 2, 5, 5, 6],
        vec![2, 7, 1, 8, 1, 2, 3, 4, 2, 1],
        vec![4, 2, 3, 7, 8, 8, 6, 1, 1, 5],
        vec![5, 6, 3, 1, 6, 1, 7, 1, 1, 4],
        vec![2, 2, 1, 7, 6, 6, 7, 2, 2, 7],
        vec![4, 2, 3, 6, 5, 8, 1, 2, 5, 5],
        vec![4, 4, 8, 2, 6, 2, 7, 6, 4, 1],
    ]
}
