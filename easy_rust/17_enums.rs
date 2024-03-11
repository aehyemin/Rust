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

// fn main() {
//     let time = 8;
//     let sky_state = create_skystate(time);
//     check_skystate(&sky_state);
// }
// print : I can see the stars

fn main() {
    check_skystate(&create_skystate(20));
}
// 로 써줄수도 있다


