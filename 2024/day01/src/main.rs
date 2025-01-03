use std::collections::BTreeMap;

fn parse_string(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (v1, v2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            let (first, second) = (
                words.next().expect("need two strings, empty line"),
                words.next().expect("needs two strings but had only one"),
            );
            let (first, second) = (
                first.parse::<i32>().expect("Need a number"),
                second.parse::<i32>().expect("Need a number"),
            );
            (first, second)
        })
        .unzip();
    (v1, v2)
}

fn part1(input: &str) -> i32 {
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = parse_string(input);
    v1.sort_unstable();
    v2.sort_unstable();

    let sum = v1
        .iter()
        .zip(v2.iter())
        .map(|(a, b)| {
            let delta = (a - b).abs();
            delta
        })
        .sum();
    sum
}

fn parse_similarity_score(input: &str) -> i32 {
    let (v1, v2): (Vec<i32>, Vec<i32>) = parse_string(input);
    let mut counter = BTreeMap::new();
    v2.into_iter().for_each(|x| {
        // https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.entry
        counter.entry(x).and_modify(|curr| *curr += 1).or_insert(1);
    });

    v1.iter().map(|x| {
        counter.get(x).unwrap_or(&0) * x
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1_small_sample() {
        let input = SAMPLE_INPUT;
        let result = part1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part1_real_problem() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 1530215);
    }

    #[test]
    fn test_part2_small_sample() {
        let input = SAMPLE_INPUT;
        let result = parse_similarity_score(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_part2_real_problem() {
        let input = include_str!("input.txt");
        let result = parse_similarity_score(input);
        assert_eq!(result, 26800609);
    }
}
