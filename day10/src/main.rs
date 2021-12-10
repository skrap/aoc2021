use std::{collections::VecDeque, iter::Peekable};

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| match parse(l.trim().chars().enumerate().peekable()) {
            Result::Corrupted { found, .. } => match found {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unimplemented!(),
            },
            Result::Ok => 0,
            Result::Incomplete(_) => 0,
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut scores: Vec<_> = input
        .lines()
        .map(|l| match parse(l.trim().chars().enumerate().peekable()) {
            Result::Corrupted { .. } => 0,
            Result::Ok => 0,
            Result::Incomplete(chars) => {
                let mut score = 0;
                for c in chars {
                    score *= 5;
                    score += match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unimplemented!(),
                    }
                }
                score
            }
        })
        .filter(|s| *s > 0)
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

enum Result {
    Ok,
    Corrupted {
        #[allow(dead_code)]
        expected: Option<char>,
        found: char,
    },
    Incomplete(Vec<char>),
}

const OPENS: [char; 4] = ['(', '[', '{', '<'];
const CLOSES: [char; 4] = [')', ']', '}', '>'];

fn parse(mut line: Peekable<impl Iterator<Item = (usize, char)>>) -> Result {
    let mut stack = VecDeque::new();
    while let Some((_idx, c)) = line.next() {
        if let Some(c_idx) = OPENS.iter().position(|f| *f == c) {
            // it's an open, so expect the close
            stack.push_front(CLOSES[c_idx]);
        } else {
            // it'd better be the expected close
            let front = stack.pop_front();
            if front != Some(c) {
                return Result::Corrupted {
                    expected: front,
                    found: c,
                };
            }
        }
    }
    if !stack.is_empty() {
        return Result::Incomplete(stack.into());
    }
    return Result::Ok;
}

#[test]
fn test_me() {
    let input = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";
    assert_eq!(part1(input), 26397);
    assert_eq!(part2(input), 288957);
}
