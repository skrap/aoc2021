enum Comm {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse(input: &str) -> Vec<Comm> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (cmd, amt) = line.split_once(' ').unwrap();
        let amt = amt.parse().unwrap();
        result.push(match cmd {
            "forward" => Comm::Forward(amt),
            "down" => Comm::Down(amt),
            "up" => Comm::Up(amt),
            _ => unimplemented!()
        });
    }
    result
}

fn part1(input: &str) -> i32 {
    let comms = parse(input);
    let mut pos = 0;
    let mut depth = 0;
    for comm in comms {
        match comm {
            Comm::Forward(x) => pos += x,
            Comm::Down(x) => depth += x,
            Comm::Up(x) => depth -= x,
        }
    }
    pos * depth
}

fn part2(input: &str) -> i32 {
    let comms = parse(input);
    let mut pos = 0;
    let mut aim = 0;
    let mut depth = 0;
    for comm in comms {
        match comm {
            Comm::Down(x) => aim += x,
            Comm::Up(x) => aim -= x,
            Comm::Forward(x) => {
                pos += x;
                depth += x*aim;
            }
        }
    }
    pos * depth
}

fn main() {
    dbg!(part1(include_str!("input")));
    dbg!(part2(include_str!("input")));
}
