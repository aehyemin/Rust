/* 
fn check_if_five(number : i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string())
    }
}

fn main() {
    let mut result_vec = Vec::new(); // Vec<Result <i32,String> >

    for number in 2..=7 {
        result_vec.push(check_if_five(number));
    }
    println!("{:#?}", result_vec);
}
*/

use std::num::ParseIntError;

// anyhow - crate

fn parse_number(number : &str) -> Result<i32, ParseIntError> {
    number.parse() 
      //  ^^^^^ expected `ParseIntError`, found `String`
}


fn main() {
    let mut result_vec = vec![];
    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("sdsdsd"));
    result_vec.push(parse_number("8"));

    for number in result_vec {
        println!("{:?}", number)
    }
// eprintln!("{}")
}


