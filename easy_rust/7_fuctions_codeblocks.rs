fn give_numver(one: i32, two: i32) -> i32 {
    let multiplied = one * two ;
    println!("{}", multiplied);
 }
 
 fn main(){
     let my_number = give_numver(9,8);
     println!("{}", my_number);
 }
 



fn give_numver(one: i32, two: i32) -> i32 {
   let multiplied_by_ten = {
    let first_number = 10;
    first_number * one * two
   };
    multiplied_by_ten
}
// fn 을 자세하게 쓸수도 있다
fn main(){
    let my_number = give_numver(9, 1);
    println("{}", my_number);

}