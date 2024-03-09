use std::fmt;

#[derive(Clone, Copy, Debug)]
struct ParseSudokuError;

#[derive(Clone, Default)]
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

    fn get_region(&self, rx: usize, ry: usize) -> Vec<u8> {
        let x =  rx * 3;
        let y =  ry * 3;
        (0..9).map(|i| self.get(x + i % 3, y + i / 3)).collect()
    }

    fn get_region_of(&self, x: usize, y: usize) -> Vec<u8> {
        self.get_region(x / 3, y / 3)
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

fn solve(sudoku: &Sudoku) -> Option<Sudoku> {
    match find_next_empty_cell(sudoku) {
        Some((x, y)) => {
            for num in find_valid_numbers(sudoku, x, y) {
                let mut new_sudoku = sudoku.clone();
                new_sudoku.set(x, y, num);
                if let Some(solved) = solve(&new_sudoku) {
                    return Some(solved);
                }
            }
            None
        }
        None => Some(sudoku.clone()),
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

fn find_valid_numbers(sudoku: &Sudoku, x: usize, y: usize) -> Vec<u8> {
    let row = sudoku.get_row(y);
    let col = sudoku.get_column(x);
    let reg = sudoku.get_region_of(x, y);
    (1..=9)
        .filter(|n| !(row.contains(n) || col.contains(n) || reg.contains(n)))
        .collect()
}

fn is_solution_to(source: &Sudoku, solution: &Sudoku) -> bool {
    is_true_to_source(source, solution) && is_solved(solution)
}

fn is_true_to_source(source: &Sudoku, solution: &Sudoku) -> bool {
    (0..81).all(|i| {
        let x = i % 9;
        let y = i / 9;
        let cell = source.get(x, y);
        cell == 0 || cell == solution.get(x, y)
    })
}

fn is_solved(sudoku: &Sudoku) -> bool {
    is_full(sudoku) && is_true(sudoku)
}

fn is_full(sudoku: &Sudoku) -> bool {
    (0..81).all(|i| sudoku.get(i % 9, i % 9) != 0)
}

fn is_true(sudoku: &Sudoku) -> bool {
    (0..9).all(|i| {
        !(contains_dup(&sudoku.get_row(i))
            || contains_dup(&sudoku.get_column(i))
            || contains_dup(&sudoku.get_region(i % 3, i / 3)))
    })
}

fn contains_dup(slice: &[u8]) -> bool {
    slice.iter().enumerate().any(|(i, x)| *x != 0 && slice[0..i].contains(x))
}

fn main() {
    let sudoku = Sudoku { grid: [
        [0, 2, 0, 0, 0, 0, 0, 0, 5],
        [0, 0, 7, 0, 8, 5, 0, 2, 0],
        [0, 0, 9, 0, 0, 0, 0, 0, 0],
        [9, 0, 4, 0, 0, 0, 1, 0, 0],
        [1, 8, 6, 0, 0, 0, 0, 0, 0],
        [0, 5, 0, 0, 0, 4, 0, 0, 8],
        [0, 0, 1, 0, 0, 0, 6, 0, 0],
        [0, 7, 5, 9, 0, 8, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 7, 0, 0]
    ]};

    assert!(is_true(&sudoku), "Cannot begin to solve invalid sudoku");

    match solve(&sudoku) {
        Some(sol) => {
            assert!(is_solution_to(&sudoku, &sol));
            println!("{}", sol);
        }
        None => {
            println!("No solution could be found");
        }
    };
}
