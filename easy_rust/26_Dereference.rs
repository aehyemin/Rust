// references and the dot operator
/* 
fn main() {
    let my_number = 10; // i32
    let reference = &my_number; // &i32

    println!("Are they the same? {}", my_number == *reference);

}
*/
// reference 를 *reference 로 바꾸면 value 자체의 값을 비교해줌?
// LHS 와 RHS 
// error[E0277]: can't compare `{integer}` with `&{integer}`
// integer 를 왼쪽에 적었을때, 비교할때 오른쪽 rhs 에 변수를 넣으면 컴파일러가 타입을 추론해주는데 그런경우가 아님



struct Item {
    number:u8
}


// . dot operator
impl Item {
    fn compare_number (&self, other_number:u8){
        println!("are they equal {}", self.number == other_number)
    }
}

// Deref *
fn main (){
    let item = Item {
        number: 10
    };

    let reference_item = &item; // &u8
    let other_reference_item = &reference_item; // &&Item


    item.compare_number(10);
    reference_item.compare_number(10); //&Item
    other_reference_item.compare_number(10);
}

// error[E0425]: cannot find value `other_number` in this scope 
// syntax 를 깔끔하게? 러스트 마도서 러스트노미콘
