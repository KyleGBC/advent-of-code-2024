use itertools::iproduct;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos(usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Direction {
    dx: isize,
    dy: isize,
}
impl Direction {
    fn next(&self, pos: Pos) -> Option<Pos> {
        let x = (pos.0 as isize + self.dx).try_into().ok();
        let y = (pos.1 as isize + self.dy).try_into().ok();
        if x.is_some() && y.is_some() {
            Some(Pos(x.unwrap(), y.unwrap()))
        } else {
            None
        }
    }
    fn all() -> Vec<Direction> {
        let mut directions = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    directions.push(Direction { dx, dy });
                }
            }
        }
        directions
    }
}

struct Board {
    width: usize,
    height: usize,
    chars: Vec<Vec<char>>,
}
impl Board {
    fn get(&self, pos: Pos) -> Option<char> {
        if pos.0 >= self.width || pos.1 >= self.height {
            None
        } else {
            Some(self.chars[pos.1][pos.0])
        }
    }

    fn check_for_words(&self, pos: Pos, word: &str) -> usize {
        Direction::all()
            .iter()
            .map(|dir| self.check_for_word(pos, word, *dir))
            .filter(|x| *x)
            .count()
    }

    fn check_for_word(&self, mut pos: Pos, word: &str, direction: Direction) -> bool {
        if word.chars().next().is_none_or(|c| self.get(pos) != Some(c)) {
            return false;
        }
        for c in word.chars().skip(1) {
            if let Some(next) = direction.next(pos) {
                pos = next;
            } else {
                return false;
            }

            let current = self.get(pos);
            if current != Some(c) {
                return false;
            }
        }
        return true;
    }

    fn check_for_x(&self, pos: Pos) -> bool {
        if self.get(pos) != Some('A') {
            return false;
        }

        let up_left = Direction { dx: -1, dy: -1 }
            .next(pos)
            .map(|p| self.get(p))
            .flatten();
        let up_right = Direction { dx: 1, dy: -1 }
            .next(pos)
            .map(|p| self.get(p))
            .flatten();
        let down_left = Direction { dx: -1, dy: 1 }
            .next(pos)
            .map(|p| self.get(p))
            .flatten();
        let down_right = Direction { dx: 1, dy: 1 }
            .next(pos)
            .map(|p| self.get(p))
            .flatten();

        if self.get(pos) != Some('A') {
            return false;
        }
        let first_arm = match (up_left, down_right) {
            (Some('M'), Some('S')) => true,
            (Some('S'), Some('M')) => true,
            _ => false,
        };

        let second_arm = match (up_right, down_left) {
            (Some('M'), Some('S')) => true,
            (Some('S'), Some('M')) => true,
            _ => false,
        };

        first_arm && second_arm
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let board = Board {
        width: chars[0].len(),
        height: chars.len(),
        chars,
    };
    let board_iter = iproduct!(0..board.height, 0..board.width);

    let part1: usize = board_iter
        .clone()
        .map(|(y, x)| board.check_for_words(Pos(x, y), "XMAS"))
        .sum();
    let part2: usize = board_iter
        .filter(|(y, x)| board.check_for_x(Pos(*x, *y)))
        .count();

    println!("Part 1: {part1}, Part 2: {part2}");
}
