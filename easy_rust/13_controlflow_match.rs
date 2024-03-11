// control flow 
//if else 나 loop 같은 거
/* 
fn main(){
    let my_number = 5;
    let my_second_number = 10;
    if my_number ==5 && my_second_number == 10 {
        println!("they both match");
    } else if my_number == 6 { // && and || or
        println!("It's six");
    } else {
        println!("It's a different number");
    }
}
// if는 괄호 없이 쓴다


fn main(){
    // match
    let my_number = 5;

    match my_number { // switch
        0 => println!("it's a zero"),
        1 => println!("It's a one"),
    }
}


fn main(){
    // match
    let my_number:u8 = 5;

    match my_number { // switch
        0 => println!("it's a zero",
        1 => println!("It's a one"),
        _ => println!("It's a different number") // _ "I don't care" "anything else"
    }
}
*/

fn main(){ // expression based language이기 때문에 let 을 이용할 수 있다
    // match
    let my_number:u8 = 5;

    let second_number = match my_number { 
        0 => 23,
        1 => 65,
        _ => 0 //"It's a different number" ,  _ "I don't care" "anything else"
    };
    println!("The second number is {}", second_number);

}
//expression based language이기 때문에 let 을 이용할 수 있다