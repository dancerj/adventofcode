use std::collections::BTreeMap;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
enum Relative {
    Positive,
    Negative,
    Invalid,
}

fn parse_line(line: &str) -> Vec<Relative> {
    let line = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().expect("Need a number"))
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|arr| {
            let x = arr.get(0).expect("Need a first number");
            let y = arr
                .get(1)
                .expect("Need two numbers. windows should give me two numbers, why?");
            let relative = match x - y {
                1..=3 => Relative::Positive,
                -3..=-1 => Relative::Negative,
                _ => Relative::Invalid,
            };
            relative
        })
        .collect::<Vec<_>>();
    if line.len() < 2 {
        panic!("I didn't think about what to do when there's less than 2 docs");
    }
    line
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let line = parse_line(line);
            if !line.windows(2).all(|window| window[0] == window[1]) {
                return false;
            }
            if line[0] == Relative::Invalid {
                return false;
            }
            true
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let line = parse_line(line);
            let mut counter = BTreeMap::new();
            line.iter().for_each(|x| {
                counter.entry(x).and_modify(|curr| *curr += 1).or_insert(1);
            });

            let size = line.len();
            // TODO: I don't need to insert the value?
            *counter.get(&Relative::Positive).unwrap_or(&0) >= size - 1
                || *counter.get(&Relative::Negative).unwrap_or(&0) >= size - 1
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1_small_sample() {
        let input = SAMPLE_INPUT;
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_real_problem() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 472);
    }

    #[test]
    fn test_part2_small_sample() {
        let input = SAMPLE_INPUT;
        let result = part2(input);

        // All of them, no that's not right.
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_real_problem() {
        let input = include_str!("input.txt");
        let result = part2(input);
        // The system says this is too high.
        assert_eq!(result, 525);
    }
}
