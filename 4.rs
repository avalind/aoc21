use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

enum Move {
    Forward(i32),
    Backward(i32),
    Up(i32),
    Down(i32),
}

fn parse_command(raw: &str) -> Move {
    let mut it = raw.split_ascii_whitespace();
    let dir = it.next();
    if let Some(magn) = it.next() {
        let f = magn.parse().unwrap();
        let m = match dir {
            Some("forward") => Move::Forward(f),
            Some("backward") => Move::Backward(f),
            Some("up") => Move::Up(f),
            Some("down") => Move::Down(f),
            _ => panic!("Bailing"),
        };
        return m;
    } else {
        panic!("Malformed input!");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let mut hor: i32 = 0;
    let mut ver: i32 = 0;
    let mut aim: i32 = 0;
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ip) = line {
                match parse_command(&ip) {
                    Move::Forward(f) => { hor += f; ver += aim * f; }
                    Move::Backward(f) => hor -= f,
                    Move::Up(f) => aim -= f,
                    Move::Down(f) => aim += f, 
                }
            }
        }
    }
    println!("{}",ver * hor);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
