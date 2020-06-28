use nanoid::nanoid;

fn main() {
    println!("Simple: {}", nanoid!());
    println!("Generate: {}", nanoid!(10));
    println!("Custom: {}", nanoid!(10, &['a', 'b', 'c', 'd']));
}
