extern crate nanoid;

fn main() {
   print!("Simple: {}\n", nanoid::simple());

   print!("Generate: {}\n", nanoid::generate(10));

   print!("Custom: {}\n", nanoid::custom(10, &['a', 'b', 'c', 'd']));
}
