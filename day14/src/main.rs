use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn parse(input: &str) -> (&str, HashMap<(u8, u8), u8>) {
    let mut map = HashMap::new();
    let (start, rules) = input.trim().split_once("\n\n").unwrap();
    for line in rules.lines() {
        let (from, to) = line.trim().split_once(" -> ").unwrap();
        let from = from.as_bytes();
        map.insert((from[0], from[1]), to.as_bytes()[0]);
    }
    (start, map)
}

fn part1(input: &str) -> usize {
    let (start, map) = parse(input);

    let mut s = start.as_bytes().to_owned();
    for _step in 0..10 {
        let last = *s.last().unwrap();
        s = s
            .windows(2)
            .map(|s| vec![s[0], map[&(s[0], s[1])]].into_iter())
            .flatten()
            .collect();
        s.push(last);
        // println!("{}", std::str::from_utf8(&s).unwrap());
        // println!("{}", counts(&s))
    }

    counts(&s)
}

fn counts(s: &[u8]) -> usize {
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for b in s {
        *counts.entry(*b).or_default() += 1;
    }
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part2(input: &str) -> usize {
    let (start, map) = parse(input);
    let start = start.as_bytes().to_owned();

    let mut letters = HashSet::new();
    for rule in &map {
        letters.insert(*rule.1);
        letters.insert(rule.0 .0);
        letters.insert(rule.0 .1);
    }

    let mut counts = HashMap::new();
    for letter in letters {
        let mut cnt = 0usize;
        let mut memo = HashMap::new();
        for pair in start.windows(2) {
            cnt += count((pair[0],pair[1]), letter, &map, 40, &mut memo);
        }
        if start[0] == letter {
            cnt += 1;
        }
        counts.insert(letter, cnt);
    }
    
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn count(pair: (u8, u8), letter: u8, map: &HashMap<(u8, u8), u8>, rounds: usize, memo: &mut HashMap<((u8,u8), u8, usize), usize>) -> usize {
    let mid = map[&pair];
    if rounds == 0 {
        // only count the 2nd one
        return if pair.1 == letter { 1 } else { 0 };
    }
    let params = (pair, letter, rounds);
    if let Some(r) = memo.get(&params) {
        return *r;
    }
    let res = count((pair.0, mid), letter, map, rounds - 1, memo) + count((mid, pair.1), letter, map, rounds - 1, memo);
    memo.insert(params, res);
    res
}

#[test]
fn test_me() {
    let input = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    assert_eq!(part1(input), 1588);
    assert_eq!(part2(input), 2188189693529);
}
