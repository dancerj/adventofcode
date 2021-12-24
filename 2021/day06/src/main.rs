fn parse_str(s: &str) -> Vec<u32> {
    s.split(",").map(|x| /*dbg!*/(x).trim().parse().unwrap()).collect()
}

fn iterate(input: &Vec<u32>) -> Vec<u32> {
    input
        .iter()
        .map(|&x| if x == 0 { vec![6, 8] } else { vec![x - 1] })
        .flatten()
        .collect()
}

fn iterate_days(days: u32, input: &Vec<u32>) -> Vec<u32>{
    let mut next : Vec<u32> = input.to_vec();
    (0..days).for_each(|_day| {
        // println!("{} {:?}", day, next);
        next = iterate(&next);
    });
    next.clone()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_str() {
        assert_eq!(parse_str(COMMANDS), [3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_iterate_days_short() {
        let input= parse_str(COMMANDS);
        assert_eq!(iterate_days(18, &input).len(), 26);
        assert_eq!(iterate_days(80, &input).len(), 5934);
    }

    #[test]
    fn test_iterate_days_real() {
        let input= parse_str(include_str!("input.txt"));
        assert_eq!(iterate_days(18, &input).len(), 1565);
        assert_eq!(iterate_days(80, &input).len(), 352195);
    }

}
