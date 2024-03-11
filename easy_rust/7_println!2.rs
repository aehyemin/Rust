/* 
fn main() {
    let my_city = "seoul";
    let year = 2024;
    let population = 9_434_353; //_를 쓰면 구분이 쉽다
    println!("the city of {} in {} had a population of {}", my_city, year, population);
}
// print : the city of seoul in 2024 had a population of 9434353
*/

fn main() {
    let my_city = "seoul";
    let year = 2024;
    let population = 9_434_353; //_를 쓰면 구분이 쉽다
    println!(
    "the city of {0} in {1} had a population of {2}",
    my_city, year, population);
}
// print : the city of seoul in 2024 had a population of 9434353