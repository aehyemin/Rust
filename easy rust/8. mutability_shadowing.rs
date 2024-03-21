// mut = mutable 

fn main() {
    let my_number = 10;
    my_number = 9;
}

// error : cannot assign twice to immutable variable 'my_number'
//러스트에서는 mut 를 사용하지 않으면 변수에 새로운 값을 할당할 수 없다.

fn main() {
    let mut my_number = 10;
    my_number = 9;
}

//로 바꿔주여야 한다. 




fn main(){
    let my_variable = 10;
    let my_variable = "my variable";
    println!("{}", my_variable);
}


// prints : my variable


fn double(input:i32) -> i32{
   input * 2
}

fn triple(input:i32) -> i32{
   input * 3
}

fn main(){
    let x = 9;
    let x = double(x);
    let x = triple(x);
    println!("{}", x);
}
// 섀도잉을 사용하면 변수명을 복잡하게 여러개 만들 필요가 없다
// 섀도잉은 원래 있던 변수를 없애는 것이 아니라 잠시 막는 것이고 shadowing 한 value 가 사라지면 원래의 값을 다시 확인할 수 있다.


fn main(){ 
    let my_valuable = 9;
    println!("{}", my_variable);
    {
        let my_variable = "some string";
        println!("{}", my_variable);
    }
    println!("{}", my_variable);
}
/*

prints : 
9
some string
9

*/