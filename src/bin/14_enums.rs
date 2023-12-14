// Data that can be one of multiple different possibilities 
// Each possibilite is called a "Variant"
// More robust program when paired with match key word

// basic structure/expample
enum Direction {
    Up, 
    Down,
    Left,
    Right,
}


fn main() {
    let go = Direction:: Left;
    match go {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}