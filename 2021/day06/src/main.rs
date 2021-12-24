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
}
