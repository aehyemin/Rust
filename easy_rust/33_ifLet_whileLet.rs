/* 
fn main() {
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0);
    let get_two = my_vec.get(10);

    println!("{:?}, {:?}", get_one, get_two);
    //  println!("{get_one:?}, {get_two:?}");
}
*/

/* 
fn main() {
    let my_vec = vec![2, 3, 4];

    // if let
    for index in 0..10 {
        if let Some(number) = my_vec.get(index){ //비교문이아니고 패턴매칭?
            println!("The number is : {}", number);
        }

        // match my_vec.get(index){
        //     Some(number) => println!("The number is : {}", number),
        //     None => {}
        // }
    }
}
*/

fn main() {
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {} : ", city.get[0]);
        while let Some(information) = city.pop() { //push <-> pop
            if let Ok(number) = information.parse::<i32>() // turbofish
                {println!("The number is : {}", number);}
        }
    }
}
