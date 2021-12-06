fn most_common_bits(set_bits: &[i32], rows: usize) -> Vec<u8> {
    set_bits
        .iter()
        .map(|cnt| if cnt * 2 > rows as i32 { 1 } else { 0 })
        .collect()
}

fn part1(input: &str) -> i32 {
    let mut bits = vec![];
    bits.resize(input.lines().next().unwrap().len(), 0);
    for line in input.lines() {
        for (val, b) in line.bytes().zip(bits.iter_mut()) {
            *b += (val - b'0') as i32;
        }
    }
    let num_lines = input.lines().count();
    let most_common = most_common_bits(&bits, num_lines);
    let mut gamma = 0;
    let mut epsilon = 0;
    for (e, b) in most_common.iter().rev().enumerate() {
        if *b != 0u8 {
            gamma += 1 << e;
        } else {
            epsilon += 1 << e;
        }
    }
    gamma * epsilon
}

fn most_common_byte<'a, T>(input: T, nth: usize) -> u8
where
    T: Iterator<Item = &'a [u8]>,
{
    let mut cnts = [0i32; 2];
    for row in input {
        cnts[match row[nth] {
            b'0' => 0,
            b'1' => 1,
            _ => unimplemented!(),
        }] += 1;
    }
    if cnts[0] <= cnts[1] { b'1' } else { b'0' }
}

fn part2(input: &str) -> usize {
    let rows = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let mut o2rows = rows.clone();
    for bitpos in 0.. {
        if o2rows.len() <= 1 {
            break;
        }
        let criteria = most_common_byte(o2rows.iter().cloned(), bitpos);
        o2rows = o2rows
            .drain(..)
            .filter(|row| row[bitpos] == criteria)
            .collect();
    }

    let mut co2rows = rows.clone();
    for bitpos in 0.. {
        if co2rows.len() <= 1 {
            break;
        }
        let criteria = most_common_byte(co2rows.iter().cloned(), bitpos);
        co2rows = co2rows
            .drain(..)
            .filter(|row| row[bitpos] != criteria)
            .collect();
    }

    fn row2num(row: &[u8]) -> usize {
        let mut accum = 0;
        for i in row {
            accum = (accum << 1) | (*i - b'0') as usize;
        }
        accum
    }
    row2num(o2rows[0]) * row2num(co2rows[0])
}

#[test]
fn test_part2() {
    let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;
    assert_eq!(part2(input), 230);
}

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}
