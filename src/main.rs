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

    fn check_matrix(&self, row: usize, col: usize, num: u8) -> bool {
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

    fn place(&mut self, row: usize, col: usize, num: u8) {
            self.matrix[row][col] = num;
    }

    fn solver(&mut self) -> bool {
        self.num_calls += 1;
        let mut break_cond = false;
        let mut checking_range = Vec::<(usize,usize,usize)>::new();
        for iind in 0..9{
            for jind in 0..9{
                if self.matrix[iind][jind] == 0{
                    break_cond = true;
                    let mut pos_sols : usize = 0;
                    for i in 1..10 {
                        if self.check_matrix(iind,jind,i as u8){
                            pos_sols += 1;
                        }
                    }
                    checking_range.push((iind,jind,pos_sols));
                }
            }
        }

        if !break_cond {
            println!();
            println!("took {} iterations to calculate solution",self.num_calls);
            println!();
            self.print_matrix();
            exit(0);
        }
        //println!("{:#?}",checking_range);
        let mut minimum_loc : (usize,usize) = (checking_range[0].0,checking_range[0].1);
        let mut low = checking_range[0].2;

        for elem in checking_range{
            if elem.2 < low {
                minimum_loc = (elem.0,elem.1);
                low = elem.2;
            }
        }

/*
	for i in range(0,10):
		if check_soduku(row,column,i,matrix):
			matrix[row][column]=i
			if sudoku_solver(matrix):
				return True
			matrix[row][column]=0
        return False
*/
        //println!("minimum after {} iterations {:?} , {} ",self.num_calls,minimum_loc,low);
        for i in 1..10 {
            if self.check_matrix(minimum_loc.0,minimum_loc.1,i as u8){
                self.place(minimum_loc.0,minimum_loc.1,i as u8);
                if self.solver() {
                    return true;
                }
                self.place(minimum_loc.0,minimum_loc.1,0 as u8);
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
    let mut solver = solver.read_files(&args[1]);
    solver.print_matrix();
    solver.solver();
}
