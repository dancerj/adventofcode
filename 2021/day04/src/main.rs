use std::collections::HashSet;

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

    let mut table: Vec<Table> = vec![Table { number: vec![] }];
    let mut i = 0;
    for line in lines {
        if line == "" {
            i += 1;
            table.push(Table { number: vec![] });
            continue;
        }
        table[i].number.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }

    (items, table)
}

fn is_bingo(matches: &Vec<Vec<bool>>) -> bool {
    let size = matches.len();

    // TODO use reduce instead of fold.

    // horizontal
    matches.iter().map(|x| {
        x.iter().fold(true, |accum, item| accum && *item)
    }).fold(false, |accum, item| accum || item) ||
    // vertical
        (0..size).map(|i| {
            matches.iter().map(|x| {
                x[i]
            }).fold(true, |accum, item| accum && item)
        }).fold(false, |accum, item| accum || item) ||
    // diagonal
        (0..size).map(|i| {
            matches[i][i]
        }).fold(true, |accum, item| accum && item) ||
    // diagonal on the other direction.
        (0..size).map(|i| {
            matches[i][size-1-i]
        }).fold(true, |accum, item| accum && item)
}

fn score(table: &Table, matches: &Vec<Vec<bool>>) -> u32 {
    (0..table.number.len())
        .map(|i| {
            (0..table.number.len())
                .map(|j| if matches[i][j] { table.number[i][j] } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn evaluate(numbers_raw: &[u32], tables: &[Table]) -> bool {
    let numbers_set: HashSet<u32> = numbers_raw.iter().cloned().collect();
    // Initialize the matched table
    let _matched_tables = tables.iter().map(|table| {
        let matched: Vec<Vec<bool>> = table
            .number
            .iter()
            .map(|x| x.iter().map(|x| numbers_set.contains(x)).collect())
            .collect();
        print!("{:?}", matched);
        let bingo = is_bingo(&matched);
        if bingo {
            (bingo, Some(score(table, &matched)))
        } else {
            (false, None)
        }
    });
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
    fn test_is_bingo() {
        assert_eq!(
            is_bingo(&vec![
                vec![false, false, false],
                vec![false, true, false],
                vec![false, false, false]
            ]),
            false
        );

        assert_eq!(
            is_bingo(&vec![
                vec![true, false, false],
                vec![false, true, false],
                vec![false, false, true]
            ]),
            true
        );

        assert_eq!(
            is_bingo(&vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, false, false]
            ]),
            true
        );

        assert_eq!(
            is_bingo(&vec![
                vec![true, false, false],
                vec![true, true, false],
                vec![true, false, false]
            ]),
            true
        );

        assert_eq!(
            is_bingo(&vec![
                vec![true, false, false],
                vec![true, true, true],
                vec![false, false, false]
            ]),
            true
        );
    }

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

        evaluate(&sequence, &boards);

        // assert_eq!(gamma, 22);
        // assert_eq!(epsilon, 9);
        Ok(())
    }
}
