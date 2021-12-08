fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn part1(input: &str) -> i32 {
    let mut nums = parse(input);
    nums.sort();
    let targets = nums[0]..*nums.last().unwrap();
    let min = targets
        .map(|target| {
            let dist: i32 = nums.iter().map(|s| (s - target).abs()).sum();
            (dist, target)
        })
        .min();
    min.unwrap().0
}

fn part2(input: &str) -> i32 {
    let mut nums = parse(input);
    nums.sort();
    let targets = nums[0]..*nums.last().unwrap();
    let min = targets
        .map(|target| {
            let dist: i32 = nums.iter().map(|s| {
                let n = (s - target).abs();
                n*(n+1)/2
            }).sum();
            (dist, target)
        })
        .min();
    min.unwrap().0
}

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}
