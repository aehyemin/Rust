// generics 일반적인 <-> concrete 특정된 i32, String

// fn give_thing<GenericType>(input: GenericType)  -> GenericType{
//     input
// }

/*
fn give_thing<T>(input : T) -> T { // T
    input
}


fn main(){
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    println!("{}", x);
    println!("{}", y);

}
*/

struct Book;

use std::fmt::Display;
fn give_thing<T: Display >(input : T) -> T { // T
    println!("{}", input); // Display
    input
}


fn main(){
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    let z = give_thing(Book);
    println!("{}", x);
    println!("{}", y);

}


/* 
error: expected `where`, `{`, `(`, or `;` after struct name, found keyword `use`
 --> src/main.rs:3:1
  |
3 | use std::fmt::Display
  | ^^^ expected `where`, `{`, `(`, or `;` after struct name

error: could not compile `playground` (bin "playground") due to 1 previous error
 */