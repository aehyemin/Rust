/* 
#[derive(Debug)]
struct Country {
    population : u32,
    capital : String,
    leader_name : String
}


fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justion".to_string();

    let my_country = Country {
        population : population // 굳이 두번 안쓰고 population 한번만 쓰면됨
        capital : capital
        leader_name : leader_name
     };
}
        
// 러스트에서는 굳이 두 번씩 안 쓰고 이름이 같다면 아래와 같이 바꿔준다


#[derive(Debug)]
struct Country {
    population : u32,
    capital : String,
    leader_name : String
}


fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justion".to_string();

    let my_country = Country {
        population, 
        capital,
        leader_name
     };
}


use std::mem::size_of_val;
#[derive(Debug)]
struct Country {
    population : u32,
    capital : String,
    leader_name : String
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justion".to_string();

    let my_country = Country {
        population, 
        capital,
        leader_name
     };
    println!("Country is {} bytes in size", size_of_val(&my_country))
}
prints : 

Country is 56 bytes in size

​

​

​

the alignment 

​

use std::mem::size_of_val;

struct Numbers {
    one:u8,
    two:u8,
    three:u8,
    four:u32 // u32 는 바이트 수가 4
}

#[derive(Debug)]
struct Country {
    population : u32,
    capital : String,
    leader_name : String
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justion".to_string();

    let my_country = Country {
        population, 
        capital,
        leader_name
     };
    println!("Country is {} bytes in size", size_of_val(&my_country));

    let numbers = Numbers {
        one:8,
        two:19,
        three:20,
        four:30,
    };
         println!("Size is : {}", size_of_val(&numbers))
}
/* 
prints :
Country is 56 bytes in size
Size is : 8
*/
*/

use std::mem::size_of_val;

#[derive(Debug)]
struct Country {
    population : u32,
    capital : String,
    leader_name : String,
    all_population: [u32; 5500]
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justion".to_string();

    let my_country = Country {
        population, 
        capital,
        leader_name,
        all_population: [500; 5500]
     };
      println!("Country is {} bytes in size", size_of_val(&my_country));
}
// prints : Country is 22056 bytes in size
 // all_population: [u32; 5500]처럼 사이즈가 너무 다르면 다른 struct로 옮겨서 쓰는 게 좋을 수도 있다.
