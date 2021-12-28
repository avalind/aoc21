use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;


fn buf2mat(raw: Vec<String>) -> isize {
    let mut cidx = 0;
    let mut most_common = Vec::<char>::new();
    let mut least_common = Vec::<char>::new();

    while cidx < raw[0].len() {
        let mut num_ones = 0;
        let mut ridx = 0;
        // for column cidx, iterate over all rows
        // summing the ones with '1' at this position
        while ridx < raw.len() {
            let ch = raw[ridx].chars().nth(cidx).unwrap();
            if ch == '1' {
                num_ones += 1;
            }
            ridx += 1;
        }
        let mut delta = raw.len() - num_ones;
        if num_ones > delta {
            most_common.push('1');
            least_common.push('0');
        } else {
            least_common.push('1');
            most_common.push('0');
        }
        cidx += 1;         
    }

    let mcn: String = most_common.into_iter().collect();
    println!("{}", mcn);
    let i = isize::from_str_radix(&mcn, 2).unwrap();
    let lcn: String = least_common.into_iter().collect();
    let j = isize::from_str_radix(&lcn, 2).unwrap();
    return i*j;
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    if let Ok(lines) = read_lines(fname) {
        let raw: Vec<String> = lines.into_iter().map(|x| x.unwrap()).collect();
        let res = buf2mat(raw);
        println!("{}", res);
                //let intval = isize::from_str_radix(&ip, 2).unwrap();
                //println!("{}", intval);
            //}
        //}
        //let a: u8 = 'a';
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
