use itertools::Itertools;

/// # Panics
/// This function panics if the input cannot be read correctly
/// # Errors
/// This function returns an `AoCError` with variants "Cache" and "Network"
pub fn main() {
    let year = 2025;
    let day = 4;
    libaoc::evaluate(part_1, part_2, year, day);
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    columns: usize, // Actual columsn are columns + 1 for newline
    cells: Vec<u8>,
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Grid {
    fn at(&self, r: usize, c: usize) -> u8 {
        self.cells[(self.columns + 1) * r + c]
    }
    fn remove(&mut self, r: usize, c: usize) {
        self.cells[(self.columns + 1) * r + c] = b'.';
    }
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_sign_loss)]
    fn adj(&self, row: usize, column: usize) -> usize {
        let mut count = 0;
        for (dr, dc) in NEIGHBORS {
            let r = row.wrapping_add_signed(dr);
            let c = column.wrapping_add_signed(dc);
            // println!("{row}:{column} looking {dr}:{dc}");
            if r < self.rows && c < self.columns && (self.at(r, c) == b'@') {
                count += 1;
            }
        }
        // println!("{row}:{column} = {count}");
        count
    }
    fn less_than(&self, n: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows)
            .cartesian_product(0..self.columns)
            .filter(move |(r, c)| self.at(*r, *c) == b'@' && self.adj(*r, *c) < n)
    }

    fn count_less_than(self, n: usize) -> usize {
        self.less_than(n).count()
    }

    fn prune_less_than(&mut self, n: usize) -> usize {
        let remove = self.less_than(n).collect::<Vec<(usize, usize)>>();
        let removed = remove.len();
        for (r, c) in remove {
            self.remove(r, c);
        }
        removed
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let columns = s.bytes().position(|b| b == b'\n').unwrap_or(s.len());
        let rows = (s.len() + 1) / (columns + 1);
        Self {
            rows,
            columns,
            cells: s.as_bytes().to_vec(),
        }
    }
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.count_less_than(4)
}

fn part_2(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut step_removed = grid.prune_less_than(4);
    let mut removed = step_removed;
    while step_removed > 0 {
        step_removed = grid.prune_less_than(4);
        removed += step_removed;
    }
    removed
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_str() {
        let input = r"123
456
780
456";
        let grid = Grid::from(input);
        assert_eq!(grid.rows, 4);
        assert_eq!(grid.columns, 3);
        assert_eq!(grid.at(0, 0), b'1');
    }
    #[test]
    fn test_adj() {
        let input = "123
@26
@8@
45@";
        let grid = Grid::from(input);
        assert_eq!(grid.adj(2, 1), 4);
        assert_eq!(grid.adj(0, 0), 1);
        assert_eq!(grid.adj(0, 2), 0);
    }

    #[test]
    fn example() {
        println!("START");
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(Grid::from(input).count_less_than(4), 13);
    }
    #[test]
    fn example2() {
        println!("START");
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = part_2(input);
        assert_eq!(result, 43);
    }
}
