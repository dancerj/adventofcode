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

fn find_oxy(lines: &[&str], pos: usize) -> String {
    // Find the most popular / least popular bit value.
    let count = lines
        .iter()
        .map(|x| match x.chars().nth(pos) {
            Some('0') => (1, 0),
            Some('1') => (0, 1),
            _ => {
                panic!("hello");
            }
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    match count {
        (a, 1) if a <= 1 => lines
            .iter()
            .filter(|x| x.chars().nth(pos) == Some('1'))
            .next()
            .unwrap()
            .to_string(),
        (zero, one) if zero <= one => {
            // Keep one.
            find_oxy(
                &(lines
                    .iter()
                    .filter(|x| x.chars().nth(pos) == Some('1'))
                    .map(|x| *x)
                    .collect::<Vec<&str>>()),
                pos + 1,
            )
        }
        // The rest
        (_, _) => {
            // Keep zero
            find_oxy(
                &lines
                    .iter()
                    .filter(|x| x.chars().nth(pos) == Some('0'))
                    .map(|x| *x)
                    .collect::<Vec<&str>>(),
                pos + 1,
            )
        }
    }
}

fn find_co2(lines: &[&str], pos: usize) -> String {
    let count = lines
        .iter()
        .map(|x| match x.chars().nth(pos) {
            Some('0') => (1, 0),
            Some('1') => (0, 1),
            _ => {
                panic!("hello");
            }
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    match count {
        (1, b) if b <= 1 => lines
            .iter()
            .filter(|x| x.chars().nth(pos) == Some('0'))
            .next()
            .unwrap()
            .to_string(),
        (zero, one) if zero <= one => {
            // Keep zero
            find_co2(
                &(lines
                    .iter()
                    .filter(|x| x.chars().nth(pos) == Some('0'))
                    .map(|x| *x)
                    .collect::<Vec<&str>>()),
                pos + 1,
            )
        }
        // The rest
        (_, _) => {
            // Keep one
            find_co2(
                &lines
                    .iter()
                    .filter(|x| x.chars().nth(pos) == Some('1'))
                    .map(|x| *x)
                    .collect::<Vec<&str>>(),
                pos + 1,
            )
        }
    }
}

fn process2(s: &str) -> (u64, u64) {
    let lines = &s.lines().collect::<Vec<&str>>();
    (
        u64::from_str_radix(&find_oxy(lines, 0), 2).unwrap(),
        u64::from_str_radix(&find_co2(lines, 0), 2).unwrap(),
    )
}

fn part1() {
    let text = fs::read_to_string("input.txt").unwrap();
    let (gamma, epsilon) = process1(&text).unwrap();
    println!("{} {} {}", gamma, epsilon, gamma * epsilon);
}

fn part2() {
    let text = fs::read_to_string("input.txt").unwrap();
    let (oxygen, co2scrubber) = process2(&text);
    println!("{} {} {}", oxygen, co2scrubber, oxygen * co2scrubber);
}

fn main() {
    part1();
    part2();
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

    #[test]
    fn test_part2() -> Result<(), std::io::Error> {
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
        let (oxygen, co2scrubber) = process2(commands);
        println!("{} {}", oxygen, co2scrubber);
        assert_eq!(oxygen, 23);
        assert_eq!(co2scrubber, 10);
        Ok(())
    }
}
