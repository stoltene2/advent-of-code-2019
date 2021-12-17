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
    fn sum_versions(&self) -> usize {
        match self {
            Packet::Literal { version, value: _ } => *version,
            Self::OperatorFixedSize {
                version,
                sub_packets,
            } => {
                let sub_packet_total: usize = sub_packets.iter().map(|p| p.sum_versions()).sum();
                *version + sub_packet_total
            }
            Self::OperatorFixedCount {
                version,
                sub_packets,
            } => {
                let sub_packet_total: usize = sub_packets.iter().map(|p| p.sum_versions()).sum();
                *version + sub_packet_total
            }
        }
    }
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

    let binary_str = hex_string_to_binary_string(&String::from(input()));
    let r = Packet::parse(binary_str);

    //println!("\ninput: {:?}", &r);
    let total = r.0.sum_versions();
    println!("sum: {}", total);
    assert_eq!(940, total);

    // assert_eq!(
    //     Packet::OperatorFixedCount {
    //         version: 7,
    //         sub_packets: vec![
    //             Packet::Literal {
    //                 version: 2,
    //                 value: 1
    //             },
    //             Packet::Literal {
    //                 version: 4,
    //                 value: 2
    //             },
    //             Packet::Literal {
    //                 version: 1,
    //                 value: 3
    //             }
    //         ]
    //     },
    //     r.0
    // );
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

fn input() -> &'static str {
    "E0525D9802FA00B80021B13E2D4260004321DC648D729DD67B2412009966D76C0159ED274F6921402E9FD4AC1B0F652CD339D7B82240083C9A54E819802B369DC0082CF90CF9280081727DAF41E6A5C1B9B8E41A4F31A4EF67E2009834015986F9ABE41E7D6080213931CB004270DE5DD4C010E00D50401B8A708E3F80021F0BE0A43D9E460007E62ACEE7F9FB4491BC2260090A573A876B1BC4D679BA7A642401434937C911CD984910490CCFC27CC7EE686009CFC57EC0149CEFE4D135A0C200C0F401298BCF265377F79C279F540279ACCE5A820CB044B62299291C0198025401AA00021D1822BC5C100763A4698FB350E6184C00A9820200FAF00244998F67D59998F67D5A93ECB0D6E0164D709A47F5AEB6612D1B1AC788846008780252555097F51F263A1CA00C4D0946B92669EE47315060081206C96208B0B2610E7B389737F3E2006D66C1A1D4ABEC3E1003A3B0805D337C2F4FA5CD83CE7DA67A304E9BEEF32DCEF08A400020B1967FC2660084BC77BAC3F847B004E6CA26CA140095003900BAA3002140087003D40080022E8C00870039400E1002D400F10038C00D100218038F400B6100229500226699FEB9F9B098021A00800021507627C321006E24C5784B160C014A0054A64E64BB5459DE821803324093AEB3254600B4BF75C50D0046562F72B1793004667B6E78EFC0139FD534733409232D7742E402850803F1FA3143D00042226C4A8B800084C528FD1527E98D5EB45C6003FE7F7FCBA000A1E600FC5A8311F08010983F0BA0890021F1B61CC4620140EC010100762DC4C8720008641E89F0866259AF460C015D00564F71ED2935993A539C0F9AA6B0786008D80233514594F43CDD31F585005A25C3430047401194EA649E87E0CA801D320D2971C95CAA380393AF131F94F9E0499A775460"
}
