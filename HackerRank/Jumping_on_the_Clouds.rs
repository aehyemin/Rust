use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn jumpingOnClouds(c: &[i32]) -> i32 {
    let mut step = 0;
    let mut loc = 0;
     println!("{}", c.len());
        while loc !=  c.len()-1 { 
            match c.get(loc+2) {
                
                Some(1) => {loc+=1 ; step+=1},
                Some(0) => {loc+=2 ; step+=1},
                _ => (),
                
            }
            println!("{}", loc);
        }   
        step
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let c: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = jumpingOnClouds(&c);

    writeln!(&mut fptr, "{}", result).ok();
}
