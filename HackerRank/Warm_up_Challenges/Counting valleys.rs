use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut height = 0;
    let mut num_of_valley = 0;
    for step in path.chars(){
    //    match step {
    //        'D' => height-=1,
    //        'U' => {height+=1; 
    //              if height==0
    //                  {num_of_valley+=1;}}
    //         _ => () //do nothing ,
            println!("{step}")
        // }
    }
    // num_of_valley
    1
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}
