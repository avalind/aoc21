use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(Default, Copy, Clone, Debug)]
struct BoardCell {
    value: i32,
    drawn: bool,
}

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<BoardCell>,
    width: usize,
    height: usize,
}

impl BoardCell {
    fn new(value: i32, drawn: bool) -> BoardCell {
        BoardCell{value, drawn}
    }

    fn mark(&mut self)  {
        self.drawn = true;
    }
}

/*
impl Clone for Board {
    fn clone(&self) -> Board {
        let mut n = Board::new(self.width, self.height);
        n.cells = 
    }
}*/

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let b = Board{
            cells: vec![BoardCell::default(); width*height],
            width: width,
            height: height,
        };
        b
    }

    fn get(&self, x: usize, y: usize) -> BoardCell {
        let idx = self.width * y + x;
        if idx > self.width * self.height {
            panic!("trying to access element outside of board, aborting.");
        }
        self.cells[idx]
    }

    fn set(&mut self, x: usize, y: usize, b: BoardCell) {
        let idx = self.width * y + x;
        if idx > self.width * self.height {
            panic!("trying to access element outside of board, aborting.");
        }
        self.cells[idx] = b;
    }

    fn mark(&mut self, x: usize, y: usize) {
        let idx = self.width * y + x;
        if idx >= self.width * self.height {
            panic!("trying to access element outside of board, aborting.");
        }
        self.cells[idx].mark();
    }

    fn mark_1d(&mut self, idx: usize) {
        self.cells[idx].mark()
    }

    fn has_value(&mut self, val: i32) -> Option<usize> {
        self.cells.iter().position(|x| x.value == val)
    }

    fn winning_column(b: &Board, cidx: usize) -> Option<usize> {
        for i in 0..b.height {
            if !b.get(cidx, i).drawn {
                return None;
            }
        }
        return Some(cidx);
    }

    fn winning_row(b: &Board, ridx: usize) -> Option<usize> {
        for i in 0..b.width {
            if !b.get(i, ridx).drawn {
                return None;
            }   
        }
        return Some(ridx);
    }

    fn has_winning_column(&self) -> Option<usize> {
        for c in 0..self.width {
            if let Some(idx) = Board::winning_column(self, c) {
                return Some(idx);
            }
        }
        return None;
    }

    fn has_winning_row(&self) -> Option<usize> {
        for c in 0..self.height {
            if let Some(idx) = Board::winning_row(self, c) {
                return Some(idx);
            }
        }
        return None;
    }

    fn contains_win(&self) -> bool {
        if let Some(idx) = self.has_winning_row() {
            return true;
        }
        if let Some(idx) = self.has_winning_column() {
            return true;
        }
        return false;
    }

    fn count_marks(&self) -> usize {
        self.cells.iter().filter(|c| c.drawn).count()
    }

    fn from_buffer(buf: &Vec<String>, w: usize, h: usize) -> Board {
        let mut b = Board::new(w, h);
        for j in 0..h {
            let tmp: Vec<i32> = buf[j]
                .trim()
                .split(" ")
                .filter_map(|x| {
                    if let Ok(v) = x.parse::<i32>() {
                        Some(v)
                    } else {
                        None
                    }})
                .collect();
            for i in 0..w {
                b.set(i, j, BoardCell::new(tmp[i], false));
            }
        }
        return b;
    }

    fn count_unmarked(&self) -> i32 {
        self.cells.iter().filter(|c| !c.drawn).map(|d| d.value).sum()
    }
}

fn parse_draws(line: &str) -> Vec<i32> {
    line.split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

#[derive(PartialEq, Debug)]
enum ParserState {
    Inside,
    Outside,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let mut boards: Vec<Board> = Vec::new();
    let mut rowcounter = 0;
    let mut draws: Vec<i32> = Vec::new();
    let mut pstate = ParserState::Outside;
    if let Ok(lines) = read_lines(fname) {
        let mut boardbuf: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                // we now have a line of text to handle
                // first case: if rowcounter == 0 we handle all draws.
                if rowcounter == 0 {
                    draws = parse_draws(&ip);
                    rowcounter += 1;
                }
                else {
                    // we are currently looking at a row that is part of a board
                    if ip.len() == 0 {
                        // we are also outside a board block
                        if pstate == ParserState::Outside {
                            continue
                        } else if pstate == ParserState::Inside {
                            
                            // if we are inside a board and find that the next line is
                            // empty, we know that we have in our buffer a complete board.
                            boards.push(Board::from_buffer(&boardbuf, 5, 5));
                            boardbuf.clear();
                            pstate = ParserState::Outside;
                        }
                    } else {
                        if pstate == ParserState::Outside {
                            pstate = ParserState::Inside;
                            boardbuf.push(ip);
                        } else if pstate == ParserState::Inside {
                            boardbuf.push(ip);
                        }
                    }
                }
            }
        }
        // handle the edge case
        if pstate == ParserState::Inside {
            boards.push(Board::from_buffer(&boardbuf, 5, 5));
            boardbuf.clear();
            pstate = ParserState::Outside;
        }
    }

    let mut last_winner = Board::new(5,5);
    let mut last_winner_elem: i32 = 0;
    for i in 0..draws.len() {
        let elem = draws[i];
        for j in 0..boards.len() {
            if let Some(idx) = boards[j].has_value(draws[i]) {
                boards[j].mark_1d(idx);
            }
        }

        // @TODO this if fucked up!
        for (j, board) in boards.iter().enumerate() {
            println!("board {} has {} marks", j, board.count_marks());
            if let Some(row_idx) = board.has_winning_row() {
                last_winner = board.clone();
                last_winner_elem = draws[i];
            }

            if let Some(col_idx) = board.has_winning_column() {
                last_winner_elem = draws[i];
                last_winner = board.clone();
            }
        }

        boards = boards.into_iter().filter(|x| !x.contains_win()).collect();
    }
    println!("{:?}", last_winner);
    let score = last_winner.count_unmarked() * last_winner_elem;
    println!("{}", score);
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
