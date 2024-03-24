// VecDeque - 뷁 
use std::collections::VecDeque;
/* 
fn main() {
    let my_vec = vec![8, 9, 10]; // .pop .push Vec::with_capacity(10) .remove(0)
}

fn main() {
    let mut my_vec = vec![0; 600_000];
    for i in 0..600_000 {
        my_vec.remove(0);
    }
}
*/

// 뷁덱은 앞이랑뒤에서만 보는거. 김밥에서 맨뒤랑 맨앞만 짤라서 먹기
fn main() {
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    for i in 0..600_000 {
        my_vec.pop_front();
    }
}