use crossterm::{
    cursor,
    style::{self},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, Stdout, Write};
use std::thread;
use std::time::Duration;
use std::{env, path::Path};

//#[derive(Clone)]
struct SudokuSolver<const N: usize> {
    matrix: [[Tile<N>; N]; N],
    done: bool,
    num_calls: usize,
    printer: Stdout,
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

impl<const N: usize> SudokuSolver<N> {
    fn new(printer: Stdout) -> SudokuSolver<N> {
        SudokuSolver {
            matrix: [[Tile::<N>::default(); N]; N],
            done: false,
            num_calls: 0,
            printer: printer,
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
        //early exit
        /*if self.matrix[row][col].pos_sols[usize::from(num - 1)] {
            return true;
        }*/
        /*let check = (0..N).all(|i| self.matrix[row][i].num != num)
            && (0..N).all(|i| self.matrix[i][col].num != num);
        */
        let mut check = false;
        for i in 0..N {
            if self.matrix[row][i].num == num {
                check = true;
            }
        }
        for i in 0..N {
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
        !check
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
        let mut break_condition = true;
        let mut checking_range = BinaryHeap::new();

        for i in 0..N {
            for j in 0..N {
                if self.matrix[i][j].num == 0 {
                    let mut pos_sols: usize = 0;
                    break_condition = false;
                    for num in 1..N + 1 {
                        if self.check_matrix(i, j, num as u8) {
                            pos_sols += 1;
                            self.matrix[i][j].pos_sols[num - 1] = true;
                        }
                    }
                    checking_range.push(Reverse((pos_sols, i, j)));
                }
            }
        }

        if break_condition {
            println!("sudoku is solved");
            return true;
        }

        let first = checking_range.pop();
        let minimum_loc: (usize, usize) = (first.unwrap().0 .1, first.unwrap().0 .2);
        for i in 0..10 {
            if self.check_matrix(minimum_loc.0, minimum_loc.1, i as u8) {
                self.place(minimum_loc.0, minimum_loc.1, i as u8);
                self.print_matrix().unwrap();
                if self.solver() {
                    return true;
                }
                self.num_calls += 1;
                self.place(minimum_loc.0, minimum_loc.1, 0 as u8);
            }
        }
        return false;
    }

    fn solve(&mut self) -> bool {
        self.setup();
        self.solver()
    }

    fn print_matrix(&mut self) -> io::Result<()> {
        let mut i = 0;
        let mut fmt = String::new();
        for row in &self.matrix[..] {
            i += 1;
            if i % 3 == 1 && i > 3 {
                //fmt.push_str("-----------------------");
                fmt.push('\n')
            }
            for (j, tile) in row.iter().enumerate() {
                if j % 3 == 0 && j > 0 {
                    fmt.push_str("║ ║");
                }
                match tile.num {
                    0 => fmt.push_str(" █ "),
                    n => fmt.push_str(&format!(" {} ", n)),
                }
                if j % 3 < 2 {
                    fmt.push_str("┃");
                } else {
                    //fmt.push('');
                }
            }
            fmt.push_str("\n━━━┛━━━┛━━━┛ ┗━━━┛━━━┛━━━┛ ┗━━━┛━━━┛━━━\n");
        }
        fmt.push_str("============================\n");
        self.printer
            .queue(cursor::MoveTo(0, 10))?
            .queue(style::Print(fmt))?;
        self.printer.flush()?;
        thread::sleep(Duration::from_millis(40));
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdout = io::stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    let solver = SudokuSolver::<9>::new(stdout);
    let mut solver = solver.read_files(&args[1]);
    solver.print_matrix().unwrap();
    if solver.solve() {
        println!("took {} calls", solver.num_calls);
        solver.print_matrix().unwrap();
    }
}
