use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;
use std::{env, path::Path};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Copy, Clone)]
struct SudokuSolver {
    matrix: [[Tile; 9]; 9],
    done: bool,
    num_calls: usize,
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    num: u8,
    pos_sols: [bool; 9],
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            num: 0u8,
            pos_sols: [false; 9],
        }
    }
}

impl Default for SudokuSolver {
    fn default() -> SudokuSolver {
        SudokuSolver {
            matrix: [[Tile::default(); 9]; 9],
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
                print!("{} ", num);
                let num = num.to_digit(10).unwrap() as u8;
                self.matrix[iind][jind].num = num;
            }
            //println!();
        }
        self.done = false;
        self.num_calls = 0;
        self
    }

    fn check_matrix(&self, row: usize, col: usize, num: u8) -> bool {
        let mut check = false;
        //early return

        if !self.matrix[row][col].pos_sols[(num - 1) as usize] {
            return !check;
        }
        for i in 0..9 {
            if self.matrix[row][i].num == num {
                check = true;
            }
        }
        for i in 0..9 {
            if self.matrix[i][col].num == num {
                check = true;
            }
        }
        let row = row - row % 3;
        let col = col - col % 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.matrix[row + i][col + j].num == num {
                    check = true;
                }
            }
        }
        return !check;
    }

    fn place(&mut self, row: usize, col: usize, num: u8) {
        self.matrix[row][col].num = num;
    }

    fn solver(&mut self) -> bool {
        self.num_calls += 1;
        let mut break_cond = false;
        let mut checking_range = BinaryHeap::new();
        for iind in 0..9 {
            for jind in 0..9 {
                if self.matrix[iind][jind].num == 0 {
                    break_cond = true;
                    let mut pos_sols: usize = 0;
                    for i in 1..10 {
                        if self.check_matrix(iind, jind, i as u8) {
                            pos_sols += 1;
                            self.matrix[iind][jind].pos_sols[i - 1] = true;
                        }
                    }
                    checking_range.push(Reverse((pos_sols,iind,jind)));
                }
            }
        }

        if !break_cond {
            println!();
            println!("took {} iterations to calculate solution", self.num_calls);
            println!();
            self.print_matrix();
            exit(0);
        }

        let first = checking_range.pop().unwrap().0;
        let minimum_loc: (usize, usize) = (first.1, first.2);
        //let low = first.0;

        for i in 1..10 {
            if self.check_matrix(minimum_loc.0, minimum_loc.1, i as u8) {
                self.place(minimum_loc.0, minimum_loc.1, i as u8);
                if self.solver() {
                    return true;
                }
                self.place(minimum_loc.0, minimum_loc.1, 0 as u8);
            }
        }

        false
    }

    fn print_matrix(self) {
        let mut i = 0;
        for row in &self.matrix[..] {
            i += 1;
            if i % 3 == 1 && i > 3 {
                println!("-----------------------");
            }
            for (j, tile) in row.iter().enumerate() {
                if j % 3 == 0 && j > 0 {
                    print!(" | ");
                }
                match tile.num {
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
    let mut solver = solver.read_files(&args[1]);
    solver.print_matrix();
    solver.solver();
}
