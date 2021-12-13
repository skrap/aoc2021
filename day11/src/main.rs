struct Octopus {
    energy: u8,
    flashed: bool,
}

fn parse(input: &str) -> Vec<Vec<Octopus>> {
    let mut result = Vec::new();
    for line in input.trim().lines() {
        let mut row = Vec::new();
        for ch in line.trim().as_bytes() {
            row.push(Octopus {
                energy: *ch - b'0',
                flashed: false,
            })
        }
        result.push(row);
    }
    result
}

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn neighbors(x: usize, y: usize, map: &[Vec<Octopus>]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 && y > 0 {
        result.push((x - 1, y - 1));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y > 0 && x + 1 < map[y].len() {
        result.push((x + 1, y - 1));
    }
    if x > 0 {
        result.push((x - 1, y));
    }
    if x + 1 < map[y].len() {
        result.push((x + 1, y));
    }
    if x > 0 && y + 1 < map.len() {
        result.push((x - 1, y + 1));
    }
    if y + 1 < map.len() {
        result.push((x, y + 1));
    }
    if y + 1 < map.len() && x + 1 < map[y].len() {
        result.push((x + 1, y + 1));
    }
    result
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);
    let mut flash_cnt = 0;
    for _step in 0..100 {
        flash_cnt += step(&mut map);
    }
    flash_cnt
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);
    let o_cnt = map.iter().flatten().count();
    for step_num in 1.. {
        if step(&mut map) == o_cnt {
            return step_num;
        }
    }
    unimplemented!()
}

fn step(map: &mut Vec<Vec<Octopus>>) -> usize {
    let mut flash_cnt = 0;
    let mut flashes = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[y][x].energy += 1;
            if map[y][x].energy > 9 {
                map[y][x].flashed = true;
                flashes.push((x, y))
            }
        }
    }
    while let Some((x, y)) = flashes.pop() {
        for (x, y) in neighbors(x, y, &*map) {
            let o = &mut map[y][x];
            o.energy += 1;
            if o.energy > 9 && !o.flashed {
                map[y][x].flashed = true;
                flashes.push((x, y));
            }
        }
    }
    for o in map.iter_mut().flatten() {
        if o.flashed {
            flash_cnt += 1;
            o.flashed = false;
            o.energy = 0;
        }
    }
    flash_cnt
}

#[test]
fn test_me() {
    let input = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";
    assert_eq!(part1(input), 1656);
    assert_eq!(part2(input), 195);
}