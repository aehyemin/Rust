fn main () {
    let sky = "cloudy";
    let temperature = "warm"

    match(sky, temperature){
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _ ) => println!("Cloudy and something else"),
         _ => println!("Not sure what the weather is."),
    }
}

/* 
_ => println!("Not sure what the weather is." 을 맨 윗줄에 쓰면
warning : unreachable patterns 이 발생한다. 
_ 를 맨 앞에 쓰면 뒷줄을 읽지 않기 때문.
*/


fn main() {
    let children = 5;
    let married = true:

    match(children, married) {
       (children,married) if married == false => println!("Not married with {} children", children),
       (c,m) if c == 0 && m => println!("Married but with no children"),
       _ => println!("Some other type of marriage and children combination")
    }
}
// prints : Some other type of marriage and children combination



fn match_colors(rag: (u32, u32, u32)){
    match rbg {
        (r, _, _) if r< 10 => println!("Not much red"), 
        (_, b, _) if b< 10 => println!("Not much blue"), 
        (_, _, g) if g< 10 => println!("Not much green"), 
        _ => println!("Every color has at least 10")
       }
  }

fn main(){
     let first = (200, 0, 0);
     let second = (50, 50, 50);
     let third = (200, 50, 0);  
 
     match_colors(first);
     match_colors(second);
     match_colors(third);

}
/* 
prints :
Not much blue
Every color has at least 10
Not much green
*/


fn main(){
    let my_number = 10;
    let some_variable = match my_number {
        10 => 8,
        _ => "not ten"
    };
}
// error : match arms have incompatible types

//타입을 일치시켜 주어야 한다. 윗줄이 integer 이면 아랫줄도 integer



fn match_number(input:i32) {
     match input {
      //  number => println!("It's the number {}", number) 로 쓰면 unreachable pattern error
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {}", number),

        _ => println!("It's greater than ten"),
     }


    fn main(){
        match_number(10);
        match_number(100);
    }
        }


// prints:
// It's between 0 and 10. It's the number 10
// It's greater than ten
// @ 로 조건부 변수를 지정해 줄 수 있음


