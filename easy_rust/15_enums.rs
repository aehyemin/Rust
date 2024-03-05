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


///////////////////////enums2///////////////////////////


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
