use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::cmp;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point{x, y}
    }

    fn from_str(line: String) -> Point {
        let parts: Vec<usize> = line
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        if parts.len() != 2 {
            panic!("Point::from_str: malformed input!");
        }
        Point{x: parts[0], y: parts[1]}
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line{start, end}
    }

    fn new_coords(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        let start = Point::new(x1, y1);
        let end = Point::new(x2, y2);
        Line{start, end}
    }

    fn from_str(line: String) -> Line {
        let cords: Vec<String> = line
            .split("->")
            .map(|x| String::from(x.trim()))
            .collect();
        if cords.len() == 2 {
            Line{
                start: Point::from_str(cords[0].clone()),
                end: Point::from_str(cords[1].clone()),
            }
        } else {
            panic!("Line::from_str: malformed input!");
        }
    }

    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn diagonal(&self) -> bool {
        !self.horizontal() && !self.vertical()
    }
}

#[derive(Debug, Clone)]
struct VentMap {
    buf: Vec<u8>,
    width: usize,
    height: usize,
}

impl VentMap {
    fn new(width: usize, height: usize) -> VentMap {
        let v = VentMap{
            buf: vec![0_u8; width*height],
            width: width,
            height: height};
        v
    }

    fn set(&mut self, val: u8, x: usize, y: usize) {
        let idx = self.width*y + x;
        if idx > self.width * self.height {
            panic!("Trying to set outside Map!");
        }
        self.buf[idx] = val;
    }

    fn get(&mut self, x: usize, y: usize) -> u8 {
        let idx = self.width*y + x;
        if idx > self.width * self.height {
            panic!("Trying to get outside Map!");
        }
        self.buf[idx]
    }

    fn add_one(&mut self, x: usize, y: usize) {
        let val = self.get(x,y) + 1;    
        self.set(val, x, y)
    }

    fn place_line(&mut self, mut line: Line) {
        println!("{:?}", line);
        if line.horizontal() {
            // we have a horizontal line
            if line.end.x < line.start.x {
                let t = line.start;
                line.start = line.end;
                line.end = t;
            }
            for xt in line.start.x..line.end.x+1 {
                self.add_one(xt, line.start.y);
            }
        }
        else if line.vertical() {
            if line.end.y < line.start.y {
                let t = line.start;
                line.start = line.end;
                line.end = t;
            }
            for yt in line.start.y..line.end.y+1 {
                self.add_one(line.start.x, yt); 
            }
        }
        else if line.diagonal() {
            println!("LEL");
        }
    }

    fn n_over_n(&mut self, lim: u8) -> usize {
        self.buf.iter().cloned().filter(|val| val >= &lim).count()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let mut lines_buf: Vec<Line> = Vec::new();
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ip) = line {
                lines_buf.push(Line::from_str(ip));
            }
        }
    }

    // need to remove all lines that are not horizontal or vertical
    lines_buf = lines_buf
        .iter()
        .filter(|line| line.horizontal() || line.vertical())
        .cloned()
        .collect::<Vec<Line>>();

    // find the dimensions of the VentMap
    let mut limits = Point{x: 0, y: 0};
    for line in &lines_buf {
        if line.horizontal() {
            let m = cmp::max(line.start.x, line.end.x);
            if m > limits.x {
                limits.x = m;
            }
        }

        if line.vertical() {
            let m = cmp::max(line.start.y, line.end.y);
            if m > limits.y {
                limits.y = m;
            }
        }
    }

    limits.x += 1;
    limits.y += 1;

    let mut vmap = VentMap::new(limits.x, limits.y);
    for line in &lines_buf {
        vmap.place_line(*line)       
    }
    println!("{}", vmap.n_over_n(2));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
