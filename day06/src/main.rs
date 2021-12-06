fn parse(input: &str) -> Vec<usize> {
    let mut result = vec![0;9];
    for num in input.trim().split(",").flat_map(str::parse::<usize>) {
        result[num] += 1;
    }
    result
}

fn part1(input: &str, days: usize) -> usize {
    let mut fish = parse(input);
    for _day in 0..days {
        let zerofish = fish[0];
        let mut newfish : Vec<usize> = fish.drain(1..).collect();
        newfish.push(zerofish); // idx 8
        newfish[6] += zerofish;
        fish = newfish;
    }
    fish.iter().sum()
}

#[test]
fn test_part1() {
    let input = "3,4,3,1,2";
    assert_eq!(part1(input, 18), 26);
    assert_eq!(part1(input, 80), 5934);
}

fn main() {
    let input = include_str!("input");
    dbg!(part1(input, 80));
    dbg!(part1(input, 256));
}
