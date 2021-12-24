use std::fmt::Debug;
use std::str::FromStr;
use std::time::Instant;

fn parse_str<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    s.split(",").map(|x| x.trim().parse().unwrap()).collect()
}

fn iterate(input: &Vec<u32>) -> Vec<u32> {
    input
        .iter()
        .map(|&x| if x == 0 { vec![6, 8] } else { vec![x - 1] })
        .flatten()
        .collect()
}

fn iterate_days(days: u32, input: &Vec<u32>) -> Vec<u32> {
    let mut next: Vec<u32> = input.to_vec();
    (0..days).for_each(|day| {
        // println!("{} {:?}", day, next);
        println!("{} {}", day, next.len());
        next = iterate(&next);
    });
    next.clone()
}

fn less_allocation_iterate(days: u16, input: &Vec<u8>) -> Vec<u8> {
    let mut current: Vec<u8> = input.to_vec();
    (0..days).for_each(|day| {
        let now = Instant::now();
        let mut to_push = vec![];
        (0..current.len()).for_each(|n| {
            if current[n] == 0 {
                current[n] = 6;
                to_push.push(8);
            } else {
                current[n] -= 1;
            }
        });
        current.extend_from_slice(&to_push);
        let elapsed_time = now.elapsed();
        println!("{} {} {:?}", day, current.len(), elapsed_time);
    });
    current
}

/* Returns a day -> count map.*/
fn convert_to_count(input: &Vec<u8>) -> Vec<u64> {
    let mut result = vec![0; 9];
    input.iter().for_each(|&x| {
        result[x as usize] += 1;
    });
    result
}

fn iterate_counts(days: u16, input: &Vec<u64>) -> u64 {
    let mut current: Vec<u64> = input.to_vec();
    (0..days).for_each(|day| {
        let mut new = vec![0, 0, 0, 0, 0, 0, current[0], 0, current[0]];
        // println!("1  {:?} ", new);

        (1..=8).for_each(|n| {
            new[n - 1] += current[n];
        });

        // println!("2  {:?} ", new);
        current = new;
        println!("{} {}", day, current.iter().sum::<u64>());
        // println!("3  {:?}", current);
    });
    current.iter().sum::<u64>()
}

fn main() {
    // let input = vec![3, 4, 3, 1, 2];
    let input = parse_str::<u8>(include_str!("input.txt"));
    println!("{}", less_allocation_iterate(256, &input).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_str() {
        assert_eq!(parse_str::<u32>(COMMANDS), [3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_iterate_days_short() {
        let input = parse_str::<u32>(COMMANDS);
        assert_eq!(iterate_days(18, &input).len(), 26);
        assert_eq!(iterate_days(80, &input).len(), 5934);
    }

    #[test]
    fn test_iterate_days_real() {
        let input = parse_str::<u32>(include_str!("input.txt"));
        assert_eq!(iterate_days(18, &input).len(), 1565);
        assert_eq!(iterate_days(80, &input).len(), 352195);
    }

    #[test]
    fn test_iterate_days_less_allocation() {
        let input = parse_str::<u8>(COMMANDS);
        assert_eq!(less_allocation_iterate(18, &input).len(), 26);
        // assert_eq!(less_allocation_iterate(80, &input).len(), 5934);
        // assert_eq!(less_allocation_iterate(256, &input).len(), 5934);
    }

    #[test]
    fn test_convert_to_count() {
        assert_eq!(convert_to_count(&vec![8]), [0, 0, 0, 0, 0, 0, 0, 0, 1]);

        let input = parse_str::<u8>(COMMANDS);
        assert_eq!(convert_to_count(&input), [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_count() {
        let input = parse_str::<u8>(COMMANDS);
        let count = convert_to_count(&input);
        assert_eq!(count.len(), 9);
        assert_eq!(iterate_counts(18, &count), 26);
        assert_eq!(iterate_counts(256, &count), 26984457539);
    }

    #[test]
    fn test_count_real() {
        let input = parse_str::<u8>(include_str!("input.txt"));
        let count = convert_to_count(&input);
        assert_eq!(iterate_counts(256, &count), 1600306001288);
    }
}
