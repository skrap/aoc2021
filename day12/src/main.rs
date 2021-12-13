use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("input");
    dbg!(run(input, 0));
    dbg!(run(input, 1));
}

struct Work<'a> {
    at: &'a str,
    past: Vec<&'a str>,
    repeats: u32,
}

fn run(input: &str, max_repeat_small: u32) -> usize {
    let map = parse(input);
    let mut routes = 0;
    let mut work = VecDeque::new();
    work.push_back(Work {
        at: "start",
        past: vec!["start"],
        repeats: 0,
    });
    while let Some(Work { at, past, repeats }) = work.pop_front() {
        if at == "end" {
            // we're at the end
            routes += 1;
        } else {
            for next in &map[at] {
                let mut repeats = repeats;
                if next.chars().all(char::is_lowercase) {
                    // it's LC, so only go here if we've not visited before
                    if past.contains(&next) {
                        if repeats < max_repeat_small && *next != "start" {
                            repeats += 1;
                        } else {
                            continue;
                        }
                    }
                }
                let mut past = past.clone();
                past.push(next);
                work.push_back(Work {
                    at: next,
                    past,
                    repeats,
                });
            }
        }
    }

    routes
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let (from, to) = line.trim().split_once("-").unwrap();
        map.entry(from).or_insert(Vec::new()).push(to);
        map.entry(to).or_insert(Vec::new()).push(from);
    }
    map
}

#[test]
fn test_me() {
    let input = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
    assert_eq!(run(input, 1), 36);
}