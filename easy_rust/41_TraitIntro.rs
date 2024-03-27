// traits
// power superpower 
// struct User // thing
// enum Months //  choices
// trait // verbs 어떤행동하는지?


use std::fmt::Debug;
/* 
#[derive(Debug)]
struct MyStruct {
    number: usize
}

fn print_as_debug<T>(input: T) 
where
     T: Debug,
{
    println!("{input:?}");

}
fn main () {
}
*/


#[derive(Clone, Copy, Debug)]
struct ThingsToAdd{
    first_thing: u32,
    second_thing: f32
}

fn main(){
    let my_thing = ThingsToAdd{
        first_thing: 32,
        second_thing: 8.8
    };

    let second_thing = ThingsToAdd{
        first_thing: 32,
        second_thing: 8.8
    };
    // must implement 'Add' trait
   // let sum = my_thing + second_thing;
}


