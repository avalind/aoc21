use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let mut last: i32 = -1;
    let mut n_inc: i32 = 0;
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ip) = line {
                let t = match ip.parse::<i32>() {
                    Ok(i) => i,
                    Err(_e) => -1
                };
                if last > -1 {
                    if t > last {
                        n_inc += 1;
                    }
                    last = t;
                } else {
                    last = t;
                    continue;
                }
            }
        }
    }
    println!("{}", n_inc);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
