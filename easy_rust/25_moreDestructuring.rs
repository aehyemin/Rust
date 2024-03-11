struct Person {
    name: String,
    real_name : String,
    height : u8,
    happiness : bool
}
#[derive(Debug)]
struct Person2 {
    name: String,
    height : u8
}


impl Person2 {
    fn from_person(input: Person) -> Self {
        let Person {name, height, .. } = input; // .. = 나머지는필요없다
        
        Self {
            name,
            height
        }
    }
}

fn main(){
    let papa_doc  = Person {
        name : "Papa Doc".to_string(),
        real_name : "Clearence".to_string(),
        height: 170,
        happiness : false
    };

    /* 
    let Person {
        name : a,
        real_name: b ,
        height: c,
        happiness: d
    } = papa_doc;
    */

    let person2 = Person2::from_person(papa_doc);

    println!("Person2 type is {:?}", person2);

    /*
    println!("They call him {} but his real name is {}. He is {} cm tall and is he happy?{}",
    a, b, c, d
    name, real_name, height, happiness
    papa_doc.name, papa_doc.real_name, papa_doc.height, papa_doc.happiness
    );
    */
}



