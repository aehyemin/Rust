// The question mark operator ?

use std::num::ParseIntError;
// gives the error to the caller
fn parse_str(input:&str) -> Result<(), ParseIntError> {
    let parsed_number = input.parse::<i32>()?; // return Error
    println!("It worked {}", parsed_number);
    Ok(())
}
fn main(){
    for item in vec!["Seven", "8", "9.0", "nice"]{
       parse_str(item);
    }
}