// fn main() {
//     let first_letter = 'A';
//     let space = ' '; // A space inside ' ' is also a char
//     let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
//     let cat_face = '😺'; // Emojis are chars too
// }

// let my_number = 8; // i32
// let second_number:u8 = 10;
// let third_number = my_number + second_number as u16;


// fn main() {
//     println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
//     println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
//     println!("Size of string containing 'ß': {}", "ß".len());
//     println!("Size of string containing '国': {}", "国".len());
//     println!("Size of string containing '𓅱': {}", "𓅱".len());
// }

use std::mem::size_of;
fn main() {
    println!("Size of a char: {}", size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}

