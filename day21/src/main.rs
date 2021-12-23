use std::{collections::HashMap, ops::Add};

fn main() {
    let input = (10,6);
    dbg!(part1(input));
    dbg!(part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Player {
    pos_1: i32,
    score: i32,
}

impl Player {
    fn do_move(&self, rolls: [i32;3]) -> Self {
        let mut pos = self.pos_1 + rolls.iter().sum::<i32>();
        pos %= 10;
        Self {
            score: self.score + pos + 1,
            pos_1: pos,
        }
    }
}

#[derive(Default)]
struct DetDie {
    val: i32,
    rolls: i32,
}

impl DetDie {
    fn next(&mut self) -> i32 {
        self.val += 1;
        if self.val > 100 {
            self.val = 1;
        }
        self.rolls += 1;
        self.val
    }
}

fn part1(input: (i32,i32)) -> i32 {
    let mut p1 = Player { pos_1: input.0-1, score: 0 };
    let mut p2 = Player { pos_1: input.1-1, score: 0 };
    let mut die = DetDie::default();

    let mut do_move = |p: &mut Player| {
        let rolls = [die.next(), die.next(), die.next()];
        *p = p.do_move(rolls);
        // println!("P rolls {},{},{} move {} score {}",
        //     rolls[0],rolls[1],rolls[2], p.pos_1 + 1, p.score
        // );
    };

    loop {
        do_move(&mut p1);
        if p1.score >= 1000 {
            break;
        }
        
        do_move(&mut p2);
        if p2.score >= 1000 {
            break;
        }
    }

    die.rolls * p1.score.min(p2.score)
}

#[derive(Clone, Copy)]
struct Wins(usize,usize);

impl Add for Wins {
    type Output = Wins;

    fn add(self, rhs: Self) -> Self::Output {
        Wins(self.0+rhs.0, self.1+rhs.1)
    }
}

fn dirac_wins(input: (Player,Player), memo: &mut HashMap<(Player,Player), Wins>) -> Wins {
    if input.0.score >= 21 {
        return Wins(1,0)
    }
    if input.1.score >= 21 {
        return Wins(0,1)
    }
    if let Some(out) = memo.get(&input) {
        return *out;
    }

    // assume it's input.0's turn
    let wins = 
        dirac_wins((input.1, input.0.do_move([1,1,1])), memo) +
        dirac_wins((input.1, input.0.do_move([1,1,2])), memo) +
        dirac_wins((input.1, input.0.do_move([1,1,3])), memo) +

        dirac_wins((input.1, input.0.do_move([1,2,1])), memo) +
        dirac_wins((input.1, input.0.do_move([1,2,2])), memo) +
        dirac_wins((input.1, input.0.do_move([1,2,3])), memo) +
        
        dirac_wins((input.1, input.0.do_move([1,3,1])), memo) +
        dirac_wins((input.1, input.0.do_move([1,3,2])), memo) +
        dirac_wins((input.1, input.0.do_move([1,3,3])), memo) +
        
        dirac_wins((input.1, input.0.do_move([2,1,1])), memo) +
        dirac_wins((input.1, input.0.do_move([2,1,2])), memo) +
        dirac_wins((input.1, input.0.do_move([2,1,3])), memo) +
        
        dirac_wins((input.1, input.0.do_move([2,2,1])), memo) +
        dirac_wins((input.1, input.0.do_move([2,2,2])), memo) +
        dirac_wins((input.1, input.0.do_move([2,2,3])), memo) +
        
        dirac_wins((input.1, input.0.do_move([2,3,1])), memo) +
        dirac_wins((input.1, input.0.do_move([2,3,2])), memo) +
        dirac_wins((input.1, input.0.do_move([2,3,3])), memo) +
        
        dirac_wins((input.1, input.0.do_move([3,1,1])), memo) +
        dirac_wins((input.1, input.0.do_move([3,1,2])), memo) +
        dirac_wins((input.1, input.0.do_move([3,1,3])), memo) +
        
        dirac_wins((input.1, input.0.do_move([3,2,1])), memo) +
        dirac_wins((input.1, input.0.do_move([3,2,2])), memo) +
        dirac_wins((input.1, input.0.do_move([3,2,3])), memo) +
        
        dirac_wins((input.1, input.0.do_move([3,3,1])), memo) +
        dirac_wins((input.1, input.0.do_move([3,3,2])), memo) +
        dirac_wins((input.1, input.0.do_move([3,3,3])), memo);

    let wins = Wins(wins.1, wins.0);

    memo.insert(input, wins);
    
    wins
}

fn part2(input: (i32,i32)) -> usize {
    let mut memo = HashMap::new();
    let input = (
        Player { pos_1: input.0-1, score: 0 },
        Player { pos_1: input.1-1, score: 0 },
    );
    let counts = dirac_wins(input, &mut memo);
    counts.0.max(counts.1)
}

#[test]
fn test_part1() {
    assert_eq!(part1((4,8)), 739785);
    assert_eq!(part2((4,8)), 444356092776315);
}