fn main (){
    let my_number = 100; //i32
    let my_number:u8 = 100 //255
    let my_other_number = 200;
    let third_number = my_number + my_other_number;
    // type inference
//다른 타입끼리는 더할 수 없음 mismatched type error


fn main() {
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too
}

let my_number = 8; // i32
let second_number:u8 = 10;
let third_number = my_number + second_number as u16;


fn main() {

    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}

use std::mem::size_of;
fn main() {
    println!("Size of a char: {}", size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}


fn main() {
    let number = 0________u8;
    let number2 = 1___6______2____4______i32;
    println!("{}, {}", number, number2);
}


fn main() {
    let my_float = 5.; // Rust sees . and knows that it is a float
}


fn main() {
    let my_number = 9.6; //f64
    let other_number = 9; //i32 
    println!("{}", my_number + other_number as f64);
}