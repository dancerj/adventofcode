use std::collections::BTreeSet;
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

fn count_nums1478(input: &Vec<SignalPatterns>) -> HashMap<i32, u32> {
    let mut count: HashMap<i32, u32> = HashMap::new();
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

fn count_sum1478(input: &Vec<SignalPatterns>) -> usize {
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

fn char_to_value(c: u8) -> u8 {
    c - b'a'
}

fn set_of_alpha(s: &str) -> BTreeSet<u8> {
    s.as_bytes().iter().map(|&c| char_to_value(c)).collect()
}

fn to_set(input: &[u8]) -> BTreeSet<u8> {
    input.iter().cloned().collect()
}

fn compute_match(input: &SignalPatterns) {
    // fake alphabet -> real alphabet possibility map.
    let possibility: Vec<Vec<bool>> = vec![vec![true; 10]; 10];

    let mut by_len = HashMap::new();
    input.digits.iter().for_each(|digit| {
        let len = digit.len();
        let x = by_len.entry(len).or_insert(vec![]);
        let a = set_of_alpha(digit);
        println!(
            "{}",
            (0..7)
                .map(|x| if a.contains(&x) { 'O' } else { '_' })
                .collect::<String>()
        );
        x.push(a);
    });

    // OOOOOOO
    //         0,6,9 d,c,e
    // Oc_OOOO
    // O_OOOOO
    // _cOOOOO

    //         3,7,5 bf/be/ce
    // _OOOOO_ -- (4) e fixed now, b or (already fixed c. -- b)
    // __OOOOO -- (3) c fixed now, e
    // OOOO_O_ -- (5) b fixed now, (e already fixed), f.

    // _OO_O_O 4 -- bcdf (6) only remaining d.
    // _O_aO__ 7 -- (1) a-- acf
    // _c__O__ 1 --  cf ((2)c -- need to match with 0,6,9)

    // ecdaf_b -- (7) remaining g

    // (1)
    let a: Vec<u8> = by_len
        .get(&3)
        .unwrap()
        .get(0)
        .unwrap()
        .difference(by_len.get(&2).unwrap().get(0).unwrap())
        .cloned()
        .collect();

    assert_eq!(a.len(), 1);
    let a = a[0];

    const INVALID_VALUE: u8 = 10;

    // (2)
    // one of the 6's negative case is 'c' that is shared with (2)cf.
    let cf = by_len.get(&2).unwrap().get(0).unwrap();
    let mut c = INVALID_VALUE;
    cf.iter().for_each(|&cf_candidate| {
        by_len.get(&6).unwrap().iter().for_each(|x| {
            if !x.contains(&cf_candidate) {
                c = cf_candidate;
            }
        })
    });
    assert_ne!(c, INVALID_VALUE);
    let mut f = INVALID_VALUE;
    cf.iter().for_each(|&cf_candidate| {
        if cf_candidate != c {
            f = cf_candidate;
        }
    });
    assert_ne!(f, INVALID_VALUE);
    let the_7 = by_len.get(&7).unwrap().get(0).unwrap();

    // (3)
    let mut e = INVALID_VALUE;
    by_len.get(&5).unwrap().iter().for_each(|value| {
        if !value.contains(&c) {
            println!("{:?}", value);
            // This node is -ce
            the_7.difference(value).for_each(|&e_candidate| {
                if e_candidate != c {
                    e = e_candidate;
                }
            });
        }
    });
    assert_ne!(e, INVALID_VALUE);

    // (4)
    let mut b = INVALID_VALUE;
    by_len.get(&5).unwrap().iter().for_each(|value| {
        if !value.contains(&e) && value.contains(&c) {
            // This node is -be.
            the_7.difference(value).for_each(|&b_candidate| {
                if b_candidate != e {
                    b = b_candidate;
                }
            });
        }
    });
    assert_ne!(b, INVALID_VALUE);

    // (5) -- f is already there.

    // (6)
    let mut d = INVALID_VALUE;
    by_len
        .get(&4)
        .unwrap()
        .get(0)
        .unwrap()
        .iter()
        .for_each(|&d_candidate| {
            if d_candidate != b && d_candidate != c && d_candidate != f {
                d = d_candidate;
            }
        });
    assert_ne!(d, INVALID_VALUE);

    let g = *the_7
        .difference(&[a, b, c, d, e, f].iter().cloned().collect::<BTreeSet<u8>>())
        .next()
        .unwrap();

    // input.digits.iter().for_each(|digit| {
    //     match digit.len() {
    //         2 => {
    //             // the two alphabets can only mean 1, cf
    //         }
    //         3 => {
    //             // the 3 mean 7, acf
    //         }
    //         4 => {
    //             // the 4 mean 4, bcdf(-abe)
    //         }
    //         5 => {
    //             // the 5 mean 2,3,5 acdeg(-bf) acdfg(-be) abdfg(-ce)
    //             // adg == true, others not sure
    //         }
    //         6 => {
    //             // the 6 mean 0,6,9 abcefg(-d) abdefg(-c) abcdfg(-e)
    //         }
    //         7 => {
    //             // the 7 mean 8, no hint
    //         }
    //         _ => {}
    //     }
    // })
    println!("{:?}", [a,b,c,d,e,f,g]);
    let lookup : Vec<BTreeSet<u8>> = vec![
        to_set(&[a,b,c,e,f,g]),
        to_set(&[c,f]),
        to_set(&[a,c,d,e,g]),
        to_set(&[a,c,d,f,g]),
        to_set(&[b,c,d,f]),
        to_set(&[a,b,d,f,g]),
        to_set(&[a,b,d,e,f,g]),
        to_set(&[a,c,f]),
        to_set(&[a,b,c,d,e,f,g]),
        to_set(&[a,b,c,d,f,g]),
    ];
    println!("{:?}", lookup);
    //todo
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
        // 0 6
        // 1 2
        // 2 5
        // 3 5
        // 4 4
        // 5 5
        // 6 6
        // 7 3
        // 8 7
        // 9 6

        assert_eq!(SEGMENTS[1].len(), 2);
        assert_eq!(SEGMENTS[4].len(), 4);
        assert_eq!(SEGMENTS[7].len(), 3);
        assert_eq!(SEGMENTS[8].len(), 7);
    }

    #[test]
    fn test_count_nums1478() {
        let c = count_nums1478(&parse(COMMANDS));
        println!("{:?}", c);
        assert_eq!(*c.get(&1).unwrap(), 8);
        assert_eq!(c[&1], 8);
        let c = count_sum1478(&parse(COMMANDS));
        assert_eq!(c, 26);
    }

    #[test]
    fn test_count_nums1478_real() {
        let c = count_sum1478(&parse(include_str!("input.txt")));
        assert_eq!(c, 548);
    }
    // A=0,
    // B=1,
    // C=2,
    // D=3,
    // E=4,
    // F=5,
    // G=6,

    #[test]
    fn test_set_of_alpha() {
        assert_eq!(set_of_alpha("def"), vec![3, 4, 5].into_iter().collect());
        assert_eq!(set_of_alpha("fed"), vec![3, 4, 5].into_iter().collect());
    }

    #[test]
    fn test_compute_match() {
        let c = parse(COMMANDS);
        compute_match(&c[0])
    }
}
