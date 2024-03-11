// impl blocks

#[derive(Debug)]
struct Animal {
    age:u8,
    animal_type : AnimalType
}


#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}

impl Animal {
    fn new_cat(age: u8) -> Self { //new 는 특별한거아님fuction signature
        Self {
            age,
            animal_type : AnimalType::Cat

        }
    }
}

fn main(){
    let my_animal = Animal::new_cat(10); //associated function
    println!("I made a : {:?}", my_animal);
}







