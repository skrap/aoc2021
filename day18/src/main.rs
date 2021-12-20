fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> i32 {
    let sns = parse(input);
    let mut it = sns.into_iter();
    let sn = it.next().unwrap();
    let sn = it.fold(sn, |a, b| reduce(add(a, b)));
    mag(sn)
}

fn part2(input: &str) -> i32 {
    let sns = parse(input);
    let mut max_mag = 0;
    for sn1 in &sns {
        for sn2 in &sns {
            max_mag = max_mag.max(mag(reduce(add(sn1.to_vec(), sn2.to_vec()))));
            max_mag = max_mag.max(mag(reduce(add(sn2.to_vec(), sn1.to_vec()))));
        }
    }
    max_mag
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Sym {
    Open,
    Close,
    Num(i32),
}

fn parse_line(line: &str) -> Vec<Sym> {
    let mut result = Vec::new();
    let mut peeks = line.trim().chars().peekable();

    while let Some(n) = peeks.next() {
        let sym = match n {
            ']' => Sym::Close,
            '[' => Sym::Open,
            ',' => continue,
            ch => {
                assert!(ch.is_digit(10));
                let mut s = String::new();
                s.push(ch);
                while let Some(ch) = peeks.next_if(|ch| ch.is_digit(10)) {
                    s.push(ch);
                }
                Sym::Num(s.parse().unwrap())
            }
        };
        result.push(sym)
    }

    result
}

fn parse(input: &str) -> Vec<Vec<Sym>> {
    input.trim().lines().map(parse_line).collect()
}

fn add(mut a: Vec<Sym>, mut b: Vec<Sym>) -> Vec<Sym> {
    let mut result = vec![Sym::Open];
    result.append(&mut a);
    result.append(&mut b);
    result.push(Sym::Close);
    result
}

fn reduce(mut s: Vec<Sym>) -> Vec<Sym> {
    'reduce: loop {
        // exploding
        let mut depth = 0;
        let mut last_nidx: Option<(usize, i32)> = None;
        for i in 0..s.len() - 1 {
            match s[i] {
                Sym::Open => {
                    if depth >= 4 {
                        let four: Vec<_> = s
                            .splice(i..(i + 4), vec![Sym::Num(0)].into_iter())
                            .collect();
                        match four[0..4] {
                            [Sym::Open, Sym::Num(n1), Sym::Num(n2), Sym::Close] => {
                                if let Some(last_nidx) = last_nidx {
                                    s[last_nidx.0] = Sym::Num(last_nidx.1 + n1);
                                }
                                for i in (i + 1)..s.len() {
                                    match &mut s[i] {
                                        Sym::Num(n) => {
                                            *n += n2;
                                            break;
                                        }
                                        _ => (),
                                    }
                                }
                                continue 'reduce;
                            }
                            _ => unimplemented!(),
                        };
                    }
                    depth += 1;
                }
                Sym::Close => depth -= 1,
                Sym::Num(n) => {
                    last_nidx = Some((i, n));
                }
            }
        }

        // try splitting
        for i in 0..s.len() - 1 {
            match s[i] {
                Sym::Num(n) if n >= 10 => {
                    s.splice(
                        i..=i,
                        vec![
                            Sym::Open,
                            Sym::Num(n / 2),
                            Sym::Num((n + 1) / 2),
                            Sym::Close,
                        ]
                        .into_iter(),
                    );
                    continue 'reduce;
                }
                _ => (),
            }
        }

        break;
    }

    s
}

enum SnailNum {
    Num(i32),
    Pair(Box<SnailNum>, Box<SnailNum>),
}

impl SnailNum {
    fn mag(&self) -> i32 {
        match self {
            SnailNum::Num(x) => *x,
            SnailNum::Pair(b1, b2) => 3 * b1.mag() + 2 * b2.mag(),
        }
    }
}

fn to_sn<'a>(it: &mut impl Iterator<Item = &'a Sym>) -> SnailNum {
    match it.next().unwrap() {
        Sym::Open => {
            let v = SnailNum::Pair(Box::new(to_sn(it)), Box::new(to_sn(it)));
            assert!(matches!(it.next().unwrap(), Sym::Close));
            v},
        Sym::Num(x) => SnailNum::Num(*x),
        Sym::Close => todo!(),
    }
}

fn mag(sn: Vec<Sym>) -> i32 {
    to_sn(&mut sn.iter()).mag()
}

#[test]
fn test_part1() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    assert_eq!(part1(input), 4140);
}

#[test]
fn test_reduce() {
    assert_eq!(
        reduce(parse_line("[[[[[9,8],1],2],3],4]")),
        parse_line("[[[[0,9],2],3],4]")
    );
    assert_eq!(
        reduce(parse_line("[7,[6,[5,[4,[3,2]]]]]")),
        parse_line("[7,[6,[5,[7,0]]]]")
    );
    assert_eq!(
        reduce(parse_line("[[6,[5,[4,[3,2]]]],1]")),
        parse_line("[[6,[5,[7,0]]],3]")
    );
    assert_eq!(
        reduce(parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
        parse_line("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    );
}
