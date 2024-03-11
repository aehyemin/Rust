/* 
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

*/

enum Season {
    Spring, // 0
    Summer,
    Autumm,
    Winter
}

fn main(){
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumm, Winter]; // vec<season>
    for season in four_seasons {
    println!("The number is : {}", season as u32);
    }
}

/*
The number is : 0
The number is : 1
The number is : 2
The number is : 3
 */