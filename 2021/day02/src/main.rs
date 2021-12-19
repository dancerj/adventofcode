use std::fs;

fn process1(s: &str) -> (u64, u64) {
    let mut depth = 0;
    let mut horizontal = 0;

    for line in s.lines() {
        let mut v = line.split(' ');
        let command = v.next().unwrap();
        let value = v.next().unwrap().parse::<u64>().unwrap();
        match command {
            "forward" => {
                horizontal += value;
            }
            "down" => {
                depth += value;
            }
            "up" => {
                depth -= value;
            }
            "back" => {
                horizontal -= value;
            }
            _ => {
                panic!("Hello world");
            }
        }
    }
    (depth, horizontal)
}

fn part1() {
    let text = fs::read_to_string("input.txt").unwrap();
    let (depth, horizontal) = process1(&text);
    println!("{} {} {}", depth, horizontal, depth * horizontal);
}

fn process2(s: &str) -> (u64, u64) {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in s.lines() {
        let mut v = line.split(' ');
        let command = v.next().unwrap();
        let value = v.next().unwrap().parse::<u64>().unwrap();
        match command {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => {
                aim += value;
            }
            "up" => {
                aim -= value;
            }
            _ => {
                panic!("Hello world");
            }
        }
    }
    (depth, horizontal)
}

fn part2() {
    let text = fs::read_to_string("input.txt").unwrap();
    let (depth, horizontal) = process2(&text);
    println!("{} {} {}", depth, horizontal, depth * horizontal);
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
        let commands = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let (depth, horizontal) = process1(commands);
        // println!("{:?}", result);
        assert_eq!(depth, 10);
        assert_eq!(horizontal, 15);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), std::io::Error> {
        let commands = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let (depth, horizontal) = process2(commands);
        assert_eq!(depth, 60);
        assert_eq!(horizontal, 15);
        Ok(())
    }
}
