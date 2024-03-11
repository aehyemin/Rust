/* 
fn main() {
    let my_number = 10;
    my_number = 9;
}

fn main() {
    let mut my_number = 10;
    my_number = 9;
}


fn main(){
    let my_variable = 10;
    let my_variable = "my variable";
    println!("{}", my_variable);
}


fn double(input:i32) -> i32{
    input * 2
 }
 
 fn triple(input:i32) -> i32{
    input * 3
 }
 
 fn main(){
     let x = 9;
     let x = double(x);
     let x = triple(x);
     println!("{}", x);
 }
*/

fn main(){ 
    let my_valuable = 9;
    println!("{}", my_variable);
    {
        let my_variable = "some string";
        println!("{}", my_variable);
    }
    println!("{}", my_variable);
}
/* 
prints :
9
some string
9
*/