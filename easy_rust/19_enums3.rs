/* 
enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiamt = 1000,
    DeadStar
}

fn main(){
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiamt, DeadStar];
    
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star : {}", size),
            size if size >= 80 => println!("Pretty big star :{}", size),
            _ => println!("Some other star"),

        }
    }
    println!("What about Deadstar? It is : {}", DeadStar as u32);
}


/* prints: 
Not the biggest star : 10
Not the biggest star : 50
Pretty big star :100
Pretty big star :1000
Pretty big star :1001
What about Deadstar? It is : 1001 */

*/


enum Number {
    Positive(u32),
    CouldBeNagative(i32)
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::Positive(input as u32),
        false => Number::CouldBeNagative(input)
    }
 
}

fn main(){
    let my_vec = vec![get_number(-800), get_number(8)]; // Vec<Number>

    for item in my_vec {
        match item {
            Number::Positive(number) => println!("It's a u32 with the {}", number),
            Number::CouldBeNagative(number) => println!("It's a i32 with the value {}", number),
        }
    }
}