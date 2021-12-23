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
        }).fold(false, |accum, item| accum || item)
}

// Return whether there was a bingo, and returns the score.
fn evaluate(numbers_raw: &[u32], tables: &[Table]) -> (bool, u32) {
    let last_number = numbers_raw[numbers_raw.len() - 1];
    let numbers_set: HashSet<u32> = numbers_raw.iter().cloned().collect();
    // Initialize the matched table
    match tables
        .iter()
        .map(|table| {
            let matched: Vec<Vec<bool>> = table
                .number
                .iter()
                .map(|x| x.iter().map(|x| numbers_set.contains(x)).collect())
                .collect();
            //debug print
            table.number.iter().for_each(|x| {
                x.iter().for_each(|x| {
                    if !numbers_set.contains(x) {
                        print!(" {:>2}  ", x)
                    } else {
                        print!("[{:>2}] ", x)
                    }
                });
                println!("");
            });

            if is_bingo(&matched) {
                let my_score: u32 = table
                    .number
                    .iter()
                    .map(|x| {
                        x.iter()
                            .map(|&x| if numbers_set.contains(&x) { 0 } else { x })
                            .sum::<u32>()
                    })
                    .sum();
                (true, Some(my_score * last_number))
            } else {
                (false, None)
            }
        })
        .find(|x| x.0)
    {
        Some((true, Some(score))) => (true, score),
        Some(_) => panic!("Can't be"),
        None => (false, 0),
    }
}

// Returns which board won, with what score.
fn how_many_bingos(
    numbers_raw: &[u32],
    tables: &[Table],
) -> Vec<(usize /* board number */, u32 /* score */)> {
    let last_number = numbers_raw[numbers_raw.len() - 1];
    let numbers_set: HashSet<u32> = numbers_raw.iter().cloned().collect();
    // Initialize the matched table
    tables
        .iter()
        .enumerate()
        .filter_map(|(serial_number, table)| {
            let matched: Vec<Vec<bool>> = table
                .number
                .iter()
                .map(|x| x.iter().map(|x| numbers_set.contains(x)).collect())
                .collect();

            if is_bingo(&matched) {
                let my_score: u32 = table
                    .number
                    .iter()
                    .map(|x| {
                        x.iter()
                            .map(|&x| if numbers_set.contains(&x) { 0 } else { x })
                            .sum::<u32>()
                    })
                    .sum();
                Some((serial_number, my_score * last_number))
            } else {
                None
            }
        })
        .collect()
}

fn main() {}

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
            false
        );

        assert_eq!(
            is_bingo(&vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, false, false]
            ]),
            false
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

        assert_eq!(evaluate(&sequence[0..5], &boards).0, false);
        assert_eq!(evaluate(&sequence[0..11], &boards).0, false);
        assert_eq!(&sequence[0..12], [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]);
        let r = evaluate(&sequence[0..12], &boards);
        assert_eq!(r.0, true);
        assert_eq!(r.1, 4512);

        // assert_eq!(gamma, 22);
        // assert_eq!(epsilon, 9);
        Ok(())
    }

    #[test]
    fn test_part1_real() {
        let (sequence, boards) = parse1(include_str!("input.txt"));
        let n = (1..sequence.len())
            .find(|&length| {
                let r = evaluate(&sequence[0..length], &boards);
                r.0
            })
            .unwrap();
        assert_eq!(n, 31);
        let r = evaluate(&sequence[0..31], &boards);
        assert_eq!(r.1, 50008);
    }

    #[test]
    fn test_part2() {
        let (sequence, boards) = parse1(COMMANDS);

        assert_eq!(how_many_bingos(&sequence[0..5], &boards).len(), 0);
        assert_eq!(how_many_bingos(&sequence[0..12], &boards).len(), 1);
        assert_eq!(how_many_bingos(&sequence[0..15], &boards).len(), 3);
    }

    #[test]
    fn test_part2_real() {
        let (sequence, boards) = parse1(include_str!("input.txt"));
        assert_eq!(boards.len(), 100);
        // TODO I could do binary search
        let n = (1..sequence.len())
            .find(|&length| {
                let r = how_many_bingos(&sequence[0..length], &boards);
                r.len() == 100
            })
            .unwrap();
        assert_eq!(n, 84);
        let before = how_many_bingos(&sequence[0..83], &boards);
        let after = how_many_bingos(&sequence[0..84], &boards);
        let mut bingo_boards = vec![true; 100];
        before.iter().for_each(|&(board_number, _score)| {
            bingo_boards[board_number] = false;
        });
        let final_score = after.iter().enumerate().find(|&(board_number, _score)| {
            if bingo_boards[board_number] {
                println!("board! {}", board_number);
                true
            } else {
                false
            }
        });
        assert_eq!(final_score.unwrap().1 .1, 17408);
    }
}
