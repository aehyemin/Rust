/* 
#[derive(Debug)]
struct Animal {
    age:u8,
    animal_type : AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl AnimalType {
    fn check_type(&self) {
        use AnimalType::*;

        match self {
        Cat => println!("Animal type is cat."),
        Dog => println!("Animal type is Dog." ),
        }
    }
}

impl Animal {
    fn new(age: u8, animal_type: AnimalType) -> Self { //new 는 특별한거아님fuction signature
        Self {age, animal_type}
    }

    fn change_to_dog(&mut self){
        self.animal_type = AnimalType::Dog;
        println!("Change to dog! Now I am {:?}", self);
    }

    fn change_to_cat(&mut self){
        self.animal_type = AnimalType::Cat;
        println!("Change to Cat! Now I am {:?}", self);
    }
}       


fn main(){
    use AnimalType::*;

    let my_dog = Animal::new(10, Dog);
    let my_cat = Animal::new(10, Cat);

    my_cat.animal_type.check_type();
}
*/






// impl blocks
#[derive(Debug)]
struct Animal {
    age:u8,
    animal_type : AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat(String),
    Dog(String),
}

impl AnimalType {
    fn print_name(&self){
        match self {
            AnimalType::Cat(name) => println!("Animal type is cat and name is : {}.", name ),
            AnimalType::Dog(name) => println!("Animal type is Dog and name is : {}.", name ),
        }
    }
}

impl Animal {
    fn new(age: u8, animal_type: AnimalType) -> Self { //new 는 특별한거아님fuction signature
        Self {age, animal_type}
    }
}       


fn main(){
    let my_cat = Animal::new(10, AnimalType::Cat("windy".to_string()));
    my_cat.animal_type.print_name();
}