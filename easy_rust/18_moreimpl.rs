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
    fn new_old_cat() -> Self{
        Self{
            age:15,
            animal_type:AnimalType::Cat
        }
    }
}

impl Animal {
    fn new_cat(age: u8) -> Self { //new 는 특별한거아님fuction signature
        Self {
            age,
            animal_type : AnimalType::Cat

        }
    }

    fn new_dog(age:u8) -> Self{
        Self {
            age,
            animal_type: AnimalType::Dog,
        }
    }
    fn print(&self) {
        println!("I am a : {:?}", self);
    }

    fn change_to_dog(&mut self){
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog now i am : {:?}", self);
    }

    fn change_to_cat(&mut self){
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat now i am : {:?}", self);
    }

}


fn main(){
    let mut my_animal = Animal::new_cat(10); //associated function
    my_animal.print(); // syntactic sugar
   // Animal::print(&my_animal); 
   my_animal.change_to_cat();
   my_animal.change_to_dog();

   let my_old_cat = Animal::new_old_cat();
}

