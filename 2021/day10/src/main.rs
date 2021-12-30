//use std::collections::BTreeSet;
use std::collections::HashMap;

fn find_invalid(s: &str) -> u32 {
    let k_scores: HashMap<char, u32> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .iter()
        .cloned()
        .collect();

    let k_match: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();

    let mut stack: Vec<char> = vec![];
    let mut score = 0;
    s.lines().for_each(|line| {
        line.chars().for_each(|c| {
            if k_match.contains_key(&c) {
                stack.push(c);
            } else {
                let top = stack.pop().unwrap();
                if k_match.get(&top).unwrap() != &c {
                    // invalid closing brace
                    println!("invalid closing '{}' for '{}'", c, top);
                    score += k_scores.get(&c).unwrap();
                } else {
                    // matched.
                }
            }
        })
    });
    score
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMANDS: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_find_invalid() {
        let i = find_invalid(COMMANDS);
        assert_eq!(i, 26397);
    }

    #[test]
    fn test_find_invalid_real() {
        let i = find_invalid(include_str!("input.txt"));
        assert_eq!(i, 415953);
    }
}
