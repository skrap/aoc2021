fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn to_reg(input: &str) -> Option<usize> {
    match input {
        "w" => Some(0),
        "x" => Some(1),
        "y" => Some(2),
        "z" => Some(3),
        _ => None,
    }
}

fn to_val(input: &str, reg: &[i64; 4]) -> i64 {
    if let Some(idx) = to_reg(input) {
        reg[idx].clone()
    } else {
        input.parse().unwrap()
    }
}

fn report(mut z: i64) {
    print!("  ");
    while z > 0 {
        print!("{} ", z%26);
        z /= 26;
    }
}

fn part1(input: &str) -> i64 {
    let mut nums = vec![9;7];
    loop {
        // let total = nums.iter().fold(0, |acc, ele| acc*10+ele);
        //println!("try: {}", total);
        match run1(input, nums.iter().cloned()) {
            Ok(result) => break result,
            Err(_) => {
                let mut idx = nums.len()-1;
                while nums[idx] == 1 {
                    nums[idx] = 9;
                    idx -= 1;
                }
                nums[idx] -= 1;
            }
        }
    }
}

fn part2(input: &str) -> i64 {
    let mut nums = vec![1;7];
    loop {
        match run1(input, nums.iter().cloned()) {
            Ok(result) => break result,
            Err(_) => {
                let mut idx = nums.len()-1;
                while nums[idx] == 9 {
                    nums[idx] = 1;
                    idx -= 1;
                }
                nums[idx] += 1;
            }
        }
    }
}

fn run1(input: &str, mut nums: impl Iterator<Item=i64>) -> Result<i64,usize> {
    let mut reg = [0; 4];
    let mut output = 0;

    for (chunk_idx, chunk) in input.split_inclusive("add z y").enumerate() {
        // println!(" chunk");
        for line in chunk.trim().lines() {
            let (opname, args) = line.split_once(" ").unwrap();
            let (arg0, arg1) = args.split_once(" ").unwrap_or((args, ""));
            let target = to_reg(arg0).unwrap();
            let arg0 = || to_val(arg0, &reg);
            let arg1 = || to_val(arg1, &reg);

            reg[target] = match opname {
                "inp" => {
                    // report(reg[3]);
                    let pat = "add x ";
                    let loc = chunk.rfind(pat).unwrap();
                    let m = chunk[loc + pat.len()..]
                        .lines()
                        .next()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();
                    let val = reg[3]%26 + m;
                    let num = if (1..=9).contains(&val) {
                        val
                    } else {
                        if let Some(val) = nums.next() {
                            val
                        } else {
                            return Err(chunk_idx)
                        }
                    };
                    //println!("  {}, using {}", if is_div { "is_div"} else {"!is_div"}, num);
                    output = output*10+num;
                    num
                }
                "add" => arg0() + arg1(),
                "mul" => arg0() * arg1(),
                "div" => arg0() / arg1(),
                "mod" => arg0() % arg1(),
                "eql" => {
                    if arg0() == arg1() {
                        1
                    } else {
                        0
                    }
                }
                _ => unimplemented!(),
            };

            //println!("{} => {}", line, reg[target]);
        }
    }

    //report(reg[3]);

    Ok(output)
}
