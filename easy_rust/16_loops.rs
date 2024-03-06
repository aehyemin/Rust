// loops

fn main() {
    let mut counter = 0;
    loop {
        counter +=1;
        println!("The counter is : {}", counter);
        if counter == 5 {
            break; // loop 에서 탈출할때
        } 
    }
}



fn main() {
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop");

    'first_loop: loop { // ' = tick 루프가 이름을 가지게함
    counter +=1;
    println!("The  counter is now: {}", counter);
    if counter > 9 {
        println!("Now entering the second loop");

    
        'second_loop : loop {
            println!("The second counter is : {}", counter2);
            counter2 +=1;
            if counter2 ==3 {
                break 'first_loop;
            }
        } 
    }
  }
}

//////////////////////////////more_loops/////////////////////
// while loop

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