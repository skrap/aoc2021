use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[test]
fn test_me() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    assert_eq!(part1(input), 40);
    assert_eq!(part2(input), 315);
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().bytes().map(|b| (b - b'0') as usize).collect())
        .collect()
}

#[derive(Default, PartialEq, Eq)]
struct Work {
    pt: (usize, usize), // this comes first so that the heap will prioritize progress towards the goal
    risk: usize,
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    build_best_risk(map)
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut bigmap = Vec::new();
    for ytile in 0..5 {
        for line in &map {
            let mut row = Vec::new();
            for xtile in 0..5 {
                for x in line.iter().map(|x| ((x+xtile+ytile-1)%9)+1) {
                    row.push(x);
                }
            }
            bigmap.push(row);
        }
    }
    build_best_risk(bigmap)
}

fn build_best_risk(map: Vec<Vec<usize>>) -> usize {
    let mut work = BinaryHeap::new();
    work.push(Default::default());
    let mut best_risk = Vec::new();
    best_risk.resize(map[0].len()*map.len(), usize::MAX);
    let mut best_total_risk = usize::MAX;
    let endpt = (map[0].len() - 1, map.len() - 1);
    while let Some(Work { pt, risk }) = work.pop() {
        let best = &mut best_risk[pt.0+pt.1*map.len()];
        if risk < *best {
            *best = risk;
        } else {
            continue;
        }
        if risk >= best_total_risk {
            continue;
        }
        if pt == endpt {
            best_total_risk = best_total_risk.min(risk);
            dbg!(best_total_risk);
            continue;
        }
        for pt in neighbors(pt, &map) {
            let risk = risk + map[pt.1][pt.0];
            work.push(Work { pt, risk })
        }
    }
    *best_risk.last().unwrap()
}

fn neighbors(pt: (usize, usize), map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if pt.1 + 1 < map.len() {
        result.push((pt.0, pt.1 + 1));
    }
    if pt.0 + 1 < map[0].len() {
        result.push((pt.0 + 1, pt.1));
    }
    if pt.0 > 0 {
        result.push((pt.0 - 1, pt.1));
    }
    if pt.1 > 0 {
        result.push((pt.0, pt.1 - 1));
    }
    result
}
