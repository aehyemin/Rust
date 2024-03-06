//struct = and
// enum = or enumeration(열거) 

// pascal case
enum ThingsInTheSky {
    Sun, // 0
    Stars, // 1

}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("l can see the stars"),
    }
}

fn main() {
    let time = 8;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state);
}
// print : I can see the stars

fn main() {
    check_skystate(&create_skystate(20));
}
// 로 써줄수도 있다


///////////////////////enums1///////////////////////////


enum Mood {
    Happy, 
    Sleepy,
    NotBad,
    Angry
}

fn match_mood(mood: &Mood) -> i32 {

    let happiness_level = match mood {
        Mood::Happy => 10,
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2
    };
    happiness_level
}

fn main (){
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);
}

// prints : Out of 1 to 10, my happiness is 7


enum Mood {
    Happy, 
    Sleepy,
    NotBad,
    Angry
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*; // *는 모든것 포함하겠다

    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2
    }
}

fn main (){
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);
}

// prints : Out of 1 to 10, my happiness is 7



enum Season {
    Spring, // 0
    Summer,
    Autumm,
    Winter
}

fn main(){
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumm, Winter]; // vec<season>
    println!("The number is : {}", season as u32);
}

/*
The number is : Spring
The number is : Summer
The number is : Autumm
The number is : Winter
 */


///////////////////////enums2///////////////////////////


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

///////////////////////////////////enums3/////////////////////////////////////////