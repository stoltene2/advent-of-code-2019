type Bits = Vec<char>;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    OperatorFixedSize {
        version: usize,
        sub_packets: Vec<Packet>,
    },
    OperatorFixedCount {
        version: usize,
        sub_packets: Vec<Packet>,
    },
}

const LITERAL: usize = 4;

impl Packet {
    fn parse(mut s: Bits) -> (Packet, Bits) {
        // print!("Parsing: ");
        // pp_bits(&s);

        let substring = s.drain(0..3).collect();
        let version: usize = binary_string_to_num(&substring);

        let substring = s.drain(0..3).collect::<Bits>();
        let type_id: usize = binary_string_to_num(&substring);

        if type_id == LITERAL {
            let value = parse_literal_value(s);
            return (
                Packet::Literal {
                    version,
                    value: value.0,
                },
                value.1,
            );
        } else {
            // Operators
            let length_type_id: Bits = s.drain(0..1).collect();

            if length_type_id[0] == '0' {
                // total length in bits (15)
                let sub_packets_size = binary_string_to_num(&s.drain(0..15).collect::<Bits>());

                let mut sub_packets_bits: Bits = s.drain(0..sub_packets_size).collect();

                //                print!("subp   : ");
                //pp_bits(&sub_packets_bits);

                let mut sub_packets: Vec<Packet> = Vec::new();

                while !sub_packets_bits.is_empty() {
                    let result = Packet::parse(sub_packets_bits);
                    sub_packets_bits = result.1;
                    let packet = result.0;
                    sub_packets.push(packet);
                }

                return (
                    Packet::OperatorFixedSize {
                        version,
                        sub_packets,
                    },
                    s,
                );
            } else {
                // total number of subpackets
                // expecting val(11_bits) of records
                let sub_packet_count = binary_string_to_num(&s.drain(0..11).collect::<Bits>());
                let mut sub_packets: Vec<Packet> = Vec::new();

                for i in 0..sub_packet_count {
                    let r = Packet::parse(s);
                    s = r.1;
                    sub_packets.push(r.0);
                }

                return (
                    Packet::OperatorFixedCount {
                        version,
                        sub_packets,
                    },
                    s,
                );
            }
        }
    }
}

fn parse_literal_value(mut remaining: Bits) -> (usize, Bits) {
    let mut value: Bits = Vec::new();

    loop {
        let bits: Bits = remaining.drain(0..5).collect();
        // print!("consumed: ");
        // pp_bits(&bits);
        // print!("remaining: ");
        // pp_bits(&bits);

        let mut sub = bits[1..5].to_vec();
        value.append(&mut sub);
        //        value += binary_string_to_num(&bits[1..5].to_vec());

        if bits[0] == '0' {
            break;
        }
    }

    // // Drop padded bits if any
    // if bits_consumed % 4 != 0 {
    //     let to_drop = 4 - (bits_consumed % 4);
    //     println!("Dropping {} bits", to_drop);
    //     remaining.drain(0..to_drop);
    // }

    let v = binary_string_to_num(&value);
    (v, remaining)
}

fn pp_bits(bits: &Bits) {
    for c in bits {
        print!("{}", c);
    }
    println!("");
}

fn main() {
    let binary_str = hex_string_to_binary_string(&String::from("D2FE28"));
    let r = Packet::parse(binary_str);
    assert_eq!(
        &Packet::Literal {
            version: 6,
            value: 2021
        },
        &r.0
    );

    let binary_str = hex_string_to_binary_string(&String::from("38006F45291200"));
    let r = Packet::parse(binary_str);

    println!("\nexample 2: {:?}", &r);

    assert_eq!(
        Packet::OperatorFixedSize {
            version: 1,
            sub_packets: vec![
                Packet::Literal {
                    version: 6,
                    value: 10
                },
                Packet::Literal {
                    version: 2,
                    value: 20
                }
            ]
        },
        r.0
    );

    let binary_str = hex_string_to_binary_string(&String::from("EE00D40C823060"));
    let r = Packet::parse(binary_str);

    println!("\nexample 3: {:?}", &r);

    assert_eq!(
        Packet::OperatorFixedCount {
            version: 7,
            sub_packets: vec![
                Packet::Literal {
                    version: 2,
                    value: 1
                },
                Packet::Literal {
                    version: 4,
                    value: 2
                },
                Packet::Literal {
                    version: 1,
                    value: 3
                }
            ]
        },
        r.0
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
