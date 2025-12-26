#[derive(Debug, Default, Clone)]
enum Op {
    #[default]
    Add,
    Mul,
}
impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Got a non [+ or *] op symbol."),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn w(&self) -> usize {
        self.grid[0].len()
    }
    fn h(&self) -> usize {
        self.grid.len()
    }
    fn at(&self, r: usize, c: usize) -> char {
        self.grid[r][c]
    }
}

impl<'a> FromIterator<&'a str> for Grid {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let grid = iter.into_iter().map(|s| s.chars().collect()).collect();
        Self { grid }
    }
}

#[derive(Debug, Clone)]
struct Accumulator {
    op: Op,
    val: usize,
}

impl Accumulator {
    fn acc(&mut self, val: usize) {
        self.val = match self.op {
            Op::Add => self.val + val,
            Op::Mul => self.val * val,
        }
    }
}

impl From<Op> for Accumulator {
    fn from(op: Op) -> Self {
        let val = match op {
            Op::Add => 0,
            Op::Mul => 1,
        };
        Accumulator { op, val }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut line_strs = input.lines().map(|s| s.split_whitespace()).rev();
    let mut accs = line_strs
        .next()
        .unwrap()
        .map(Op::from)
        .map(Accumulator::from)
        .collect::<Vec<_>>();
    let line_values = line_strs.map(|l| l.map(|s| s.parse::<usize>().unwrap()));
    for line in line_values {
        for (acc, val) in accs.iter_mut().zip(line) {
            acc.acc(val);
        }
    }
    accs.iter().map(|a| a.val).sum()
}

pub fn part_2(input: &str) -> usize {
    let grid: Grid = input.lines().collect();
    let w = grid.w();
    let h = grid.h();
    let mut total = 0;
    let mut numbers = Vec::new();
    let mut op_idx = 0;
    let mut op_complete = false;
    for column in (0..w).rev() {
        if op_complete {
            op_complete = false;
            op_idx = 0;
            numbers.clear();
            continue;
        }
        numbers.push(0);

        for row in 0..h {
            // dbg!((column, row, op_idx));
            match grid.at(row, column) {
                c if c.is_ascii_digit() => {
                    numbers[op_idx] = numbers[op_idx] * 10 + c.to_digit(10).unwrap() as usize;
                    // dbg!(&numbers, op_idx, c);
                }
                ' ' if row == h - 1 => op_idx += 1,
                '+' => {
                    let value = numbers.iter().sum::<usize>();
                    // dbg!('+', &numbers, value);
                    total += value;
                    op_complete = true
                }
                '*' => {
                    let value = numbers.iter().take(h - 1).product::<usize>();
                    // dbg!('*', &numbers, value);
                    total += value;
                    op_complete = true
                }
                _ => {}
            }
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    #[test]
    fn example1() {
        let input = indoc! {"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "};
        assert_eq!(part_1(input), 4277556);
    }
    #[test]
    fn example2() {
        let input = indoc! {"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "};
        assert_eq!(part_2(input), 3263827);
    }
}
