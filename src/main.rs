use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;
use std::{env, path::Path};

#[derive(Copy, Clone)]
struct SudokuSolver {
    matrix: [[u8; 9]; 9],
    done: bool,
    num_calls: usize,
}

//#[derive(Debug)]
//struct SolveHelper(usize,usize,usize);

impl Default for SudokuSolver {
    fn default() -> SudokuSolver {
        SudokuSolver {
            matrix: [[0u8; 9]; 9],
            done: false,
            num_calls: 0,
        }
    }
}
impl SudokuSolver {
    fn read_files<P: AsRef<Path>>(mut self, path: P) -> Self {
        let matfile = File::open(path).expect("can't open file");
        let mut lines = io::BufReader::new(matfile).lines();
        let _ = lines.next().unwrap();
        for (iind, line) in lines.enumerate() {
            let line = line.unwrap();
            for (jind, num) in line.chars().enumerate() {
                //print!("{} ",num);
                self.matrix[iind][jind] = num.to_digit(10).unwrap() as u8;
            }
            //println!();
        }
        self.done = false;
        self.num_calls = 0;
        self
    }

    fn check_matrix(self, row: usize, col: usize, num: u8) -> bool {
        let mut check = false;
        for i in 0..9 {
            if self.matrix[row][i] == num {
                check = true;
            }
        }
        for i in 0..9 {
            if self.matrix[i][col] == num {
                check = true;
            }
        }
        let row = row - row % 3;
        let col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.matrix[row + i][col + j] == num {
                    check = true;
                }
            }
        }
        return !check;
    }

    fn print_matrix(self) {
        let mut i = 0;
        for row in &self.matrix[..] {
            i += 1;
            if i % 3 == 1 && i > 3 {
                println!("-----------------------");
            }
            for (j, num) in row.iter().enumerate() {
                if j % 3 == 0 && j > 0 {
                    print!(" | ");
                }
                match *num {
                    0 => print!("_"),
                    n => print!("{}", n),
                }
                if j % 3 < 2 {
                    print!("|");
                } else {
                    print!("");
                }
            }
            println!();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let solver = SudokuSolver::default();
    let solver = solver.read_files(&args[1]);
    solver.print_matrix();
    //solver.solve(solver, solver.matrix);
}
