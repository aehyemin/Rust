//Result - may not work
// Option - maybe there, maybe not

/* 
enum Option<T> {
    None,
    Some<T>,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

//is_some()
//is_none()


//.is_ok()
//.is_err()
fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

// None.unwrap -> panic
// Err.unwrap -> panic


fn main(){
    // if check_error(5).is_ok(){
    //     println!("It;s okay, guys!")
    // } else {
    //     println!("It's an error, guys")
    // }


    match check_error(5) {
        Ok(_) => println!("Okay guys"), // _ 는 아무거나
        Err(_) => println!("It's an error"),
    }
}