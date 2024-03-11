// memory
// the stack, the heap, and pointers
// stack(정적메모리 저장) is very fast , but heap(동적메모리 저장) is not so fast
// pointer is called reference in rust. like a table of contents in the book

fn main() {
    let my_number = 15; // This is an i32
    let single_reference = &my_number; //  reference to my_number
    let double_reference = &single_reference; // This is a &&i32
    let five_references = &&&&&my_number; // This is a &&&&&i32
}

// These are all different types