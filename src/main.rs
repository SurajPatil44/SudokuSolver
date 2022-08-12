use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, path::Path};

#[derive(Copy, Clone)]
struct SudokuSolver<const N: usize> {
    matrix: [[Tile<N>; N]; N],
    done: bool,
    num_calls: usize,
}

#[derive(Copy, Clone, Debug)]
struct Tile<const N: usize> {
    num: u8,
    pos_sols: [bool; N],
}

impl<const N: usize> Default for Tile<N> {
    fn default() -> Tile<N> {
        Tile {
            num: 0u8,
            pos_sols: [false; N],
        }
    }
}

impl<const N: usize> Default for SudokuSolver<N> {
    fn default() -> SudokuSolver<N> {
        SudokuSolver {
            matrix: [[Tile::<N>::default(); N]; N],
            done: false,
            num_calls: 0,
        }
    }
}

impl<const N: usize> SudokuSolver<N> {
    fn read_files<P: AsRef<Path>>(mut self, path: P) -> Self {
        let matfile = File::open(path).expect("can't open file");
        let mut lines = io::BufReader::new(matfile).lines();
        let _ = lines.next().unwrap();
        for (iind, line) in lines.enumerate() {
            let line = line.unwrap();
            for (jind, num) in line.chars().enumerate() {
                // print!("{} ", num);
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
        let check = (0..N).all(|i| self.matrix[row][i].num != num)
            && (0..N).all(|i| self.matrix[i][col].num != num);
        let row = row - row % 3;
        let col = col - col % 3;
        check
            && (0..3)
                .zip(0..3)
                .all(|(i, j)| self.matrix[row + i][col + j].num != num)
    }

    fn place(&mut self, row: usize, col: usize, num: u8) {
        self.matrix[row][col].num = num;
        if num > 0 {
            self.matrix[row][col].pos_sols[usize::from(num - 1)] = false;
        }
    }

    fn setup(&mut self) {
        for iind in 0..9 {
            for jind in 0..9 {
                if self.matrix[iind][jind].num == 0 {
                    for i in 1..10 {
                        if self.check_matrix(iind, jind, i as u8) {
                            self.matrix[iind][jind].pos_sols[i - 1] = true;
                        }
                    }
                }
            }
        }
    }

    fn solver(&mut self) -> bool {
        let mut checking_range = BinaryHeap::new();
        for iind in 0..9 {
            for jind in 0..9 {
                if self.matrix[iind][jind].num == 0 {
                    let mut pos_sols: usize = 0;
                    for i in 1..10 {
                        if self.check_matrix(iind, jind, i as u8) {
                            pos_sols += 1;
                            self.matrix[iind][jind].pos_sols[i - 1] = true;
                        }
                    } 
                   /* 
                    let pos_sols = self.matrix[iind][jind]
                        .pos_sols
                        .iter()
                        .filter(|&n| *n)
                        .count();
                    */
                    checking_range.push(Reverse((pos_sols, iind, jind)));
                }
            }
        }
        let first = checking_range.pop();
        if first.is_none() {
            return true;
        }
        let minimum_loc: (usize, usize) = (first.unwrap().0 .1, first.unwrap().0 .2);
        //let low = first.0;

        for i in 1..10 {
            if self.check_matrix(minimum_loc.0, minimum_loc.1, i as u8) {
                self.place(minimum_loc.0, minimum_loc.1, i as u8);
                if self.solver() {
                    return true;
                }
                self.num_calls += 1;
                self.place(minimum_loc.0, minimum_loc.1, 0 as u8);
            }
        }

        false
    }

    fn solve(&mut self) -> bool {
        self.setup();
        self.solver()
    }

    fn print_matrix(&self) {
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
    let solver = SudokuSolver::<9>::default();
    let mut solver = solver.read_files(&args[1]);
    solver.print_matrix();
    if solver.solve() {
        println!("took {} calls", solver.num_calls);
        solver.print_matrix();
    }
}
