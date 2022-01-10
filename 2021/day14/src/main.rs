use std::collections::HashMap;

fn parse(s: &str) -> (&str, HashMap<&str, &str>) {
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    assert_eq!(lines.next().unwrap(), "");
    let rules = lines
        .map(|line| {
            let mut words = line.split(" ");
            match (
                words.next().unwrap(),
                words.next().unwrap(),
                words.next().unwrap(),
            ) {
                (origin, "->", target) => (origin, target),
                _ => {
                    panic!();
                }
            }
        })
        .collect();

    (first_line, rules)
}

fn apply(s: &str, rules: &HashMap<&str, &str>) -> String {
    assert!(s.len() > 1);
    let mut result: String = "".to_string();
    for pos in 0..(s.len() - 1) {
        result.push(s.chars().nth(pos).unwrap());
        if let Some(got) = rules.get(s.get(pos..=pos + 1).unwrap()) {
            result.push(got.chars().nth(0).unwrap());
        }
    }
    result.push(s.chars().nth(s.len() - 1).unwrap());
    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_parse() {
        let (s, rules) = parse(COMMANDS);
        let s1 = apply(s, &rules);
        assert_eq!(s1, "NCNBCHB");
        let s2 = apply(&s1, &rules);
        assert_eq!(s2, "NBCCNBBBCBHCB");
    }
}
