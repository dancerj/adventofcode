use std::fs;
use std::num::ParseIntError;

fn process1(s: &str) -> Result<(u64, u64), ParseIntError> {
    // each digit, 0, and 1.
    let first = s.lines().next().unwrap();
    let len_of_bits = first.len();
    let mut count = vec![(0, 0); len_of_bits];

    for line in s.lines() {
        for pos in 0..len_of_bits {
            match line.chars().nth(pos) {
                Some('0') => {
                    count[pos].0 += 1;
                }
                Some('1') => {
                    count[pos].1 += 1;
                }
                _ => {
                    panic!("hello");
                }
            }
        }
    }
    let gamma = u64::from_str_radix(
        &count
            .iter()
            .map(|p| if p.0 > p.1 { '0' } else { '1' })
            .collect::<String>(),
        2,
    )?;
    let epsilon = u64::from_str_radix(
        &count
            .iter()
            .map(|p| if p.0 > p.1 { "1" } else { "0" })
            .collect::<String>(),
        2,
    )?;
    // println!("{:?} {:?} ", gamma, epsilon);
    Ok((gamma, epsilon))
}

fn part1() {
    let text = fs::read_to_string("input.txt").unwrap();
    let (gamma, epsilon) = process1(&text).unwrap();
    println!("{} {} {}", gamma, epsilon, gamma * epsilon);
}

fn main() {
    part1();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), std::io::Error> {
        let commands = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let (gamma, epsilon) = process1(commands).unwrap();
        println!("{} {}", gamma, epsilon);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        Ok(())
    }
}
