#[derive(Debug)]
struct Sq {
    marked: bool,
    id: i32,
}

struct Board {
    squares: [Sq; 25], // [row*5 + col]
}

impl Board {
    fn unmarked_sum(&self) -> i32 {
        let unmarked_sum = self
            .squares
            .iter()
            .filter(|sq| !sq.marked)
            .map(|sq| sq.id)
            .sum();
        unmarked_sum
    }

    fn mark(&mut self, id: i32) -> bool {
        for sq in self.squares.iter_mut() {
            if !sq.marked && sq.id == id {
                sq.marked = true;
                return true;
            }
        }
        false
    }

    fn has_won(&self) -> bool {
        for row in 0..5 {
            if (0..5).all(|col| self.squares[row * 5 + col].marked) {
                return true;
            }
        }
        for col in 0..5 {
            if (0..5).all(|row| self.squares[row * 5 + col].marked) {
                return true;
            }
        }
        false
    }
}

fn read_board(input: &str) -> Board {
    Board {
        squares: input
            .split_whitespace()
            .map(|num| Sq {
                id: num.parse().unwrap(),
                marked: false,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    }
}

fn parse(input: &str) -> (Vec<i32>, Vec<Board>) {
    let nums;
    let mut boards = vec![];

    let mut sections = input.split("\n\n");
    nums = sections
        .next()
        .unwrap()
        .trim()
        .split(',')
        .flat_map(str::parse)
        .collect();

    boards = sections.map(read_board).collect();

    (nums, boards)
}

struct Game {
    nums: Vec<i32>,
    boards: Vec<Board>,
    num_i: usize,
}

impl Game {
    fn new(input: &str) -> Self {
        let (nums, mut boards) = parse(input);
        Self {
            nums,
            boards,
            num_i: 0,
        }
    }
}

impl Iterator for Game {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        for num_i in self.num_i..self.nums.len() {
            let num = self.nums[num_i];
            for board in self.boards.iter_mut() {
                if board.has_won() { continue; }
                if board.mark(num) && board.has_won() {
                    let unmarked_sum = board.unmarked_sum();
                    self.num_i = num_i;
                    return Some(unmarked_sum * num);
                }
            }
        }
        None
    }
}

fn part1(input: &str) -> i32 {
    let mut game = Game::new(input);
    game.next().unwrap()
}

fn part2(input: &str) -> i32 {
    let game = Game::new(input);
    game.last().unwrap()
}

fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}
