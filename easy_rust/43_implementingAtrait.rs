use std::fmt;

#[derive(Debug)]
struct Cat {
    name : String,
    age : u8
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let age = self.age;
        write!(f, "My cat's name is {name} and it it {age} years old.", )
    }
}


fn main(){
    let mr_mantle = Cat {
        name : "Reggie Mantle".to_string(),
        age : 4
    };
    println!("{mr_mantle}");
    //보라색은 트레잇 빨간색은스트럭
}
