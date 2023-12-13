fn main() {
    let my_name = "John";
    match my_name {
        "Dipak" => println!("Dipak"),
        "Bob" => println!("not my name"),
        "Alice" => println!("Hello Alice"),
        _ => println!("Nice to meet you!"),
    }
}