// blanket trait implementations
//implementing a trait for every type that you want to have it
/* 
trait Prints {

    fn prints_something(&self) {
        println!("I like to print things");
    }
}

struct Person;
struct Building;

//impl Prints for Person {}
impl<T> Prints for T {

}

fn main() {
    let my_person = Person;
    let my_building = Building;
    my_person.prints_something();
    let x = String::from("Hello");
    x.prints_something();
    }


    trait Prints {

    fn prints_something(&self) {
        println!("I like to print things");
    }
}

struct Person;
struct Building;

//impl Prints for Person {}
impl<T> Prints for T {

}

fn main() {
    let my_person = Person;
    let my_building = Building;
    my_person.prints_something();
    let x = String::from("Hello");
    x.prints_something();
}
    */

use std::fmt::{Debug, Display};

trait Prints {
    fn Debug_print(&self) where Self: Debug {
            println!("I am : {:?}", self);
        }
    
    fn display_print(&self) where Self:Display {
        println!("I am : {}", self);
    }
}

#[derive(Debug)]
struct Person;
#[derive(Debug)]
struct Building;
    
//impl Prints for Person {}
impl<T:Debug> Prints for T {}



    
fn main() {
    let my_person = Person;
    let my_building = Building;
    my_person.Debug_print();
    // let x = String::from("Hello");
    // x.Debug_print();
    let my_string = String::from("Hello there");
    my_string.Debug_print();
    my_string.display_print();
}