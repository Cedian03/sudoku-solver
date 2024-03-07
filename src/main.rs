use std::fmt;

#[derive(Clone, Copy, Default)]
struct Sudoku {
    pub grid: [[u8; 9]; 9],
}

impl Sudoku {
    fn new() -> Self {
        Self::default()
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y][x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.grid[y][x]
    }

    fn set(&mut self, x: usize, y: usize, num: u8) {
        assert!((1..=9).contains(&num));
        *self.get_mut(x, y) = num;
    }

    fn get_row(&self, y: usize) -> Vec<u8> {
        (0..9).map(|x| self.get(x, y)).collect()
    }

    fn get_column(&self, x: usize) -> Vec<u8> {
        (0..9).map(|y| self.get(x, y)).collect()
    }

    fn get_region_of(&self, x: usize, y: usize) -> Vec<u8> {
        let rx = (x / 3) * 3;
        let ry = (y / 3) * 3;
        (0..9).map(|i| self.get(rx + i % 3, ry + i / 3)).collect()
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..9 {
            for x in 0..9 {
                write!(f, "{}", self.get(x, y))?;

                if x != 8 {
                    write!(f, " ")?;
                }

                if x == 8 && y != 8 {
                    write!(f, "\n")?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for y in 0..9 {
            for x in 0..9 {
                write!(f, "{}", self.get(x, y))?;

                if x != 8 {
                    write!(f, ", ")?;
                }

                if x == 8 && y != 8 {
                    write!(f, "\n ")?;
                }
            }
        }
        write!(f, "]")
    }
}

fn solve(sudoku: Sudoku) -> Option<Sudoku> {
    match find_next_empty_cell(&sudoku) {
        Some((x, y)) => {
            for num in valid_numbers(&sudoku, x, y) {
                let mut new_sudoku = sudoku.clone();
                new_sudoku.set(x, y, num);
                if let Some(solved) = solve(new_sudoku) {
                    return Some(solved);
                }
            }
            None
        }
        None => Some(sudoku),
    }
}

fn find_next_empty_cell(sudoku: &Sudoku) -> Option<(usize, usize)> {
    for y in 0..9 {
        for x in 0..9 {
            if sudoku.get(x, y) == 0 {
                return Some((x, y));
            }
        }
    }
    None
}

fn valid_numbers(sudoku: &Sudoku, x: usize, y: usize) -> Vec<u8> {
    let row = sudoku.get_row(y);
    let col = sudoku.get_column(x);
    let reg = sudoku.get_region_of(x, y);
    (1..=9)
        .filter(|n| !(row.contains(n) || col.contains(n) || reg.contains(n)))
        .collect()
}

fn main() {
    let sudoku = Sudoku::new();
    println!("{}", solve(sudoku).unwrap());
}
