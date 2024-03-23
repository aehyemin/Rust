// pub fn entry(&mut self, key: K) -> Entry<K, V>

use std::collections::HashMap;
/* 
// macro_rules!{}
fn main(){
    let book_collection = vec![
      "L'Allemagne Moderne",
      "Le Petit Prince",
      "섀도우 오브 유머 스마일",
      "Eye of the World",
      "Eye of the World"
      ]; 
  //  .entry() .insert()

    let mut book_hashmap = HashMap::new();
    
    for book in book_collection {
       
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap{
        println!("Do we have {} ? {}", book, true_or_false);
    }
}
*/

fn main(){
    let book_collection = vec![
      "L'Allemagne Moderne",
      "Le Petit Prince",
      "섀도우 오브 유머 스마일",
      "Eye of the World",
      "Eye of the World"
      ]; 
  //  .entry() .insert()

    let mut book_hashmap = HashMap::new();
    
    for book in book_collection {
        let number_of_books = book_hashmap.entry(book).or_insert(0);
        *number_of_books +=1;
    }
    for (book, number) in book_hashmap{
        println!("{} : {}.", book, number);
    }  
}
