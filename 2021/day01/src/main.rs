use std::fs::File;
use std::io::{self, BufRead};

fn main() {
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
