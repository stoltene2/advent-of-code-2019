type Bits = Vec<char>;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal { version: usize, value: usize },
}

const LITERAL: usize = 4;
const OPERATOR: usize = 3;

impl Packet {
    fn parse(mut s: Bits) -> Packet {
        println!("s.len() = {}", s.len());
        let substring = s.drain(0..3).collect();
        let version: usize = binary_string_to_num(&substring);

        let substring = s.drain(0..3).collect::<Bits>();
        let type_id: usize = binary_string_to_num(&substring);

        if type_id == LITERAL {
            let value = parse_literal_value(s);
            return Packet::Literal { version, value };
        } else {
            panic!("oops")
        }
    }
}

fn parse_literal_value(mut remaining: Bits) -> usize {
    let mut value = 0;
    let mut end = false;
    let mut bits_consumed = 6; // type and version were already parsed

    while !end {
        let bits: Bits = remaining.drain(0..5).collect();

        if bits[0] == '0' {
            end = true;
        }

        value += binary_string_to_num(&bits[1..5].to_vec());
        bits_consumed += 5;
    }

    // Drop padded bits if any
    if bits_consumed % 4 != 0 {
        let to_drop = 4 - (bits_consumed % 4);
        remaining.drain(0..to_drop);
    }

    value
}

fn main() {
    let binary_str = hex_string_to_binary_string(&String::from("D2FE28"));
    let r = Packet::parse(binary_str);

    println!("r: {:?}", &r);

    assert_eq!(
        &Packet::Literal {
            version: 6,
            value: 26
        },
        &r
    );
}

fn hex_string_to_binary_string(s: &String) -> Bits {
    let mut new_s = Vec::with_capacity(4 * s.len());

    for c in s.chars() {
        new_s.append(&mut hex_char_to_binary_string(c));
    }

    new_s
}

fn binary_string_to_num(s: &Bits) -> usize {
    let mut result = 0;
    for (i, num) in s.iter().rev().enumerate() {
        if num == &'1' {
            result += 1 << i;
        }
    }

    result
}

fn hex_char_to_binary_string(c: char) -> Bits {
    let s = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        not_hex => panic!("Not valid hex, {}", not_hex),
    };

    s.chars().collect()
}
