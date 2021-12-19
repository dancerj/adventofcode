use std::fs::File;
use std::io::{self, BufRead};

fn part1() {
    let file = File::open("input.txt").unwrap();
    let mut prev = None;
    let mut count = 0;
    let mut increased_count = 0;
    for line in io::BufReader::new(file).lines() {
        let num = line.unwrap().parse::<u64>().unwrap();
        match prev {
            None => {
                // ignore
            }
            Some(prev_num) => {
                if num > prev_num {
                    // increased
                    increased_count += 1;
                }
            }
        }
        prev = Some(num);
        count += 1;
    }
    println!("{}", count);
    println!("{}", increased_count);
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let input_lines: Vec<u64> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<u64>().unwrap())
        .collect();
    let mut prev = None;
    let mut count = 0;
    let mut increased_count = 0;
    for i in 2..input_lines.len() {
        let avg = (input_lines[i] + input_lines[i - 1] + input_lines[i - 2]) as f64 / 3f64;
        match prev {
            None => {
                // ignore
            }
            Some(prev_num) => {
                if avg > prev_num {
                    // increased
                    increased_count += 1;
                }
            }
        }
        prev = Some(avg);
        count += 1;
    }
    println!("{}", count);
    println!("{}", increased_count);
}

fn main() {
    part1();
    part2();
}
