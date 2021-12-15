use std::collections::HashMap;

type Pair = (char, char);
type Transform = HashMap<Pair, char>;
type Sequence = HashMap<Pair, usize>;

fn main() {
    let input = build_map(input_sequence());
    let input_transforms = input_transformations();

    let mut result: Sequence = input;

    let mut totals = "VCOPVNKPFOOVPVSBKCOF"
        .chars()
        .fold(HashMap::new(), |mut total, char| {
            let count = total.entry(char).or_insert(0);
            *count += 1;
            total
        });

    for _i in 1..=40 {
        result = transform_sequence(result, &input_transforms, &mut totals);
    }

    assert_eq!(10002813279337, sum_of_most_and_least_common(totals));
}

fn sum_of_most_and_least_common(sequence: HashMap<char, usize>) -> usize {
    let (_x, min) = sequence
        .iter()
        .min_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
        .unwrap();

    let (_z, max) = sequence
        .iter()
        .max_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
        .unwrap();

    max - min
}

/// Ever iteration through the loop will cause total to be updated
/// with new characters introduced into the sequence.
fn transform_sequence(seq: Sequence, t: &Transform, total: &mut HashMap<char, usize>) -> Sequence {
    let mut next = HashMap::new();
    for (pair, value) in seq {
        let t = transform_pair(&t, &pair);
        let p1 = t[0];
        let p2 = t[1];

        let new_char_to_count = p1.1;
        let t = total.entry(new_char_to_count).or_insert(0);
        *t += value;

        let p1_value = next.entry(p1).or_insert(0);
        *p1_value += value;

        let p2_value = next.entry(p2).or_insert(0);
        *p2_value += value;
    }

    next
}

fn transform_pair(t: &Transform, p: &Pair) -> Vec<Pair> {
    let t = t.get(p).unwrap();
    let mut pairs = Vec::new();
    pairs.push((p.0, *t));
    pairs.push((*t, p.1));
    pairs
}

fn build_transform(pairs: Vec<(Pair, char)>) -> Transform {
    pairs.iter().fold(HashMap::new(), |mut acc, (pair, value)| {
        acc.insert(*pair, *value);
        acc
    })
}

#[allow(dead_code)]
fn test_input_transformations() -> Transform {
    let v = vec![
        (('C', 'H'), 'B'),
        (('H', 'H'), 'N'),
        (('C', 'B'), 'H'),
        (('N', 'H'), 'C'),
        (('H', 'B'), 'C'),
        (('H', 'C'), 'B'),
        (('H', 'N'), 'C'),
        (('N', 'N'), 'C'),
        (('B', 'H'), 'H'),
        (('N', 'C'), 'B'),
        (('N', 'B'), 'B'),
        (('B', 'N'), 'B'),
        (('B', 'B'), 'N'),
        (('B', 'C'), 'B'),
        (('C', 'C'), 'N'),
        (('C', 'N'), 'C'),
    ];

    build_transform(v)
}

fn input_transformations() -> Transform {
    let v = vec![
        (('N', 'O'), 'K'),
        (('P', 'O'), 'B'),
        (('H', 'S'), 'B'),
        (('F', 'P'), 'V'),
        (('K', 'N'), 'S'),
        (('H', 'V'), 'S'),
        (('K', 'C'), 'S'),
        (('C', 'S'), 'B'),
        (('K', 'B'), 'V'),
        (('O', 'B'), 'V'),
        (('H', 'N'), 'S'),
        (('O', 'K'), 'N'),
        (('P', 'C'), 'H'),
        (('O', 'O'), 'P'),
        (('H', 'F'), 'S'),
        (('C', 'B'), 'C'),
        (('S', 'B'), 'V'),
        (('F', 'N'), 'B'),
        (('P', 'H'), 'K'),
        (('K', 'H'), 'P'),
        (('N', 'B'), 'F'),
        (('K', 'F'), 'P'),
        (('F', 'K'), 'N'),
        (('F', 'B'), 'P'),
        (('F', 'O'), 'H'),
        (('C', 'V'), 'V'),
        (('C', 'N'), 'P'),
        (('B', 'N'), 'N'),
        (('S', 'C'), 'N'),
        (('P', 'B'), 'K'),
        (('V', 'S'), 'N'),
        (('B', 'P'), 'P'),
        (('C', 'K'), 'O'),
        (('P', 'S'), 'N'),
        (('P', 'F'), 'H'),
        (('H', 'B'), 'S'),
        (('V', 'N'), 'V'),
        (('O', 'S'), 'V'),
        (('O', 'C'), 'O'),
        (('B', 'B'), 'F'),
        (('S', 'K'), 'S'),
        (('N', 'F'), 'F'),
        (('F', 'S'), 'S'),
        (('S', 'N'), 'N'),
        (('F', 'C'), 'S'),
        (('B', 'H'), 'N'),
        (('H', 'P'), 'C'),
        (('V', 'K'), 'F'),
        (('C', 'C'), 'N'),
        (('S', 'V'), 'H'),
        (('S', 'O'), 'F'),
        (('H', 'H'), 'C'),
        (('P', 'K'), 'P'),
        (('N', 'V'), 'B'),
        (('K', 'S'), 'H'),
        (('N', 'P'), 'H'),
        (('V', 'O'), 'C'),
        (('B', 'K'), 'V'),
        (('V', 'V'), 'P'),
        (('H', 'K'), 'B'),
        (('C', 'F'), 'B'),
        (('B', 'F'), 'O'),
        (('O', 'V'), 'B'),
        (('O', 'H'), 'C'),
        (('P', 'P'), 'S'),
        (('S', 'P'), 'S'),
        (('C', 'H'), 'B'),
        (('O', 'F'), 'F'),
        (('N', 'K'), 'F'),
        (('F', 'V'), 'F'),
        (('K', 'P'), 'O'),
        (('O', 'P'), 'O'),
        (('S', 'S'), 'P'),
        (('C', 'P'), 'H'),
        (('B', 'O'), 'O'),
        (('K', 'K'), 'F'),
        (('H', 'C'), 'N'),
        (('K', 'O'), 'V'),
        (('C', 'O'), 'F'),
        (('N', 'C'), 'P'),
        (('O', 'N'), 'P'),
        (('K', 'V'), 'C'),
        (('B', 'V'), 'K'),
        (('H', 'O'), 'F'),
        (('P', 'V'), 'H'),
        (('V', 'C'), 'O'),
        (('N', 'H'), 'B'),
        (('P', 'N'), 'H'),
        (('V', 'P'), 'O'),
        (('N', 'S'), 'N'),
        (('N', 'N'), 'S'),
        (('B', 'S'), 'H'),
        (('S', 'H'), 'P'),
        (('V', 'B'), 'V'),
        (('V', 'H'), 'O'),
        (('F', 'H'), 'K'),
        (('F', 'F'), 'H'),
        (('S', 'F'), 'N'),
        (('B', 'C'), 'H'),
        (('V', 'F'), 'P'),
    ];

    build_transform(v)
}

#[allow(dead_code)]
fn test_input_sequence() -> Vec<(char, char)> {
    let sequence = "NNCB";
    sequence.chars().zip(sequence.chars().skip(1)).collect()
}

fn input_sequence() -> Vec<(char, char)> {
    let sequence = "VCOPVNKPFOOVPVSBKCOF";
    sequence.chars().zip(sequence.chars().skip(1)).collect()
}

fn build_map(input: Vec<(char, char)>) -> HashMap<(char, char), usize> {
    input.iter().fold(HashMap::new(), |mut result, c| {
        let count = result.entry(*c).or_insert(0);
        *count += 1;

        result
    })
}
