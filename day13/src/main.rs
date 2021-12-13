use std::collections::{HashSet};

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

#[derive(Clone, Copy)]
enum Fold {
    Left(i32),
    Up(i32),
}

fn part1(input: &str) -> usize {
    let (mut map, folds) = parse(input);
    step(folds[0], &mut map);
    map.len()
}

fn part2(input: &str) {
    let (mut map, folds) = parse(input);
    for fold in folds {
        step(fold, &mut map);
    }
    let max_x = map.iter().map(|s| s.0).max().unwrap();
    let max_y = map.iter().map(|s| s.1).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if map.contains(&(x,y)) { '#' } else { ' ' });
        }
        println!();
    }
}

fn parse(input: &str) -> (HashSet<(i32, i32)>, Vec<Fold>) {
    let (coords, folds) = input.trim().split_once("\n\n").unwrap();
    let mut map = HashSet::new();
    for line in coords.trim().lines() {
        let (x,y) = line.split_once(",").unwrap();
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        map.insert((x,y));
    }
    let folds : Vec<_> = folds.lines().map(|f| {
        let (instr,coord) = f.split_once("=").unwrap();
        match instr {
            "fold along x" => Fold::Left(coord.parse().unwrap()),
            "fold along y" => Fold::Up(coord.parse().unwrap()),
            _ => unimplemented!()
        }
    }).collect();
    (map, folds)
}

fn step(fold: Fold, map: &mut HashSet<(i32, i32)>) {
    *map = map.iter().cloned().filter_map(|(x,y)| {
        match fold {
            Fold::Left(fold_x) => {
                let x = match x.cmp(&fold_x) {
                    std::cmp::Ordering::Less => Some(x),
                    std::cmp::Ordering::Equal => unimplemented!(),
                    std::cmp::Ordering::Greater => Some(fold_x-(x-fold_x)),
                };
                x.map(|x| (x,y))
            }
            Fold::Up(fold_y) => {
                let y = match y.cmp(&fold_y) {
                    std::cmp::Ordering::Less => Some(y),
                    std::cmp::Ordering::Equal => unimplemented!(),
                    std::cmp::Ordering::Greater => Some(fold_y-(y-fold_y)),
                };
                y.map(|y| (x,y))
            }
        }
    }).collect();
}

#[test]
fn test_me() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    assert_eq!(part1(input), 17);
}