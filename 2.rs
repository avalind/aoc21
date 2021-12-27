use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn running_sum(input: Vec<i32>, n: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::<i32>::new();
    let mut idx: i32 = 0;
    while idx <= (input.len() as i32)-n {
        let start = idx as usize;
        let end = start+(n as usize);
        let s: i32 = input[start..end].into_iter().sum();
        output.push(s);
        idx += 1;
    }
    return output;
}

fn count_increases(rsums: Vec<i32>) -> i32 {
    let mut n_inc: i32 = 0;
    let mut idx: usize = 1;
    while idx < rsums.len() {
        if rsums[idx] > rsums[idx-1] {
            n_inc += 1;
        }
        idx += 1;
    }
    return n_inc;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    if let Ok(lines) = read_lines(fname) {
        let measurements: Vec<i32> = lines.map(|x| {
            if let Ok(val) = x {
                val.parse::<i32>().unwrap()
            } else {
                panic!("malformed input!");
            }
        }).collect();

        let averages = running_sum(measurements, 3);
        println!("{:?}", count_increases(averages));
    }
    //let v = vec![199,200,208,210,200,207,240,269,260,263];
    //let rs = running_sum(v, 3);
    //println!("{:?}", rs); 
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
