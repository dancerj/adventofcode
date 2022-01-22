fn parse(s: &str) -> String {
    let b: String = s
        .chars()
        .map(|hex| {
            let num = hex.to_digit(16).unwrap();
            let binary: String = (0..3)
                .map(|shift| if num & (1u32 << shift) > 0 { "1" } else { "0" })
                .collect();
            binary
        })
        .collect();
    b
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "D2FE2838006F45291200EE00D40C8230608A004A801A8002F478620080001611562C8802118E34C0015000016115A2E0802F182340A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_parse() {
        let t = parse(COMMANDS);
        // panic!(t);
    }
}
