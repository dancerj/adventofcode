#[derive(Debug, PartialEq)]
struct Num {
    n: u32,
    depth: u32,
}

fn parse(s: &str) -> Vec<Vec<Num>> {
    s.lines()
        .map(|line| {
            let mut depth = 0;
            let mut result = vec![];
            for c in line.chars() {
                match c {
                    '[' => {
                        depth += 1;
                    }
                    ']' => {
                        depth -= 1;
                    }
                    ',' => {
                        // number follows on the right.
                    }
                    c if c.is_ascii_digit() => {
                        let n = c.to_digit(10).unwrap();
                        result.push(Num { n, depth });
                    }
                    _ => {
                        panic!()
                    }
                }
            }
            result
        })
        .collect::<Vec<Vec<Num>>>()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMAND: &str = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";

    #[test]
    fn test_parse() {
        let r = parse(&COMMAND);
        assert_eq!(r[0][0], Num { n: 1, depth: 1 });
    }
}
