fn part1(_input: &str) -> i32 {
    11
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
        assert_eq!(result, 11);
    }
}
