use std::fmt::Debug;
use std::str::FromStr;

fn parse_str<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    s.split(",").map(|x| x.trim().parse().unwrap()).collect()
}

fn main() {}

fn get_fuel(positions: &[i32], goal: i32) -> i32 {
    positions.iter().map(|x| (x - goal).abs()).sum()
}

fn get_fuel2(positions: &[i32], goal: i32) -> i32 {
    positions.iter().map(|x| (1..=(x - goal).abs()).sum::<i32>()).sum()
}

fn search_min(positions: &[i32]) -> (i32, i32) {
    let mut position = positions.iter().sum::<i32>() / positions.len() as i32;
    loop {
        let last = get_fuel(positions, position);
        if get_fuel(positions, position - 1) < last {
            position -= 1;
        } else if get_fuel(positions, position + 1) < last {
            position += 1;
        } else {
            return (position, last);
        }
    }
}

fn search_min2(positions: &[i32]) -> (i32, i32) {
    let mut position = positions.iter().sum::<i32>() / positions.len() as i32;
    loop {
        let last = get_fuel2(positions, position);
        if get_fuel2(positions, position - 1) < last {
            position -= 1;
        } else if get_fuel2(positions, position + 1) < last {
            position += 1;
        } else {
            return (position, last);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse_str() {
        assert_eq!(parse_str::<i32>(COMMANDS), [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_get_distance() {
        let commands = parse_str::<i32>(COMMANDS);
        let results: Vec<i32> = (0..10).map(|goal| get_fuel(&commands, goal)).collect();
        println!("{:?}", results);
        println!("{:?}", commands.iter().sum::<i32>() / commands.len() as i32);
        assert_eq!(get_fuel(&commands, 2), 37);
    }

    #[test]
    fn test_search_min() {
        let commands = parse_str::<i32>(COMMANDS);
        assert_eq!(search_min(&commands).0, 2);
    }

    #[test]
    fn test_search_min_real() {
        let commands = parse_str::<i32>(include_str!("input.txt"));
        assert_eq!(search_min(&commands).0, 313);
        assert_eq!(search_min(&commands).1, 335271);
    }

    #[test]
    fn test_search_min2() {
        let commands = parse_str::<i32>(COMMANDS);
        assert_eq!(search_min2(&commands), (5,168));
    }

    #[test]
    fn test_search_min2_real() {
        let commands = parse_str::<i32>(include_str!("input.txt"));
        assert_eq!(search_min2(&commands), (461, 95851339));
    }
}
