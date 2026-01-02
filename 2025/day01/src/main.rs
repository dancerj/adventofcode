fn part1(input: &str) -> i32 {
    let mut position = 50;
    let mut count = 0;
    input.lines().for_each(|line| {
        let direction = match line.chars().nth(0).unwrap() {
            'R' => 1,
            'L' => -1,
            _ => panic!(),
        };
        let size: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        position += direction * size;
        while position < 0 {
            position += 100
        }
        while position > 99 {
            position -= 100
        }
        assert!(position >= 0);
        assert!(position <= 100);

        if position == 0 {
            count += 1;
        }
    });
    count
}

fn part2(input: &str) -> u32 {
    let mut position = 50;
    let mut count_latches: u32 = 0;
    input.lines().for_each(|line| {
        let direction = match line.chars().nth(0).unwrap() {
            'R' => 1,
            'L' => -1,
            _ => panic!(),
        };
        let size: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        position += direction * size;
        while position < 0 {
            position += 100;
            count_latches += 1;
            if position != 0 {
                count_latches += 1;
            }
        }
        while position > 99 {
            position -= 100;
            count_latches += 1;
            if position != 0 {
                count_latches += 1;
            }
        }
        assert!(position >= 0);
        assert!(position <= 100);
    });
    count_latches.div_ceil(2)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1_small_sample() {
        let input = SAMPLE_INPUT;
        let result = part1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_real_problem() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 1123);
    }

    #[test]
    fn test_part2_small_sample() {
        let input = SAMPLE_INPUT;
        let result = part2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_artificial_sample() {
        let input = "R50
L10
R10
R50";
        let result = part2(input);
        assert_eq!(result, 2);
    }


    #[test]
    fn test_part2_artificial_sample2() {
        let input = "L250";
        let result = part2(input);
        assert_eq!(result, 2); // TODO
    }

    // #[test]
    fn _test_part2_real_problem() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 6374);
    }
}
