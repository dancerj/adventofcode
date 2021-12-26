use std::collections::HashMap;

const SEGMENTS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

#[derive(Debug)]
struct SignalPatterns<'a> {
    digits: Vec<&'a str>,
    output: Vec<&'a str>,
}

fn parse(s: &str) -> Vec<SignalPatterns> {
    s.lines()
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            assert_eq!(words[10], "|");
            SignalPatterns {
                digits: words[0..10].to_vec(),
                output: words[11..15].to_vec(),
            }
        })
        .collect()
}

fn count_nums1478(input: Vec<SignalPatterns>) -> HashMap<i32, u32> {
    let mut count: HashMap<i32,u32> = HashMap::new();
    input
        .iter()
        .map(|signal_patterns| {
            signal_patterns
                .output
                .iter()
                .filter_map(|output| match output.len() {
                    2 => Some(1),
                    4 => Some(4),
                    3 => Some(7),
                    7 => Some(8),
                    _ => None,
                })
        })
        .flatten()
        .for_each(|value| {
            let x = count.entry(value).or_insert(0);
                *x += 1;

        });
    count
}

fn count_sum1478(input: Vec<SignalPatterns>) -> usize{
    input
        .iter()
        .map(|signal_patterns| {
            signal_patterns
                .output
                .iter()
                .filter_map(|output| match output.len() {
                    2 => Some(1),
                    4 => Some(4),
                    3 => Some(7),
                    7 => Some(8),
                    _ => None,
                })
        })
        .flatten()
        .count()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_parse_str() {
        println!("{:?}", parse(COMMANDS));
    }

    #[test]
    fn test_segments() {
        (0..10).for_each(|x| {
            println!("{} {}", x, SEGMENTS[x].len());
        });
        assert_eq!(SEGMENTS[1].len(), 2);
        assert_eq!(SEGMENTS[4].len(), 4);
        assert_eq!(SEGMENTS[7].len(), 3);
        assert_eq!(SEGMENTS[8].len(), 7);
    }

    #[test]
    fn test_count_nums1478() {
        let c = count_nums1478(parse(COMMANDS));
        println!("{:?}", c);
        assert_eq!(*c.get(&1).unwrap(), 8);
        assert_eq!(c[&1], 8);
        let c = count_sum1478(parse(COMMANDS));
        assert_eq!(c, 26);
    }

    #[test]
    fn test_count_nums1478_real() {
        let c = count_sum1478(parse(include_str!("input.txt")));
        assert_eq!(c, 548);
    }

}
