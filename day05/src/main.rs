use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Sub},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn new(input: &str) -> Self {
        let (c0, c1) = input.split_once(",").unwrap();
        Self(c0.parse().unwrap(), c1.parse().unwrap())
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct Line {
    ends: [Point; 2],
}

impl Line {
    fn new(input: &str) -> Self {
        let (p0, p1) = input.split_once(" -> ").unwrap();
        Self {
            ends: [Point::new(p0), Point::new(p1)],
        }
    }

    fn is_ortho(&self) -> bool {
        self.ends[0].0 == self.ends[1].0 || self.ends[0].1 == self.ends[1].1
    }
}

struct LineIter {
    next: Point,
    incr: Point,
    terminal: Point,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.terminal {
            None
        } else {
            let next = self.next;
            self.next += self.incr;
            Some(next)
        }
    }
}

impl IntoIterator for &Line {
    type Item = Point;

    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        let incr = self.ends[1] - self.ends[0];
        let incr = Point(
            if incr.0 == 0 { 0 } else { incr.0 / incr.0.abs() },
            if incr.1 == 0 { 0 } else { incr.1 / incr.1.abs() }
        );
        LineIter {
            next: self.ends[0],
            incr: incr,
            terminal: self.ends[1] + incr,
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut board = HashMap::new();
    for line in input.lines() {
        let line = Line::new(line);
        if line.is_ortho() {
            for point in &line {
                *board.entry(point).or_insert(0) += 1;
            }
        }
    }
    board.values().filter(|v| **v > 1).count() as i32
}

fn part2(input: &str) -> i32 {
    let mut board = HashMap::new();
    for line in input.lines() {
        let line = Line::new(line);
        for point in &line {
            *board.entry(point).or_insert(0) += 1;
        }
    }
    board.values().filter(|v| **v > 1).count() as i32
}

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
    println!("Hello, world!");
}
