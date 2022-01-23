use std::convert::TryInto;

struct BitReader {
    data: String,
    offset: usize,
}

impl BitReader {
    fn new(s: &str) -> Self {
        Self {
            data: s.to_string(),
            offset: 0,
        }
    }

    fn read(&mut self, n: usize) -> u32 {
        let result = (0..n)
            .map(|shift| {
                if self.data.chars().nth(self.offset + shift) == Some('1') {
                    1u32 << (n - shift - 1)
                } else {
                    0
                }
            })
            .sum();
        self.offset += n;
        result
    }

    fn read_literal(&mut self) -> u32 {
        // prerequisite is that previous reads were 6, 4 (literal
        // value).
        let mut result = 0;
        loop {
            let cont = self.read(1) > 0;
            let value = self.read(4);
            result <<= 4;
            result += value;
            if !cont {
                break;
            }
        }
        result
    }

    fn add_version_numbers_internal(&mut self) -> u32 {
        let mut version = self.read(3);
        let t = self.read(3);
        if t == PacketType::Literal as u32 {
            println!("literal {}", version);
            self.read_literal();
        } else {
            match self.read(1) {
                0 => {
                    // length of packet
                    let l = self.read(15);
                    println!("component of len {}, version {}", l, version);
                    let start = self.offset;
                    while (self.offset - start) < l.try_into().unwrap() {
                        version += self.add_version_numbers_internal();
                    }
                }
                1 => {
                    // number of sub-packets
                    let n = self.read(11);
                    println!("component of {} sub packets, version {} ", n, version);
                    for _ in 0..n {
                        version += self.add_version_numbers_internal();
                    }
                }
                _ => {
                    panic!()
                }
            };
        };
        version
    }

    fn parse_operators(&mut self) -> u32 {
        let mut values: Vec<u32> = vec![];
        let _version = self.read(3);
        let t = self.read(3);
        if t == PacketType::Literal as u32 {
            values.push(self.read_literal());
            println!("literal {:?}", values);
        } else {
            match self.read(1) {
                0 => {
                    // length of packet
                    let l = self.read(15);
                    println!("component of len {}", l);
                    let start = self.offset;
                    while (self.offset - start) < l.try_into().unwrap() {
                        values.push(self.parse_operators());
                    }
                }
                1 => {
                    // number of sub-packets
                    let n = self.read(11);

                    println!("component of {} sub packets", n);
                    for _ in 0..n {
                        values.push(self.parse_operators());
                    }
                }
                _ => {
                    panic!()
                }
            };
        }

        let t: PacketType = unsafe { std::mem::transmute(t as i8) };

        match t {
            PacketType::Sum => values.iter().sum::<u32>(),
            PacketType::Product => {
                println!("product {:?}", values);
                values.iter().product::<u32>()
            },
            PacketType::Minimum => *values.iter().min().unwrap(),
            PacketType::Maximum => *values.iter().max().unwrap(),
            PacketType::Literal => {
                assert_eq!(values.len(), 1);
                values[0]
            }
            PacketType::GreaterThan => {
                assert_eq!(values.len(), 2);
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            PacketType::LessThan => {
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            PacketType::EqualTo => {
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn sync_to_alignment(&mut self) {
        // Hex values are 8 bit aligned.
        if self.offset % 8 != 0 {
            assert!(self.read(8 - self.offset % 8) == 0)
        }
    }
}

fn parse(s: &str) -> BitReader {
    let b: String = s
        .trim()
        .chars()
        .map(|hex| {
            let num = hex.to_digit(16).unwrap();
            let binary: String = (0..4)
                .map(|shift| {
                    if num & (1u32 << (4 - shift - 1)) > 0 {
                        "1"
                    } else {
                        "0"
                    }
                })
                .collect();
            binary
        })
        .collect();
    BitReader::new(&b)
}

fn main() {}

#[allow(dead_code)] // I never construct these, they are converted from int.
#[repr(i8)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "D2FE2838006F45291200EE00D40C8230608A004A801A8002F478620080001611562C8802118E34C0015000016115A2E0802F182340A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_parse() {
        let mut t = parse(COMMANDS);
        // D2FE28
        assert_eq!(t.read(3), 6);
        assert_eq!(t.read(3), PacketType::Literal as u32);
        assert_eq!(t.read_literal(), 2021);
        t.sync_to_alignment();

        // 38006F45291200
        assert_eq!(t.read(3), 1);
        assert_eq!(t.read(3), PacketType::LessThan as u32);
        assert_eq!(t.read(1), 0); // length type ID.
        assert_eq!(t.read(15), 27); // length.
        t.read(27); // Skip.
        t.sync_to_alignment();

        // EE00D40C823060
        assert_eq!(t.read(3), 7);
        assert_eq!(t.read(3), PacketType::Maximum as u32);
        assert_eq!(t.read(1), 1); // length type ID.
        assert_eq!(t.read(11), 3); // number of sub-packets
    }

    #[test]
    fn test_add_version_numbers_internal() {
        let mut t = parse(COMMANDS);
        assert_eq!(t.add_version_numbers_internal(), 6);
        assert_eq!(
            parse("8A004A801A8002F478").add_version_numbers_internal(),
            16
        );
        assert_eq!(
            parse("620080001611562C8802118E34").add_version_numbers_internal(),
            12
        );
        assert_eq!(
            parse("C0015000016115A2E0802F182340").add_version_numbers_internal(),
            23
        );
        assert_eq!(
            parse("A0016C880162017C3686B18A3D4780").add_version_numbers_internal(),
            31
        );
    }

    #[test]
    fn test_add_version_numbers_internal_real() {
        assert_eq!(
            parse(include_str!("input.txt")).add_version_numbers_internal(),
            940
        );
    }

    #[test]
    fn test_parse_operators() {
        assert_eq!(parse("C200B40A82").parse_operators(), 3);
        assert_eq!(parse("04005AC33890").parse_operators(), 54);
        assert_eq!(parse("880086C3E88112").parse_operators(), 7);
        assert_eq!(parse("CE00C43D881120").parse_operators(), 9);
        assert_eq!(parse("D8005AC2A8F0").parse_operators(), 1);
        assert_eq!(parse("F600BC2D8F").parse_operators(), 0);
        assert_eq!(parse("9C005AC2F8F0").parse_operators(), 0);
        assert_eq!(parse("9C0141080250320F1802104A08").parse_operators(), 1);
    }
}
