fn main() {
    let headings: Vec<(i64, i64)> = vec![(5, 0), (0, 5), (8, 0), (0, -3), (0, 8), (2, 0)];

    let position = headings
        .iter()
        .fold((0, 0), |(x_0, y_0), (x, y)| (x_0 + x, y_0 + y));

    let result = position.0 * position.1;
    assert_eq!(result, 150);
    println!("Example result: {}", result);

    let position = problem_1_input()
        .iter()
        .fold((0, 0), |(x_0, y_0), (x, y)| (x_0 + x, y_0 + y));

    let result = position.0 * position.1;
    println!("Part 1 result: {}", result);
    assert_eq!(result, 1962940);
}

/*
To generate this input I just used Emacs with the following passed as `query-replace-regexp`

forward \(.*\) -> (\1, 0)
down \(.*\) -> (0, \1)
up \(.*\) -> (0, -\1)

*/
fn problem_1_input() -> Vec<(i64, i64)> {
    vec![
        (4, 0),
        (0, 8),
        (0, 1),
        (6, 0),
        (7, 0),
        (0, 7),
        (3, 0),
        (5, 0),
        (0, -9),
        (0, 1),
        (5, 0),
        (0, 8),
        (4, 0),
        (5, 0),
        (0, 5),
        (0, 1),
        (1, 0),
        (0, 3),
        (5, 0),
        (5, 0),
        (0, 1),
        (0, -2),
        (0, 2),
        (0, 5),
        (0, 5),
        (3, 0),
        (7, 0),
        (5, 0),
        (9, 0),
        (8, 0),
        (0, 4),
        (0, 6),
        (0, -5),
        (0, 1),
        (6, 0),
        (0, -3),
        (7, 0),
        (4, 0),
        (0, 7),
        (0, -5),
        (0, -5),
        (0, -1),
        (0, -5),
        (5, 0),
        (2, 0),
        (7, 0),
        (0, 7),
        (9, 0),
        (0, 9),
        (0, -8),
        (0, -8),
        (0, -2),
        (5, 0),
        (8, 0),
        (0, -5),
        (1, 0),
        (0, 1),
        (0, 6),
        (1, 0),
        (2, 0),
        (4, 0),
        (6, 0),
        (0, -4),
        (0, -5),
        (0, 4),
        (0, 9),
        (0, 4),
        (4, 0),
        (0, -8),
        (0, -2),
        (0, 2),
        (0, -9),
        (9, 0),
        (4, 0),
        (1, 0),
        (6, 0),
        (0, -3),
        (6, 0),
        (2, 0),
        (0, -3),
        (0, 3),
        (6, 0),
        (0, 9),
        (0, 7),
        (3, 0),
        (0, -7),
        (0, -8),
        (3, 0),
        (0, 1),
        (0, 8),
        (7, 0),
        (3, 0),
        (0, 2),
        (0, 5),
        (5, 0),
        (1, 0),
        (0, 1),
        (0, 3),
        (0, 5),
        (1, 0),
        (0, 1),
        (0, 7),
        (1, 0),
        (0, -2),
        (0, 5),
        (0, -3),
        (0, -2),
        (0, 7),
        (0, -4),
        (2, 0),
        (0, 3),
        (0, 1),
        (0, -7),
        (0, 6),
        (0, 1),
        (7, 0),
        (0, 5),
        (0, 2),
        (7, 0),
        (0, -9),
        (6, 0),
        (6, 0),
        (2, 0),
        (6, 0),
        (0, 2),
        (4, 0),
        (0, 5),
        (4, 0),
        (0, 8),
        (3, 0),
        (0, 9),
        (0, -5),
        (6, 0),
        (0, 5),
        (5, 0),
        (0, 4),
        (0, 1),
        (3, 0),
        (0, -9),
        (0, -5),
        (0, -9),
        (0, 3),
        (7, 0),
        (7, 0),
        (0, -5),
        (0, -6),
        (0, -3),
        (0, 9),
        (0, 4),
        (0, -8),
        (0, 9),
        (0, 6),
        (5, 0),
        (0, 6),
        (7, 0),
        (0, 4),
        (0, 9),
        (0, 9),
        (6, 0),
        (0, 4),
        (0, -2),
        (0, 8),
        (0, -3),
        (0, -7),
        (0, -1),
        (9, 0),
        (0, 4),
        (0, 8),
        (0, -2),
        (7, 0),
        (5, 0),
        (0, 9),
        (0, 9),
        (0, -5),
        (0, 4),
        (8, 0),
        (0, -3),
        (0, -4),
        (0, -8),
        (0, 7),
        (6, 0),
        (0, 8),
        (0, 1),
        (0, -1),
        (0, 7),
        (0, 7),
        (3, 0),
        (0, 9),
        (0, -2),
        (2, 0),
        (0, -1),
        (0, -1),
        (0, 2),
        (0, 8),
        (0, -5),
        (0, 3),
        (0, 3),
        (2, 0),
        (0, 4),
        (2, 0),
        (0, 2),
        (3, 0),
        (0, 6),
        (8, 0),
        (0, 5),
        (0, 6),
        (9, 0),
        (2, 0),
        (0, 6),
        (0, 4),
        (0, -9),
        (2, 0),
        (1, 0),
        (0, -9),
        (0, 9),
        (8, 0),
        (0, 4),
        (0, -3),
        (0, 1),
        (9, 0),
        (9, 0),
        (3, 0),
        (4, 0),
        (0, 2),
        (0, 1),
        (5, 0),
        (0, -3),
        (6, 0),
        (0, 8),
        (0, 8),
        (0, 7),
        (1, 0),
        (6, 0),
        (0, 9),
        (0, 6),
        (8, 0),
        (0, 5),
        (0, -6),
        (0, 2),
        (2, 0),
        (0, -3),
        (6, 0),
        (4, 0),
        (0, -4),
        (0, 5),
        (2, 0),
        (0, 5),
        (1, 0),
        (5, 0),
        (0, -7),
        (0, -1),
        (0, 3),
        (0, -8),
        (4, 0),
        (8, 0),
        (8, 0),
        (0, -2),
        (0, 8),
        (0, -2),
        (0, -2),
        (0, -7),
        (0, 9),
        (0, 1),
        (1, 0),
        (0, 3),
        (0, 1),
        (0, 4),
        (3, 0),
        (0, 4),
        (0, 5),
        (7, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (0, -6),
        (0, 1),
        (0, 9),
        (0, -2),
        (0, -2),
        (1, 0),
        (0, -9),
        (6, 0),
        (0, 2),
        (6, 0),
        (8, 0),
        (0, -8),
        (0, 6),
        (2, 0),
        (0, -4),
        (0, -5),
        (0, 3),
        (0, 2),
        (7, 0),
        (0, 8),
        (4, 0),
        (8, 0),
        (0, -4),
        (0, 7),
        (6, 0),
        (1, 0),
        (0, -4),
        (0, 4),
        (0, 9),
        (0, 7),
        (0, 6),
        (0, 1),
        (7, 0),
        (0, -3),
        (0, 1),
        (0, 9),
        (0, 9),
        (0, 1),
        (0, 7),
        (0, 8),
        (0, -9),
        (0, 7),
        (0, -4),
        (4, 0),
        (0, 2),
        (0, -8),
        (0, 6),
        (0, 6),
        (4, 0),
        (0, -5),
        (0, 9),
        (0, 8),
        (0, -7),
        (0, 4),
        (9, 0),
        (0, -3),
        (0, 6),
        (7, 0),
        (0, -4),
        (9, 0),
        (0, 6),
        (6, 0),
        (0, 3),
        (0, 5),
        (0, 4),
        (0, -5),
        (0, 8),
        (0, 8),
        (5, 0),
        (1, 0),
        (0, 3),
        (7, 0),
        (0, 3),
        (0, -6),
        (5, 0),
        (0, -7),
        (8, 0),
        (0, 1),
        (7, 0),
        (8, 0),
        (9, 0),
        (7, 0),
        (0, -5),
        (9, 0),
        (0, -7),
        (0, 7),
        (8, 0),
        (0, 8),
        (0, -6),
        (0, 4),
        (6, 0),
        (3, 0),
        (3, 0),
        (6, 0),
        (0, 3),
        (0, -4),
        (0, 3),
        (0, 8),
        (2, 0),
        (0, 1),
        (0, 5),
        (2, 0),
        (0, -3),
        (0, -5),
        (2, 0),
        (8, 0),
        (0, 7),
        (0, 9),
        (8, 0),
        (5, 0),
        (2, 0),
        (0, 3),
        (6, 0),
        (3, 0),
        (4, 0),
        (9, 0),
        (0, 8),
        (2, 0),
        (0, 6),
        (0, 8),
        (1, 0),
        (5, 0),
        (0, -3),
        (8, 0),
        (0, -3),
        (2, 0),
        (0, 3),
        (0, 5),
        (0, -4),
        (0, 9),
        (0, -5),
        (0, 2),
        (7, 0),
        (8, 0),
        (2, 0),
        (4, 0),
        (6, 0),
        (0, 1),
        (0, -3),
        (3, 0),
        (0, -6),
        (1, 0),
        (0, 9),
        (4, 0),
        (5, 0),
        (3, 0),
        (0, 7),
        (0, 9),
        (1, 0),
        (5, 0),
        (0, -1),
        (0, 6),
        (0, 7),
        (0, -4),
        (0, -7),
        (2, 0),
        (0, 7),
        (5, 0),
        (0, -9),
        (0, -8),
        (8, 0),
        (0, -1),
        (0, -6),
        (0, 7),
        (0, -8),
        (2, 0),
        (0, 1),
        (7, 0),
        (6, 0),
        (2, 0),
        (0, -7),
        (0, 5),
        (0, 6),
        (8, 0),
        (0, 3),
        (0, 2),
        (5, 0),
        (0, 7),
        (2, 0),
        (0, 9),
        (7, 0),
        (9, 0),
        (1, 0),
        (0, 7),
        (0, 3),
        (0, 8),
        (0, 4),
        (0, -1),
        (0, 2),
        (5, 0),
        (9, 0),
        (5, 0),
        (0, -6),
        (0, -1),
        (3, 0),
        (1, 0),
        (7, 0),
        (0, 9),
        (4, 0),
        (0, 7),
        (0, -6),
        (1, 0),
        (0, 7),
        (5, 0),
        (0, 4),
        (0, 2),
        (0, -1),
        (6, 0),
        (0, -6),
        (0, 3),
        (0, -5),
        (0, 8),
        (0, 5),
        (2, 0),
        (0, 1),
        (8, 0),
        (4, 0),
        (0, 3),
        (3, 0),
        (6, 0),
        (2, 0),
        (9, 0),
        (2, 0),
        (0, 3),
        (8, 0),
        (0, 4),
        (0, 1),
        (4, 0),
        (0, 1),
        (5, 0),
        (0, 5),
        (0, 6),
        (6, 0),
        (0, 6),
        (0, 9),
        (7, 0),
        (0, 6),
        (6, 0),
        (7, 0),
        (1, 0),
        (4, 0),
        (2, 0),
        (3, 0),
        (0, -8),
        (0, 3),
        (0, 7),
        (6, 0),
        (4, 0),
        (0, -7),
        (6, 0),
        (6, 0),
        (0, 7),
        (0, -8),
        (0, 5),
        (6, 0),
        (8, 0),
        (0, 3),
        (0, -2),
        (0, 5),
        (2, 0),
        (5, 0),
        (0, -8),
        (1, 0),
        (0, 3),
        (3, 0),
        (2, 0),
        (0, 3),
        (0, 8),
        (3, 0),
        (1, 0),
        (0, 5),
        (0, 1),
        (0, -1),
        (9, 0),
        (0, 7),
        (0, -2),
        (8, 0),
        (0, 6),
        (0, 5),
        (0, -9),
        (2, 0),
        (5, 0),
        (8, 0),
        (0, -2),
        (0, -5),
        (2, 0),
        (0, 2),
        (0, 9),
        (0, 3),
        (7, 0),
        (0, -5),
        (7, 0),
        (0, 6),
        (2, 0),
        (7, 0),
        (8, 0),
        (8, 0),
        (0, 7),
        (3, 0),
        (6, 0),
        (0, 5),
        (8, 0),
        (6, 0),
        (0, -2),
        (1, 0),
        (0, -9),
        (1, 0),
        (0, -3),
        (6, 0),
        (0, 4),
        (0, 5),
        (0, 8),
        (0, -6),
        (1, 0),
        (0, 8),
        (3, 0),
        (2, 0),
        (9, 0),
        (0, 5),
        (0, 9),
        (5, 0),
        (0, 7),
        (0, -9),
        (5, 0),
        (7, 0),
        (6, 0),
        (5, 0),
        (0, 3),
        (6, 0),
        (0, 9),
        (0, -8),
        (4, 0),
        (7, 0),
        (3, 0),
        (0, 7),
        (8, 0),
        (0, 5),
        (3, 0),
        (0, -6),
        (0, -5),
        (9, 0),
        (0, -4),
        (0, -9),
        (9, 0),
        (3, 0),
        (0, 8),
        (8, 0),
        (0, 3),
        (2, 0),
        (0, 4),
        (0, 1),
        (2, 0),
        (0, -9),
        (0, 7),
        (4, 0),
        (0, -3),
        (0, 9),
        (0, 6),
        (2, 0),
        (5, 0),
        (0, 7),
        (0, 2),
        (8, 0),
        (0, 5),
        (8, 0),
        (0, 8),
        (0, 4),
        (0, 1),
        (0, 2),
        (5, 0),
        (0, 8),
        (0, 1),
        (0, 2),
        (8, 0),
        (3, 0),
        (0, 8),
        (0, -8),
        (0, -8),
        (0, 3),
        (3, 0),
        (6, 0),
        (0, 9),
        (0, -1),
        (6, 0),
        (0, -1),
        (0, 1),
        (0, 9),
        (3, 0),
        (0, -1),
        (7, 0),
        (6, 0),
        (1, 0),
        (0, -3),
        (0, 8),
        (7, 0),
        (0, 3),
        (0, 5),
        (0, 7),
        (6, 0),
        (0, 9),
        (9, 0),
        (8, 0),
        (0, 9),
        (1, 0),
        (0, 2),
        (0, -7),
        (0, 3),
        (0, 1),
        (8, 0),
        (4, 0),
        (9, 0),
        (0, -9),
        (0, 4),
        (1, 0),
        (0, 1),
        (0, -1),
        (0, -1),
        (0, -6),
        (0, 7),
        (0, 5),
        (1, 0),
        (7, 0),
        (0, -3),
        (0, 7),
        (0, -3),
        (0, 4),
        (0, -9),
        (0, -9),
        (1, 0),
        (0, 4),
        (0, 6),
        (2, 0),
        (6, 0),
        (0, -1),
        (1, 0),
        (0, 8),
        (7, 0),
        (0, -6),
        (6, 0),
        (3, 0),
        (0, -1),
        (0, -6),
        (1, 0),
        (0, 2),
        (8, 0),
        (4, 0),
        (2, 0),
        (0, 3),
        (2, 0),
        (3, 0),
        (1, 0),
        (0, 6),
        (7, 0),
        (7, 0),
        (0, 4),
        (6, 0),
        (0, -3),
        (0, -4),
        (0, -6),
        (0, 7),
        (0, 8),
        (3, 0),
        (0, 2),
        (5, 0),
        (0, 4),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
        (3, 0),
        (0, 1),
        (8, 0),
        (1, 0),
        (0, 8),
        (0, -1),
        (0, 3),
        (0, 6),
        (0, 1),
        (0, -1),
        (1, 0),
        (0, 6),
        (0, 5),
        (6, 0),
        (0, 1),
        (0, 5),
        (7, 0),
        (0, -3),
        (4, 0),
        (4, 0),
        (1, 0),
        (0, -6),
        (0, -2),
        (0, -4),
        (0, 4),
        (0, -4),
        (8, 0),
        (0, -8),
        (1, 0),
        (0, 5),
        (5, 0),
        (0, 7),
        (0, -5),
        (0, -7),
        (0, -5),
        (9, 0),
        (0, 1),
        (0, 1),
        (4, 0),
        (0, 2),
        (0, 2),
        (0, 3),
        (0, 1),
        (1, 0),
        (0, -7),
        (6, 0),
        (9, 0),
        (0, -5),
        (1, 0),
        (9, 0),
        (0, -2),
        (5, 0),
        (0, 4),
        (6, 0),
        (0, 9),
        (0, 3),
        (1, 0),
        (0, 2),
        (0, 3),
        (0, 1),
        (0, 3),
        (8, 0),
        (0, -6),
        (2, 0),
        (0, 5),
        (0, 9),
        (0, 4),
        (0, -2),
        (0, -9),
        (2, 0),
        (0, 7),
        (9, 0),
        (0, 5),
        (0, 5),
        (0, -6),
        (1, 0),
        (5, 0),
        (9, 0),
        (0, 4),
        (2, 0),
        (7, 0),
        (0, 2),
        (4, 0),
        (0, 2),
        (3, 0),
        (0, 3),
        (0, 2),
        (0, -5),
        (8, 0),
        (0, -8),
        (0, 9),
        (9, 0),
        (0, 9),
        (0, 4),
        (0, 1),
        (4, 0),
        (9, 0),
        (0, 5),
        (0, 9),
        (0, 4),
        (0, 5),
        (1, 0),
        (0, 3),
        (0, 3),
        (0, 4),
        (6, 0),
        (5, 0),
        (0, 3),
        (0, -4),
        (9, 0),
        (5, 0),
        (3, 0),
        (6, 0),
        (0, 8),
        (0, -9),
        (2, 0),
        (0, -6),
        (2, 0),
        (0, 9),
        (0, -9),
        (0, 4),
        (1, 0),
        (9, 0),
        (0, 5),
        (9, 0),
        (4, 0),
        (0, 6),
        (7, 0),
        (4, 0),
        (0, 7),
        (0, 1),
        (9, 0),
        (0, 6),
        (0, 5),
        (5, 0),
        (0, 5),
        (0, 1),
        (3, 0),
        (0, 7),
        (0, 5),
        (0, 9),
        (0, 5),
        (0, -6),
        (0, -5),
        (0, 5),
        (0, -1),
        (0, 9),
        (5, 0),
        (9, 0),
        (3, 0),
        (4, 0),
        (0, 7),
        (3, 0),
        (3, 0),
        (0, 5),
        (7, 0),
        (0, 9),
        (8, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (1, 0),
        (6, 0),
        (0, -9),
        (0, 3),
        (1, 0),
        (4, 0),
        (0, 2),
        (0, 8),
        (0, -4),
        (0, 4),
        (1, 0),
        (0, 5),
        (0, 3),
        (0, 9),
        (0, -1),
        (8, 0),
        (0, 6),
        (0, 4),
        (3, 0),
        (0, 8),
        (0, 2),
        (0, -6),
        (0, 5),
        (8, 0),
        (0, 4),
        (0, -1),
        (5, 0),
        (0, 1),
        (0, 9),
        (0, 1),
        (0, 9),
        (0, 3),
        (0, 3),
        (2, 0),
        (6, 0),
        (0, 8),
        (1, 0),
        (0, -4),
        (0, 3),
        (9, 0),
        (0, -2),
        (0, 4),
        (9, 0),
        (0, 3),
        (0, 1),
        (0, 3),
        (0, 4),
        (0, -6),
        (0, 2),
        (3, 0),
        (9, 0),
        (7, 0),
        (0, 2),
        (0, 5),
        (4, 0),
        (5, 0),
        (0, 9),
        (0, -3),
        (5, 0),
        (9, 0),
        (0, -2),
        (3, 0),
        (0, 4),
        (2, 0),
        (0, 5),
        (0, 8),
        (0, 1),
        (4, 0),
        (0, -4),
        (7, 0),
        (0, 9),
        (8, 0),
        (0, 8),
        (3, 0),
        (0, 6),
        (0, -9),
        (0, -6),
        (0, 2),
        (6, 0),
        (0, -1),
        (0, 5),
        (0, 5),
        (0, 9),
        (0, -2),
        (0, 2),
        (1, 0),
        (8, 0),
        (0, 2),
        (0, -8),
        (0, 3),
        (2, 0),
        (0, 1),
        (0, 5),
        (0, 5),
        (0, -4),
        (5, 0),
    ]
}
