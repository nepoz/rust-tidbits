mod utils;
use crate::utils::sorting;

fn main() {
   let a = ['a', 'b', 'z', 'c'];
   let mut b: Vec<char> = Vec::new();
   b.extend_from_slice(&a);

   let largest = sorting::find_largest(&a);

   println!("Largest value in a is {}", largest);

   // reassigning a variable
   let largest = sorting::find_largest(&b);

   println!("Largest value in b is {}", largest);

  let biggest = sorting::find_biggest(&b);
  println!("Biggest: {}", biggest)
}
