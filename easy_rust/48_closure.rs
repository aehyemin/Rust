// closure = anonymous functions that capture their enviroment
// a|nonymous = no name 
// closure = enclose 

// || - pipes
// ()

/*
fn do_it(input: i32){
}

fn main() {
    do_it(8);
}
 */


//  fn main(){
//     let my_closure = |x:i32|println!("This is a closure");
//     my_closure();
//  }
// error[E0057]: this function takes 1 argument but 0 arguments were supplied

/*
fn main(){
    let my_number = 10;
    let my_closure = |x:i32|println!("{}", x + my_number);
    my_closure(9);
 }
 */

/* 
 .iter().map(|item| {
    let my_number = 7;
    item + my_closure
    }).
    collect()
*/

fn main(){
    let my_closure = ||  {
        let my_number = 7;
        let other_number = 10;
        println!("The two numbers are {my_number} and {other_number}");
        my_number + other_number // return
    };
    let my_var = my_closure();
    println!("{my_var}");
}