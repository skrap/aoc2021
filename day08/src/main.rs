use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        // be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        for output in line.split_once(" | ").unwrap().1.trim().split_whitespace() {
            match output.len() {
                2 | 4 | 3 | 7 => total += 1,
                _ => (),
            }
        }
    }
    total
}

fn part2(input: &str) -> usize {
    input.lines().map(solve_line).sum()
}

fn solve_line(line: &str) -> usize {   
    let ssds = [
        vec![0, 1, 2, 4, 5, 6],    // 0
        vec![2, 5],                // 1
        vec![0, 2, 3, 4, 6],       // 2
        vec![0, 2, 3, 5, 6],       // 3
        vec![1, 2, 3, 5],          // 4
        vec![0, 1, 3, 5, 6],       // 5
        vec![0, 1, 3, 4, 5, 6],    // 6
        vec![0, 2, 5],             // 7
        vec![0, 1, 2, 3, 4, 5, 6], // 8
        vec![0, 1, 2, 3, 5, 6],    // 9
    ];
 
    let (clues, output) = line.split_once(" | ").unwrap();
    let mut clues: Vec<_> = clues.trim().split_whitespace().collect();
    clues.sort_by_key(|c| c.len());
    let output: Vec<_> = output.trim().split_whitespace().collect();

    let txs = (0..7).permutations(7);
    'txs: for tx in txs {
        let mut key = HashMap::new();
        for clue in clues.iter().cloned() {
            let mut txd: Vec<u32> = clue
                .chars()
                .map(|c| {
                    let i = c as u32 - 'a' as u32;
                    tx[i as usize]
                })
                .collect();
            txd.sort();
            if let Some((digit, _ssd)) = ssds.iter().find_position(|ssd| *ssd == &txd) {
                let mut word = clue.chars().collect::<Vec<char>>();
                word.sort();
                key.insert(word, digit);
            } else {
                continue 'txs;
            }
        }
        // we have a working solution!
        let mut result = 0;
        for outword in output {
            let mut word = outword.chars().collect::<Vec<_>>();
            word.sort();
            result = result*10 + key[&word];
        }
        return result;
    }
    unimplemented!()
}
