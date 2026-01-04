fn find_largest_joltage(input: &str) -> u64 {
    if let Some((index_1, value_1)) = input[0..(input.len() - 1)]
        .chars()
        .enumerate()
        // Don't use max_by_key because it tries to keep the last max
        // item, manually compare with index too as to prioritize the
        // first occurrence.
        .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then_with(|| i2.cmp(i1)))
    {
        // First max value
        dbg!(index_1);
        if let Some(value_2) = input[(index_1 + 1)..].chars().max() {
            let mut s = value_1.to_string();
            s.push(value_2);
            return dbg!(s).parse().expect("A number");
        }
    }
    panic!();
}

fn part1(input: &str) -> u64 {
    input.lines().map(|line| find_largest_joltage(line)).sum()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1_small_sample() {
        let input = SAMPLE_INPUT;
        let result = part1(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_joltage() {
        assert_eq!(find_largest_joltage("218984"), 98);
        assert_eq!(find_largest_joltage("218984595"), 99);
    }

    #[test]
    fn test_part1_real_problem() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 16973);
    }
}
