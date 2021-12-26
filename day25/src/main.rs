fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
}

fn part1(input: &str) -> usize {
    let mut map: Vec<u8> = input.trim().lines().map(|l| l.as_bytes()).flatten().cloned().collect();
    let width = input.lines().next().unwrap().len();
    let height = map.len()/width;

    for step in 1.. {
        let mut num_moves = 0;

        let mut newmap = map.clone();

        // eastward
        for y in 0..height {
            let mut x = 0;
            while x < width {
                let target = y*width+x;
                let target_1 = y*width+((x+1)%width);
                if map[target] == b'>' && map[target_1] == b'.' {
                    newmap[target] = b'.';
                    newmap[target_1] = b'>';
                    num_moves += 1;
                    x += 2;
                } else {
                    x += 1;
                }
            }
        }

        map = newmap.clone();

        // southward
        for x in 0..width {
            let mut y = 0;
            while y < height {
                let target = y*width+x;
                let target_1 = ((y+1)%height)*width+x;
                if map[target] == b'v' && map[target_1] == b'.' {
                    newmap[target] = b'.';
                    newmap[target_1] = b'v';
                    num_moves += 1;
                    y += 2;
                } else {
                    y += 1;
                }
            }
        }

        map = newmap;

        if false {
            println!("After {} steps", step);
            for y in 0..height {
                println!("{}", std::str::from_utf8(&map[y*width..(y+1)*width]).unwrap())
            }
        }
        
        if num_moves == 0 {
            return step;
        }
    }

    unreachable!();
}


#[test]
fn test_part1() {
    let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    assert_eq!(part1(input), 58);
}