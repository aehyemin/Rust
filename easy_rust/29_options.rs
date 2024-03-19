// option
// Result
// OCaml

// 러스트는 null 에러가 없다?

// enum Option<T> {
//     None,
//     Some(T),
// }

// use std::Option:*;
// Rust prelude 제일편리한거 

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) // i32
    }
    
}

// wrap in an Option

fn main (){
    let new_vec = vec![1, 2, 4, 7, 8, 10, 10];
    let index = take_fifth(new_vec);
    println!("{:?}", index);

}