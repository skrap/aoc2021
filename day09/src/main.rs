use std::collections::{VecDeque, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> i32 {
    let map = parse(input);
    let lows = find_lows(&map);
    lows.into_iter().map(|(x,y)| (map[y][x] - b'0') as i32 +1).sum()
}

fn find_lows(map: &Vec<&[u8]>) -> Vec<(usize, usize)> {
    let mut lows = vec![];
    let xrange = 0..map[0].len();
    let yrange = 0..map.len();
    for (x, y) in xrange.clone().cartesian_product(yrange.clone()) {
        let here = map[y][x];
        if (x == 0 || map[y][x - 1] > here)
            && (x + 1 >= xrange.end || map[y][x + 1] > here)
            && (y == 0 || map[y - 1][x] > here)
            && (y + 1 >= yrange.end || map[y + 1][x] > here)
        {
            lows.push((x,y));
        }
    }
    lows
}

fn parse(input: &str) -> Vec<&[u8]> {
    let mut map = vec![];
    for line in input.trim().lines() {
        let bytes = line.trim().as_bytes();
        map.push(bytes);
    }
    map
}

#[test]
fn test_part1() {
    let input = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678
    ";
    assert_eq!(part1(input), 15);
}

fn neighbors(map: &[&[u8]], pt: &(usize, usize)) -> Vec<(usize,usize)> {
    let mut result = Vec::new();
    if pt.0 > 0 { result.push((pt.0-1,pt.1)); }
    if pt.0+1 < map[0].len() { result.push((pt.0+1,pt.1)); }
    if pt.1 > 0 { result.push((pt.0,pt.1-1)); }
    if pt.1+1 < map.len() { result.push((pt.0,pt.1+1)); }
    result
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let lows = find_lows(&map);
    let mut sizes= Vec::new();
    for low in lows {
        let mut work = VecDeque::new();
        work.push_back(low);
        let mut seen = HashSet::new();
        while let Some(spot) = work.pop_front() {
            seen.insert(spot);
            for neighbor in neighbors(&map, &spot) {
                if !seen.contains(&neighbor) && map[neighbor.1][neighbor.0] < b'9' {
                    work.push_back(neighbor);
                }
            }
        }
        sizes.push(seen.len());
    }
    sizes.sort();
    sizes.into_iter().rev().take(3).product()
}
