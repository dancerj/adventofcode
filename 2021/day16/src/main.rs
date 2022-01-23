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

    fn sync_to_alignment(&mut self) {
        // Hex values are 8 bit aligned.
        if self.offset % 8 != 0 {
            assert!(self.read(8 - self.offset % 8) == 0)
        }
    }
}

fn parse(s: &str) -> BitReader {
    let b: String = s
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

enum PacketType {
    Literal = 4,
    OperatorA = 6,
    OperatorB = 3,
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
        assert_eq!(t.read(3), PacketType::OperatorA as u32);
        assert_eq!(t.read(1), 0); // length type ID.
        assert_eq!(t.read(15), 27); // length.
        t.read(27); // Skip.
        t.sync_to_alignment();

        // EE00D40C823060
        assert_eq!(t.read(3), 7);
        assert_eq!(t.read(3), PacketType::OperatorB as u32);
        assert_eq!(t.read(1), 1); // length type ID.
        assert_eq!(t.read(11), 3); // number of sub-packets
    }
}
