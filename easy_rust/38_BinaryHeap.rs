// a(n) >= a2n , a2n + 1 어센딩오더? ascending 
// priority  queue
use std::collections::BinaryHeap;

/*
fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> { // This function shows the remainder in the BinaryHeap. Actually an iterator would be
                                                         // faster than a function - we will learn them later.
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number)
    }
    remainder_vec
}

fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30]; // These numbers are in order
    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }
   
   while let Some(number) = my_heap.pop() {
    println!("Popped off {}. Remaining numbers are : {:?}",
        number,
        show_remainder(&my_heap)
    );
   }
 }
 */

// pop 은 빼오기 peek 은 슬쩍훔쳐보기

 fn main(){
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Write back to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    while let Some(job) = jobs.pop() {
        println!("You need to: {}", job.1);
    }
 }