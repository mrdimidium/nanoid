extern crate nanoid;

fn main() {
    println!("Simple: {}", nanoid::simple());

    println!("Generate: {}", nanoid::generate(10));

    println!("Custom: {}", nanoid::custom(10, &['a', 'b', 'c', 'd']));
}
