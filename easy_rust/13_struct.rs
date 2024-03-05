// struct = and , enum = or 비슷하지만 많이 다름
// unit struct ( 거의 안 씀), tuple struct, named struct ( 가장 많이 씀, struct 다운 struct)

// unit struct
struct FileDirectory;

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}
fn main(){
    let x = FileDirectory;
    takes_file_directory(x)
}
//prints : I got a file directory


// unit struct
struct FileDirectory;

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}
fn main(){
    let x = FileDirectory;
    println!("The size is {}",  std::mem::size_of_val(&x));
}
// prints : The size is 0



//tuple struct
// trait
struct color(u8, u8, u8);

fn main(){
    let my_color = color(20, 50, 100);
    println!("The seconf color is {}", my_color.1);
}
// prints: The second color is 50


//tuple struct
// trait
#[derive(Debug)] // attribute

struct color(u8, u8, u8);

fn main(){
    let my_color = color(20, 50, 100);
    println!("The seconf color is {}", my_color);
}
// prints : The second color is color(20, 50, 100)


// named struct
struct Country {
    population : u32,
    capital : String,
    leader_name : String
}

fn main() {
    let canada = Country {
        population : 35_000_000,
        capital : "Ottawa".to_string(),
        leader_name : "Justin".to_string()
       };
    println!("The population is : {} \n The capital is {}", cananda.population);
}

/*
prints :
The population is : 35000000
The capital is Ottawa
*/


// named struct
#[derive(Debug)]
struct Country {
    population : u32,
    capital : String,
    leader_name : String
}

fn main() {
    let canada = Country {
        population : 35_000_000,
        capital : "Ottawa".to_string(),
        leader_name : "Justin".to_string()
    };
    println!("The country is : {:?}", cananda);
}
The country is : Country { population: 35000000, capital: "Ottawa", leader_name: "Justin" };


/* 
{:#?} 는 pretty print 인데 좀 더 깔끔하게 프린트 가능
The country is : Country {
population: 35000000, 
capital: "Ottawa", 
leader_name: "Justin" 
*/



