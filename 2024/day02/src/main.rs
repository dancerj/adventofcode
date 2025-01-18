#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Relative {
    Positive,
    Negative,
    Invalid,
}
fn parse_ascii(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<i32>().expect("Need a number"))
        .collect()
}

fn parse_sequence(line: &Vec<i32>) -> Vec<Relative> {
    let line = line
        .windows(2)
        .map(|arr| {
            let x = arr.get(0).expect("Need a first number");
            let y = arr
                .get(1)
                .expect("Need two numbers. windows should give me two numbers, why?");
            let relative = match y - x {
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

fn is_valid(line: &Vec<Relative>) -> bool {
    if !line.windows(2).all(|window| window[0] == window[1]) {
        return false;
    }
    if line[0] == Relative::Invalid {
        return false;
    }
    true
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_valid(&parse_sequence(&parse_ascii(line))))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let seq = parse_ascii(line);
            let line = parse_sequence(&seq);
            if is_valid(&line) {
                return true;
            }

            for r in 0..seq.len() {
                let partial: Vec<i32> = seq
                    .iter()
                    .enumerate()
                    .filter(|(pos, _)| *pos != r)
                    .map(|(_, value)| *value)
                    .collect();
                if is_valid(&parse_sequence(&partial)) {
                    return true;
                }
            }
            false
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
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_real_problem() {
        let input = include_str!("input.txt");
        let result = part2(input);
        // The system says this is too high.
        assert_eq!(result, 520);
    }
}
