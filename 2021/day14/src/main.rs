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

fn get_count(s: &str) -> Vec<(u64, char)> {
    let mut map: HashMap<char, u64> = HashMap::new();
    s.chars().for_each(|c| {
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
    });
    let mut by_count: Vec<(u64, char)> = map.iter().map(|(&c, &count)| (count, c)).collect();
    by_count.sort();
    // println!("{} {:?}", s, by_count);
    by_count
}

fn apply_n(n: u32, s: &str, rules: &HashMap<&str, &str>) -> u64 {
    let mut result = s.to_string();
    (0..n).for_each(|_| {
        result = apply(&result, rules);
    });
    let by_count = get_count(&result);
    let most_common_element = by_count[by_count.len() - 1];
    let least_common_element = by_count[0];
    most_common_element.0 - least_common_element.0
}

// We just need the bigram and not the actual order, so bigram->count map is good enough.
struct Bigrams {
    // Needs String because we construct it later on.
    count: HashMap<String, u64>,
    start: char,
    end: char,
}

fn convert_to_bigram(s: &str) -> Bigrams {
    let mut result = Bigrams {
        count: HashMap::new(),
        start: s.chars().nth(0).unwrap(),
        end: s.chars().nth(s.len() - 1).unwrap(),
    };
    for i in 0..s.len() - 1 {
        *result.count.entry(s[i..=i + 1].to_string()).or_insert(0) += 1
    }

    // dbg!(&result.count);
    result
}

fn apply_bigram(b: &Bigrams, rules: &HashMap<&str, &str>) -> Bigrams {
    let mut result = Bigrams {
        count: HashMap::new(),
        start: b.start,
        end: b.end,
    };

    for (bigram, count) in b.count.iter() {
        if let Some(&got) = rules.get(&bigram as &str) {
            *result
                .count
                .entry(bigram[0..=0].to_owned() + got)
                .or_insert(0) += count;
            *result
                .count
                .entry(got.to_owned() + &bigram[1..=1])
                .or_insert(0) += count;
        } else {
            *result.count.entry(bigram.to_string()).or_insert(0) += count;
        }
    }
    result
}

fn count_bigram(b: &Bigrams) -> Vec<(u64, char)> {
    let mut map: HashMap<char, u64> = HashMap::new();
    for (bigram, count) in b.count.iter() {
        *map.entry(bigram.chars().nth(0).unwrap()).or_insert(0) += count;
        *map.entry(bigram.chars().nth(1).unwrap()).or_insert(0) += count;
    }
    *map.entry(b.start).or_insert(0) += 1;
    *map.entry(b.end).or_insert(0) += 1;

    // half the count here.
    let mut by_count: Vec<(u64, char)> = map.iter().map(|(&c, &count)| (count / 2, c)).collect();
    by_count.sort();
    // println!("{} {:?}", b, by_count);
    by_count
}

fn apply_n_bigram(n: u64, s: &str, rules: &HashMap<&str, &str>) -> u64 {
    let mut result = convert_to_bigram(&s);
    (0..n).for_each(|_| {
        result = apply_bigram(&result, rules);
    });
    let by_count = count_bigram(&result);
    let most_common_element = by_count[by_count.len() - 1];
    let least_common_element = by_count[0];
    most_common_element.0 - least_common_element.0
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
        let c = apply_n(10, &s, &rules);
        assert_eq!(c, 1588);
    }

    #[test]
    fn test_parse_real() {
        let (s, rules) = parse(include_str!("input.txt"));
        let c = apply_n_bigram(10, &s, &rules);
        assert_eq!(c, 3284);
    }

    #[test]
    fn test_bigram() {
        let (s, rules) = parse(COMMANDS);
        let b = convert_to_bigram(&s);
        let b = apply_bigram(&b, &rules);
        count_bigram(&b);

        let c = apply_n_bigram(40, &s, &rules);
        assert_eq!(c, 2188189693529);
    }
    #[test]
    fn test_bigram_real() {
        let (s, rules) = parse(include_str!("input.txt"));
        let b = convert_to_bigram(&s);
        let b = apply_bigram(&b, &rules);
        count_bigram(&b);

        let c = apply_n_bigram(40, &s, &rules);
        assert_eq!(c, 4302675529689);
    }
}
