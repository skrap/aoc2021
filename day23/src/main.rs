use std::collections::{HashMap, VecDeque, BinaryHeap};

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
}

struct Map {
    data: Vec<u8>,
    width: usize,
}

impl Map {
    fn is_space(&self, x: u8, y: u8) -> bool {
        let x: usize = x.into();
        let y: usize = y.into();
        self.data[y * self.width + x] != b'#'
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Anthropod {
    pos: [i8; 2],
    kind: u8,
}

enum Location {
    Hallway,
    HomeRoom,
    OtherRoom,
}

impl Anthropod {
    fn loc(&self) -> Location {
        use Location::*;
        match self.pos {
            [x, 1] if LEGAL_HALLWAY_X.contains(&x) => Hallway,
            [3, _] => {
                if self.kind == b'A' {
                    HomeRoom
                } else {
                    OtherRoom
                }
            }
            [5, _] => {
                if self.kind == b'B' {
                    HomeRoom
                } else {
                    OtherRoom
                }
            }
            [7, _] => {
                if self.kind == b'C' {
                    HomeRoom
                } else {
                    OtherRoom
                }
            }
            [9, _] => {
                if self.kind == b'D' {
                    HomeRoom
                } else {
                    OtherRoom
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    apods: [Anthropod; 8],
}

const HALLWAY: i8 = 1;
const LEGAL_HALLWAY_X: [i8; 7] = [1, 2, 4, 6, 8, 10, 11];

impl State {
    fn is_winning(&self) -> bool {
        self.apods
            .iter()
            .all(|apod| matches!(apod.loc(), Location::HomeRoom))
    }

    fn reachable_hallways(&self, i: usize, map: &Map) -> Vec<[i8; 2]> {
        let mut pos = self.apods[i].pos;
        while pos[1] > 1 {
            pos[1] -= 1;
            if !self.is_open(pos) {
                return vec![];
            }
        }
        let mut result = vec![];
        let mut posleft = pos;
        let mut posright = pos;
        drop(pos);

        while posleft[0] > 1 {
            posleft[0] -= 1;
            if self.is_open(posleft) {
                if LEGAL_HALLWAY_X.contains(&posleft[0]) {
                    result.push(posleft);
                }
            } else {
                break;
            }
        }

        while (posright[0] as usize) < map.width - 1 {
            posright[0] += 1;
            if self.is_open(posright) {
                if LEGAL_HALLWAY_X.contains(&posright[0]) {
                    result.push(posright);
                }
            } else {
                break;
            }
        }
        result
    }

    fn reachable_homerooms(&self, i: usize) -> Vec<[i8; 2]> {
        let targetx = match self.apods[i].kind {
            b'A' => 3,
            b'B' => 5,
            b'C' => 7,
            b'D' => 9,
            _ => unimplemented!(),
        };
        let mut pos = self.apods[i].pos;
        if pos[0] != targetx {
            // not in homeroom, ascend to hallway
            while pos[1] != HALLWAY {
                pos[1] -= 1;
                if !self.is_open(pos) {
                    return vec![];
                }
            }
            // follow hallway
            while pos[0] != targetx {
                if targetx > pos[0] {
                    pos[0] += 1;
                } else {
                    pos[0] -= 1
                }
                if !self.is_open(pos) {
                    return vec![];
                }
            }
        }
        if pos[1] == 1 {
            let nextpos = match self.at([pos[0],3]) {
                Some(k) => if k == self.apods[i].kind { [pos[0],2] } else { return vec![]; }
                None => if self.is_open([pos[0],2]) { [pos[0],3] } else { return vec![]; }
            };
            return vec![nextpos];
        } else if pos[1] == 2 {
            pos[1] = 3;
            if self.is_open(pos) {
                return vec![pos];
            }
        }
        vec![]
    }

    fn is_open(&self, pos: [i8; 2]) -> bool {
        self.at(pos).is_none()
    }

    fn at(&self, pos: [i8; 2]) -> Option<u8> {
        self.apods.iter().find(|a| a.pos == pos).map(|x| x.kind)
    }

    fn apod_actions(&self, i: usize, map: &Map) -> Vec<[i8; 2]> {
        let apod = &self.apods[i];

        match apod.loc() {
            Location::HomeRoom => {
                let mut r = self.reachable_homerooms(i);
                if apod.pos[1] == 2 {
                    r.append(&mut self.reachable_hallways(i, map));
                }
                r
            }
            Location::OtherRoom => self.reachable_hallways(i, map),
            Location::Hallway => self.reachable_homerooms(i),
        }
    }

    fn actions(&self, energy: usize, map: &Map) -> Vec<(State, usize)> {
        (0..self.apods.len())
            .map(|i| {
                // for each apod, collect its legal moves
                self.apod_actions(i, map).into_iter().map(move |pos| {
                    let mut state = self.clone();
                    let old_pos = state.apods[i].pos;
                    let spaces = (old_pos[0] - pos[0]).abs() + (old_pos[1] - pos[1]).abs();
                    let energy = energy
                        + spaces as usize
                            * match state.apods[i].kind {
                                b'A' => 1,
                                b'B' => 10,
                                b'C' => 100,
                                b'D' => 1000,
                                _ => unimplemented!(),
                            };
                    state.apods[i].pos = pos;
                    for i in 0..state.apods.len() {
                        state.apods[i].loc();
                    }
                    (state, energy)
                })
            })
            .flatten()
            .collect()
    }
}

fn parse(input: &str) -> (State, Map) {
    let input = input.trim();
    let width = input.lines().next().unwrap().trim().len();
    let map: Vec<u8> = input
        .lines()
        .map(|s| {
            let mut s = s.as_bytes().to_vec();
            s.resize(width, b' ');
            s.into_iter()
        })
        .flatten()
        .collect();
    let apods: Vec<Anthropod> = map
        .iter()
        .enumerate()
        .filter_map(|(i, ch)| {
            if (b'A'..=b'D').contains(&ch) {
                Some(Anthropod {
                    pos: [(i % width) as i8, (i / width) as i8],
                    kind: *ch,
                })
            } else {
                None
            }
        })
        .collect();
    (
        State {
            apods: apods.try_into().unwrap(),
        },
        Map { data: map, width },
    )
}

fn part1(input: &str) -> usize {
    let (state, map) = parse(input);
    let mut best_energy: HashMap<State, usize> = HashMap::new();

    let mut work = BinaryHeap::new();
    work.push((0usize,state));
    let mut best_win = usize::MAX;

    while let Some((energy, state)) = work.pop() {
        for (next, energy) in state.actions(energy, &map) {
            let prev_best = best_energy.entry(next).or_insert(usize::MAX);
            if *prev_best > energy {
                *prev_best = energy;
                if next.is_winning() {
                    best_win = best_win.min(energy);
                }
                if energy < best_win {
                    work.push((energy, next));
                }
            }
        }
    }

    best_win
}

#[test]
fn test_part1() {
    let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";
    assert_eq!(part1(input), 12521);
}

#[test]
fn test_1step() {
    let start = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    let (start, map) = parse(start);
    let mut step1 = start.clone();
    step1.apods[2].pos = [4,1];
    let nexts = start.actions(0, &map);
    assert!(nexts.contains(&(step1, 40)));
}