fn is_fake(input: &str) -> bool {
    let len = input.len();
    let half = &input[0..(len / 2)];
    let double = half.to_owned() + half;
    input == double
}

fn part1(input: &str) -> u64 {
    let total = input
        .trim_end()
        .split(',')
        .map(|range| {
            let mut count = 0;
            dbg!(range);
            let mut range = range.split('-');
            let start: u64 = range
                .next()
                .expect("start exected")
                .parse()
                .expect("number");
            let end: u64 = range
                .next()
                .expect("end expected after -")
                .parse()
                .expect("number");
            assert_eq!(range.next(), None);
            for i in start..=end {
                // evaluate
                if is_fake(&i.to_string()) {
                    count += i;
                }
            }
            count
        })
        .sum();
    total
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_small_sample() {
        let input = SAMPLE_INPUT;
        let result = part1(input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part1_real_problem() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 31000881061);
    }
}
