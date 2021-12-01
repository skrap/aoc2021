fn part1() {
    let input = include_str!("input");
    let nums: Vec<_> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let count = nums.windows(2).filter(|f| f[0] < f[1]).count();
    dbg!(count);
}

fn part2() {
    let input = include_str!("input");
    let nums: Vec<_> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    let windowed_sums : Vec<i32> = nums.windows(3)
        .map(|f| f.iter().sum()).collect();
    let count = windowed_sums.windows(2).filter(|pair| pair[0] < pair[1]).count();
    dbg!(count);
}

fn main() {
    part1();
    part2();
}
