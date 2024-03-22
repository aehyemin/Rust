use std::collections::HashMap;


/* 
fn main(){ // key -> Value
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }    
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }
// may cause a panic if mistyped
    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeklklklklklkld")); 
}

fn main() {
    let mut book_hashmap = HashMap::new();
    
    book_hashmap.insert(1, "lanamdf");
    book_hashmap.insert(1, "성인군주");
    book_hashmap.insert(1, "babo");
// insert 는 덮어쓰기
    println!("{:?}", book_hashmap.get(&1));
    
}



fn main() {
    let mut book_hashmap = HashMap::new();
    
    book_hashmap.insert(1, "lanamdf");

    if book_hashmap.get(&1).is_none() {
        book_hashmap.insert(1, "성인군주");
    } else {
        println!("Already got a book.");
    }
    
}

*/
fn main() {
    let mut book_hashmap = HashMap::new();
    
    book_hashmap.insert(1, "lanamdf");

    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Already got a book : {}", book_name);
    } else {
        book_hashmap.insert(1, "성인군주");
    }
    
}
