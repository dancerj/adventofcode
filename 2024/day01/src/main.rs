fn main() {
    println!("Hello, world!");
}

fn parse_input_string(input: &str) -> i32 {
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            if let (Some(first), Some(second)) = (words.next(), words.next()) {
                if let (Ok(first), Ok(second)) = (first.parse::<i32>(), second.parse::<i32>()) {
                    (first, second)
                } else {
                    panic!("// had non-number string?");
                }
            } else {
                panic!("// had empty lines?");
            }
        })
        .unzip();
    v1.sort_unstable();
    v2.sort_unstable();

    let sum = v1
        .into_iter()
        .zip(v2.into_iter())
        .map(|(a, b)| {
            let delta = (a - b).abs();
            delta
        })
        .sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_sample() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = parse_input_string(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_parse_str() {
        let input = include_str!("input.txt");
        let result = parse_input_string(input);
        assert_eq!(result, 1530215);
    }
}
