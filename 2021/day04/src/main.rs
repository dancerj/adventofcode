#[derive(Debug)]
struct Table {
    number: Vec<Vec<u32>>,
}

fn parse1(s: &str) -> (Vec<u32>, Vec<Table>) {
    // each digit, 0, and 1.
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    let items = first_line.split(',').map(|x| x.parse().unwrap()).collect();

    // next line is empty.
    assert_eq!(lines.next().unwrap(), "");

    let mut table: Vec<Table> = vec![Table{number: vec![]}];
    let mut i = 0;
    for line in lines {
        if line == "" {
            i += 1;
            table.push(Table{number: vec![]});
            continue;
        }
        table[i].number.push(line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect());
    }

    (items, table)
}

fn evaluate(_numbers: &[u32], _tables: &[Table])-> bool {
    true
}

fn part1() {}

fn main() {
    part1();
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() -> Result<(), std::io::Error> {
        let (sequence, boards) = parse1(COMMANDS);
        println!("{:?} {:?}", sequence, boards);
        assert_eq!(
            sequence,
            [
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        // assert_eq!(gamma, 22);
        // assert_eq!(epsilon, 9);
        Ok(())
    }
}
