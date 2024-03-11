/* 
// () - empty type, unit type(void)

fn number() -> i32 {
    8 //;
}

fn main() {
    let my_number = number();
}


// () - empty type, unit type(void)
fn number() -> i32 {
    8;
}

fn main() {
    let my_number = number();
}
// error : mismatched types occur
*/




// expression-based language
fn empty_tuple() -> () { 
}

fn main() {
    let tuple = empty_tuple();
    println!("{:?}", tuple);
}

/* 
error : '()' doesnt implement 'std::fmt::display' 가 발생한다.
 display { }(깔끔하고 자기가 원하는 식)  가 아닌 
 debug print(프로그래밍 전용 프린트 느낌) 를 써야한다.  -> {:?} or {:#?}

함수의 구성은 fn 함수의이름 ( 입력값 : 타입 ) -> 리턴타입 {
       내용
     return 또는 return 리턴값
 }
 *\