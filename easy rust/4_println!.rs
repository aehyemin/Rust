// macro 는 코드 작성을 도와주는 코드이다.
// macro = fuction that writes code


fn main() {
    println!("hello, world");
}
// prints : hello, world



fn main() {
    let my_name = "David";
    let my_age = 25;
    println!("my name is {} and my age is {}", my_name, my_age);
}
// prints : my name is David and my age is 25




fn give_age() -> i32 {
    25
//rust 에서는 return을 사용할 필요가 없다
}
fn main() {
    let my_name = "David";
    println!("my name is {} and my age is {}", my_name, give_age());
}
// prints :my name is David and my age is 25




fn give_age() -> i32 {
    25
//rust 에서는 return을 사용할 필요가 없다
}

fn main() {
    let my_name = "David";
    let my_age = 25;
    println!("my name is {my_name} and my age is {my_age}");
}

// prints : my name is David and my age is 25

