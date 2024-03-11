// while loop
/* 
fn main() {
    let mut counter = 0;
    while counter != 5 {
        counter += 1;
        println!("The counter is : {}", counter);

    }
}



//for loop

fn main() {
    for number in 0..3 {  //exclusive range 0..3(0,1,2), inclusive range 0..=3(0,1,2,3)
        println!("The number is {}", number);
    }
}


fn main() {
    for _ in 0..3 {  //exclusive range 0..3(0,1,2), inclusive range 0..=3(0,1,2,3)
        println!("I don't care");
    }
}
/* prints:
I don't care
I don't care
I don't care
*/



fn main() {
    for _number in 0..3 {  //exclusive range 0..3(0,1,2), inclusive range 0..=3(0,1,2,3)
        println!("I don't care");
    }
}
// 나중에 사용하고 싶을때 _number

*/

fn main() { 
    let mut counter = 5;

    let my_number = loop {
        counter +=1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("My_number is now : {}", my_number);
}
// break 하면서 return 도 가능